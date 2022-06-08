
use std::thread;
use tokio::runtime::Runtime;
use oads_camera::info::CameraInfo;
use oads_camera::vision::Vision;

fn execute_main_loop(infos: Vec<CameraInfo>) {
    let mut camera_runtimes: Vec<Runtime> = Vec::new();
    for x in infos {
        let mut rt = Runtime::new().expect("Failed to initiate runtime");
        rt.block_on(
            async move {
                tokio::spawn(async move {
                    let cam_info = Box::new(CameraInfo::from(&x));
                    let mut v = Vision::new(cam_info);
                    v.execute();
                });
            }
        );
        camera_runtimes.push(rt);
    }
}

fn main() {
    let mut scan_devices = oads_camera::read::Read::new();
    scan_devices.validate_and_match();

    let device_count = scan_devices.device_count();
    if device_count == 0 {
        println!("failed to detect any valid device");
    } else {
        println!("detected {:?} devices\n", device_count);
    }
    scan_devices.save_updated_ids();
    execute_main_loop(scan_devices.validated_cameras().clone());
}
