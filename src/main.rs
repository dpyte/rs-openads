use log::{error, info};
use tokio::runtime::Runtime;

use oads_log::LOG_FILE;
use oads_camera::vision::Vision;
use oads_camera::data::info::CameraInfo;

async fn launch_camera_services(validated_cameras: CameraInfo) {
	info!("preparing data for opencv-pipeline");
	let mut camera = Vision::new(validated_cameras, true);
	camera.init();
}

fn main() {
	log4rs::init_file(LOG_FILE, Default::default()).unwrap();
	let mut scan_devices = oads_camera::data::  read::Read::new();
	scan_devices.validate_and_match();

	let validated_cameras = scan_devices.validated_camera();
	scan_devices.save_updated_ids();

	let device_count = scan_devices.device_count();
	let to_continue = if device_count == 0 { error!(target: "syslog", "failed to detect any valid device"); false }
		else { info!("detected {:?}", validated_cameras.g_id()); true };

	if to_continue {
		let mut c_service_rt = Runtime::new().unwrap();
		let cam_service = launch_camera_services(validated_cameras);
		c_service_rt.block_on(cam_service);
	}
}
