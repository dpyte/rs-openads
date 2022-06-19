use std::thread;
use opencv::core::Mat;
use log::{debug, info};
use opencv::prelude::*;
use std::time::Duration;
use opencv::{imgproc, videoio};
use opencv::videoio::VideoCapture;
use oads_models::models::ModelBank;
use opencv::objdetect::CascadeClassifier;
use opencv::dnn::enable_model_diagnostics;
use crossbeam::channel::{Receiver, Sender};
use opencv::highgui::{imshow, named_window};

use crate::data::info::CameraInfo;
use crate::data::scan::IdInformation;
use crate::storage::{Channel, Storage};
use crate::process::{execution, processing};
use crate::process::execution::RuntimeMode;


/// Path to haar cascade file extracted from OpenCV
static HCFFD: &str = "/var/system/openads/config/models/facial/haarcascade_frontalfacial_default.xml";

/// Shared references
struct Handlers {
	storage: Channel<bool>
}

pub struct Vision {
	// Torch Model
	rx:             Receiver<Mat>,
	rt_mode:        RuntimeMode,
	camera_info:    CameraInfo,
	models:         ModelBank,
	is_active:      bool,
}

impl Vision {
	/// initializes *Self and executes the opencv-based camera backend
	/// * update[:?] -> storage backend is now build into the function.
	pub fn new(camera_info: CameraInfo, enable_storage: bool) -> Self {
		let rt_mode = execution::execution_mode();
		info!("starting process in {:?}", rt_mode);

		let models = ModelBank::new();

		// setup channel to relay `frames`
		let (mut tx, rx) = crossbeam::channel::bounded(8);

		let device_id = camera_info.g_id();
		// Start sender
		thread::spawn(move || { Self::start_capture_server(device_id, tx); });

		// Start storage
		if enable_storage {
			let receiver = rx.clone();
			Self::activate_storage(&camera_info, receiver);
		}

		Self { models, rt_mode, camera_info, rx, is_active: true }
	}

	/// Start storage
	fn activate_storage(camera_info: &CameraInfo, receiver: Receiver<Mat>) {
		// Communication to the storage pool will be done via mpsc and hence, there is no need to
		// store any active reference of that class
		// TODO: Implement channels for communication b/w two processes
		let mut storage = Storage::new(camera_info.g_id(), camera_info.g_name(),
		                           camera_info.g_vendor_id(), camera_info.g_product_id());
		thread::spawn(move || { storage.enable_storage_pool(receiver); });
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

		let mut cascade: CascadeClassifier;
		// let (mut cascade_classifier, to_enable_cascade) = Self::enable_facial_detection();
		// if to_enable_cascade { cascade = cascade_classifier.unwrap(); }

		// All the frames will be in RGB format and it is necessary to reformat them to gray
		while self.is_active {
			let frame = self.rx.recv().unwrap();
			if frame.size().unwrap().width == 0 {
				thread::sleep(Duration::from_millis(120));
				continue;
			}

			let faces = crate::process::processing::detect_facial_presence(&frame);
		}
		info!("Stopping camera services");
	}

	/*fn enable_facial_detection() -> (Result<CascadeClassifier, E>, bool) {
		let cc = opencv::objdetect::CascadeClassifier::new(HCFFD);
		let to_enable = match cc { Ok(_)  => true, Err(_) => false, };
		(cc, to_enable)
	}*/

}
