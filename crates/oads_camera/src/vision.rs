use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use crate::info::CameraInfo;
use crate::scan::IdInformation;

const V_HEIGHT: u16 = 240;
const V_WIDTH: u16 = 320;
const SYS_DEV_PATH: &str = "/sys/class/video4linux";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_device_serial_port(vid: String, pid: String) {
    static mut run_once: bool = true;

    unsafe {
        if run_once {
            run_once = false;
        } else {
            return;
        }
    }

    let mut paths: Vec<String> = Vec::new();
    // TODO: Fix and exit gracefully
    let file_listings = fs::read_dir(SYS_DEV_PATH).unwrap();
    for path in file_listings {
        let mut file = path.unwrap().path().display().to_string();
        file.push_str("/device/uevent");

        let contents = read_lines(file).expect("Unable to open file");
        for line in contents {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }
}

pub struct Vision {
    camera: CameraInfo,
    is_active: bool,
    // OpenCV Frame
    // Torch Model
}

impl Vision {
    pub fn new(camera: CameraInfo) -> Vision {
        Vision {
            camera,
            is_active: true
        }
    }

    pub fn execute(&self) -> bool {
        let vend_id = self.camera.g_vendor_id();
        let prod_id = self.camera.g_product_id();

        let serial_port_is = get_device_serial_port(vend_id, prod_id);
        true
    }
}
