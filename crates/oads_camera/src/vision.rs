use std::thread;
use log::{debug, info};
use std::time::Duration;
use v4l::io::traits::OutputStream;
use crossbeam::channel::{Receiver, Sender};

use crate::storage::Storage;
use crate::process::{execution};
use crate::data::info::CameraInfo;
use crate::data::scan::IdInformation;
use crate::camera::{FrameBuff, Streamer};
use crate::process::execution::RuntimeMode;


pub struct Vision {
	// Torch Model
	rx:             Receiver<FrameBuff>,
	rt_mode:        RuntimeMode,
	camera_info:    CameraInfo,
	// models:         ModelBank,
	is_active:      bool,
}

impl Vision {
	/// initializes *Self and executes the opencv-based camera backend
	/// * update[:?] -> storage backend is now build into the function.
	pub fn new(camera_info: CameraInfo, enable_storage: bool) -> Self {
		let rt_mode = execution::execution_mode();
		info!("starting process in {:?}", rt_mode);

		// setup channel to relay `frames`
		let (tx, rx) = crossbeam::channel::bounded(8);

		let device_id = camera_info.g_id();
		thread::spawn(move || { Self::start_capture_server(device_id, tx); });

		// Start storage
		if enable_storage {
			let receiver = rx.clone();
			Self::activate_storage(&camera_info, receiver);
		}

		// let models = ModelBank::new();
		Self { rt_mode, camera_info, rx, is_active: true }
	}

	/// Start storage
	fn activate_storage(camera_info: &CameraInfo, receiver: Receiver<FrameBuff>) {
		// Communication to the storage pool will be done via mpsc and hence, there is no need to
		// store any active reference of that class
		// TODO: Implement channels for communication b/w two processes
		let mut storage = Storage::new(camera_info.g_id(), camera_info.g_name(),
		                           camera_info.g_vendor_id(), camera_info.g_product_id());
		thread::spawn(move || { storage.enable_storage_pool(receiver); });
	}

	/// Capture current frame and forward it to the channel
	fn start_capture_server(device_id: String, tx: Sender<FrameBuff>) {
		info!("starting streamer service for {:?}", device_id);

		let mut streamer = Streamer::new().unwrap();
		loop {
			let (buf, meta) = streamer.stream.next().unwrap();
			let buffer = FrameBuff::new(buf);
			tx.send(buffer).unwrap();
		}
	}

	/// Starts the storage backend as well as the receiver
	pub fn init(&mut self) {
		info!("Starting vision services ...");

		thread::sleep(Duration::from_millis(500));
		while self.is_active {
			let v_frame = match self.rx.recv() {
				Ok(obj) => obj.buffer(),
				Err(_) => Vec::new()
			};
		}
		info!("Stopping camera services");
	}

	/*fn enable_facial_detection() -> (Result<CascadeClassifier, E>, bool) {
		let cc = opencv::objdetect::CascadeClassifier::new(HCFFD);
		let to_enable = match cc { Ok(_)  => true, Err(_) => false, };
		(cc, to_enable)
	}*/
}
