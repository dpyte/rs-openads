use std::{fs, io, u16};
use std::collections::HashMap;

use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use opencv::core::Mat;
use opencv::prelude::VideoCaptureTrait;
use opencv::videoio;
use opencv::videoio::VideoCapture;

use crate::info::CameraInfo;
use crate::read_lines;
use crate::scan::IdInformation;

const SYS_DEV_PATH: &str = "/sys/class/video4linux";

pub struct Vision {
    camera: Box<CameraInfo>,
    is_active: bool,
    match_device_info: HashMap<(String, String), String>,
    // OpenCV
    v_frame: Mat,
    // Torch Model
}

impl Vision {
    pub fn new(camera: Box<CameraInfo>) -> Vision {
        let copy = camera.clone();
        Vision {
            camera: copy,
            is_active: true,
            match_device_info: HashMap::new(),
            v_frame:Default::default()
        }
    }

    pub fn execute(&mut self)  {
        self.get_device_serial_port();
        let mut skip_counter =  0; // Return from the function once this flag count reaches 15
        let mut device_is = String::from("/dev/");

        let vend_id = self.camera.g_vendor_id();
        let prod_id = self.camera.g_product_id();
        if self.match_device_info.contains_key(&(vend_id.to_string(), prod_id.to_string())) {
            let path_is = self.match_device_info.get(&(vend_id.to_string(), prod_id.to_string()));
            let fetch_last: Vec<&str> = path_is.unwrap().split("/").collect();
            device_is.push_str(fetch_last[fetch_last.len() - 1]);
        }

        let mut capture_device = VideoCapture::from_file(&device_is, videoio::CAP_ANY).expect("failed to open device");
        while self.is_active {
            if skip_counter == 15 {
                return;
            }
            capture_device.read(&mut self.v_frame).expect("TODO: panic message");
        }
    }

    fn get_device_serial_port(&mut self) {
        // TODO: Fix and exit gracefully
        let file_listings = fs::read_dir(SYS_DEV_PATH).unwrap();
        for path in file_listings {
            let path_is = &mut path.unwrap().path().display().to_string();

            let mut file = path_is.clone();
            file.push_str("/device/uevent");

            let contents = read_lines(file).expect("Unable to open file");
            for line in contents {
                if let Ok(line) = line {
                    if line.starts_with("PRODUCT") {
                        // Guaranteed to have '=' in the string ...
                        let first_split: Vec<&str> = line.split("=").collect();
                        let collection: Vec<&str> = first_split[1].split("/").collect();

                        let vendor  = u16::from_str_radix(collection[0], 16).expect("failed to parse vendor id").to_string();
                        let product = u16::from_str_radix(collection[1], 16).expect("failed to parse product id").to_string();

                        self.match_device_info.insert(
                            (vendor, product),
                            path_is.to_string()
                        );
                    }
                }
            }
        }
    }

}
