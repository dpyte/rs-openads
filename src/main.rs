use oads_log::LOG_FILE;
use log::{info, error};
use tokio::runtime::Runtime;
use oads_camera::vision::Vision;
use oads_camera::info::CameraInfo;

fn execute_main_loop(infos: Vec<CameraInfo>) {
    let mut camera_runtimes: Vec<Runtime> = Vec::new();

    info!("preparing data for opencv-pipeline");
    for x in infos {
        let mut rt = Runtime::new().expect("Failed to initiate runtime");
        rt.block_on(async move {
            tokio::spawn(async move {
                let mut v = Vision::new(x);
                v.execute();
            });
        });
        camera_runtimes.push(rt);
    }
}


fn main() {
    log4rs::init_file(LOG_FILE, Default::default()).unwrap();
    let mut scan_devices = oads_camera::read::Read::new();
    scan_devices.validate_and_match();

    let device_count = scan_devices.device_count();
    if device_count == 0 {
        error!(target: "syslog", "failed to detect any valid device");
    } else {
        info!("detected {:?} devices\n", device_count);
    }
    scan_devices.save_updated_ids();
    let _storage_containers = execute_main_loop(scan_devices.validated_cameras().clone());
}
