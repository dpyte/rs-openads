use log::info;
use std::thread;
use opencv::core::Mat;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc::Receiver;
use oads_models::elephant::model::Elephant;

use crate::camera::Camera;
use crate::info::CameraInfo;
use crate::scan::IdInformation;

pub struct Vision {
	device_name:    String,
	elephant:         Elephant,
	// Torch Model
}

impl Vision {
	/// initializes *Self and executes the opencv-based camera backend
	/// * update[:?] -> storage backend is now build into the function.
	pub async fn new(camera: CameraInfo) -> Self {
		let device_name = camera.g_name();
		let (camera, recv) = Camera::new(camera);

		info!("starting an executor service for {}", device_name);

		Self::start_video_processor(camera, recv);

		let elephant = Elephant::new();
		Self { device_name, elephant }
	}

	async fn start_video_processor(mut camera: Camera, mut recv: Receiver<Mat>) {
		thread::spawn(move || { camera.start_streamer() });
		info!("preparing vision for image processing ...");
		loop {
			let item = recv.recv().await.unwrap();
			println!("{:?}", item);
		}
	}
}
