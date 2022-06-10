use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use oads_camera::vision::Vision;
use oads_camera::info::CameraInfo;
use oads_camera::scan::IdInformation;
use oads_storage::containers::Container;
use oads_storage::storage::Storage;

fn execute_main_loop(infos: Vec<CameraInfo>) -> Vec<Container> {
    let mut camera_runtimes: Vec<Runtime> = Vec::new();
    let mut containers = Vec::new();

    for x in infos {
        containers.push(Container::new(
            x.g_name().to_string(),
            x.g_id()
        ));

        let mut rt = Runtime::new().expect("Failed to initiate runtime");
        rt.block_on(async move {
            tokio::spawn(async move {
                let cam_info = Box::new(CameraInfo::from(&x));
                let mut v = Vision::new(cam_info);
                v.execute();
            });
        });
        camera_runtimes.push(rt);
    }
    containers
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

    let mut storage_runtime = Runtime::new()
        .expect("Failed to initiate runtime for storage container");
    let storage_containers = execute_main_loop(scan_devices.validated_cameras().clone());
    storage_runtime.block_on(async move {
        tokio::spawn(async move {
            let mut storage_pool = Storage::new(storage_containers);
            storage_pool.activate_storage_pool();
        });
    });
}
