

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

}
