use log::{debug, info};
use std::thread;
use opencv::videoio;
use opencv::core::Mat;
use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use oads_models::models::ModelBank;
use crossbeam::channel::{Receiver, Sender};
use opencv::highgui::{imshow, named_window};

use crate::info::CameraInfo;
use crate::storage::Storage;
use crate::scan::IdInformation;

pub struct Vision {
	// Torch Model
	storage:        Storage,
	models:         ModelBank,
	camera_info:    CameraInfo,
	rx:             Receiver<Mat>,
}

impl Vision {
	/// initializes *Self and executes the opencv-based camera backend
	/// * update[:?] -> storage backend is now build into the function.
	pub fn new(camera_info: CameraInfo, enable_storage: bool) -> Self {
		let models = ModelBank::new();
		let mut storage = Storage::new(camera_info.g_id(), camera_info.g_name(), camera_info.g_vendor_id(), camera_info.g_product_id());
		let camera_storage = storage.clone();

		// setup channel to relay frames
		let (mut tx, rx) = crossbeam::channel::bounded(8);

		let device_id = camera_info.g_id();
		let receiver = rx.clone();

		// Start sender
		thread::spawn(move || { Self::start_capture_server(device_id, tx); });
		// Start storage
		thread::spawn(move || { if enable_storage { storage.enable_storage_pool(receiver); } });

		Self { storage: camera_storage, models, camera_info, rx }
	}

	/// Capture current frame and forward it to the channel
	fn start_capture_server(device_id: String, mut tx: Sender<Mat>) {
		info!("starting streamer service for {:?}", device_id);
		// calls user defined streamer and activates the capture buffer provide via opencv
		// - conditionally enables storage server as long as no issues are encountered in step above
		// !TODO: Gracefully exit the function in case of failure
		let mut video_capture = match VideoCapture::new(0, videoio::CAP_ANY) {
			Ok(vc) => vc,
			Err(_) => VideoCapture::default().expect("Failed to load video capture @ try-2")
		};
		loop {
			let mut frame = Mat::default();
			let _ = video_capture.read(&mut frame);
			let _ = tx.send(frame);
		}
	}

	/// Starts the storage backend as well as the receiver
	pub fn init(&mut self) {
		info!("Starting vision services ...");
		loop {
			let frame = self.rx.recv().unwrap();
			if !frame.empty() {

			}
		}
	}
}
