

fn main() {
    let scan_devices = oads_camera::read::Read::new();
    scan_devices.validate_devices();
}
