use std::thread;

use log::{debug, info};
use opencv::prelude::*;
use opencv::videoio;
use opencv::videoio::VideoCapture;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::info::CameraInfo;
use crate::scan::IdInformation;
use crate::storage::Storage;

/// To store video data
// Camera data was first processed by 'vision' however, the functionality of vision has been reassigned to
// strictly process frame data
pub struct Camera {
	camera_info: CameraInfo,
	vision_sender: Sender<Mat>,
}

impl Camera {
	pub fn new(camera_info: CameraInfo) -> (Self, Receiver<Mat>) {
		let (vision_sender, receiver) = mpsc::channel(8);
		(Self { camera_info, vision_sender }, receiver)
	}

	/// calls the opencv backend and capture the frame data in self.frame
	pub fn start_streamer(&mut self) {
		// calls user defined streamer and activates the capture buffer provide via opencv
		// - conditionally enables storage server as long as no issues are encountered in step above

		let (mut storage_sender, storage_receiver) = mpsc::channel(8);
		let storage = Storage::new(self.camera_info.g_id().to_string(), self.camera_info.g_name(),
		                           self.camera_info.g_vendor_id(), self.camera_info.g_product_id(), storage_receiver);
		let device_is = storage.g_capture_device();

		debug!("starting streamer for {:?}", self.camera_info.g_name());
		let mut enable_storage = true;
		let mut video_capture = match VideoCapture::from_file(device_is.as_str(), videoio::CAP_ANY) {
			Ok(vc) => vc,
			Err(_) => {
				enable_storage = false;
				VideoCapture::default().unwrap()
			}
		};

		if enable_storage {
			thread::spawn(move || { storage.enable_storage_pool(); });
		}

		loop {
			let mut frame = Mat::default();
			let _ = video_capture.read(&mut frame).unwrap();

			let _ = self.vision_sender.send(frame.clone());
			let _ = storage_sender.send(frame);
		}
	}
}
