use chrono::Local;
use std::{fs, u16};
use opencv::videoio;
use log::{error, info, warn};
use opencv::core::Mat;
use opencv::prelude::*;
use filesize::file_real_size;
use std::collections::HashMap;
use crossbeam::channel::{Receiver, Sender};
use opencv::videoio::{VideoCapture, VideoCaptureTrait, VideoWriter};

use crate::read_lines;
use crate::info::CameraInfo;
use crate::scan::IdInformation;

static SYS_DEV_PATH: &str = "/sys/class/video4linux";
static STORAGE_CAPACITY: &str = "/var/system/openads/config/storage/capacity";

/// read and configure storage capacity for defined camera
/// defaults to 4GB in case it encounters some errors when reading from the file
fn update_storage_capacity() -> u64 {
    let contents = read_lines(STORAGE_CAPACITY);
    for line in contents.expect("unable to open file") {
        if let Ok(line) = line {
            if !line.starts_with("#") {
                let raw_literal: Vec<&str> = line.split("=").collect();
                return raw_literal[1].to_string().parse::<u64>().unwrap_or(4 << 30);
            }
        }
    }
    (4 << 30) as u64
}

/// creates a save target using the camera id and the last saved location
fn set_save_directory(id: &String, save_location: &String) -> String {
    let save_root = vec![
        String::from("/var/system/openads/storage/"),
        "".to_string(),
        id.to_string(),
    ].join("");

    let save_to = vec![save_root, "/".to_string(), save_location.to_string(), ".avi".to_string()]
        .join("");

    println!("setting save location for {} to {}", id, save_to);
    save_to
}

fn update_last_saved(last_saved: String) -> String {
    let local: String = Local::now().date().to_string()
        .replace('-', "")
        .replace(':', "");

    if local == last_saved {
        return last_saved
    }
    local
}

fn check_storage_capacity(to_check: &String, sky: &u64) -> bool {
    let file = file_real_size(to_check).unwrap_or(1 << 30);
    *sky < file
}

pub struct Vision {
    // OpenCV
    camera:             CameraInfo,
    is_active:          bool,
    last_saved:         String,
    match_device_info:  HashMap<(String, String), String>,
    save_to:            String,
    storage_capacity:   u64,
    send:               Sender<Mat>,
    recv:               Receiver<Mat>,
    // Torch Model
}

impl Vision {
    pub fn new(camera: CameraInfo) -> Self {
        let copy = camera.clone();

        let storage_capacity = update_storage_capacity();

        let last_saved = update_last_saved(String::new());
        let save_to = set_save_directory(&camera.g_id(), &last_saved.to_string());
        let (send, recv) = crossbeam::channel::unbounded();

        info!(" {} => setting save location to {}", camera.g_name(), save_to);
        Self {
            camera: copy,
            is_active: true,
            last_saved,
            match_device_info: HashMap::new(),
            save_to,
            storage_capacity,
            send,
            recv,
        }
    }

    /// initializes and executes the opencv-based camera backend
    /// * update -> storage backend is now build into the function.
    pub fn execute(&mut self) {
        self.get_device_serial_port();

        let mut skip_counter = 0; // Return from the function once this flag count reaches 15
        let mut device_is = String::from("/dev/");

        let vend_id = self.camera.g_vendor_id();
        let prod_id = self.camera.g_product_id();
        if self.match_device_info.contains_key(&(vend_id.to_string(), prod_id.to_string())) {
            let path_is = self.match_device_info.get(&(vend_id.to_string(), prod_id.to_string()));
            let fetch_last: Vec<&str> = path_is.unwrap().split("/").collect();
            device_is.push_str(fetch_last[fetch_last.len() - 1]);
        }

        let mut capture_device = match VideoCapture::from_file(&device_is, videoio::CAP_ANY) {
            Ok(cd) => cd,
            Err(_) => {
                error!("failed to initialize capture device for {}", self.camera.g_name());
                VideoCapture::default().unwrap()
            }
        };

        crossbeam::scope(|s| {
            s.spawn(|_| {
                while self.is_active {
                    if skip_counter == 15 { return; }
                    let mut frame_be = Mat::default();
                    let capture_status = capture_device.read(&mut frame_be);
                    // reset skip flag if _capture_status_ => success
                    skip_counter = if capture_status.is_ok() {
                        let send_status = self.send.send(frame_be).is_ok();
                        if !send_status { /* TODO: log this failure */ }
                        0
                        // insert_to_frame(frame_be);
                    } else {
                        skip_counter += 1;
                        skip_counter
                    };
                    self.is_active = if skip_counter == 0 { true } else { false }
                }
            });
        }).expect("TODO: panic message");

        // activate storage pool for the associated cam device.
        self.activate_storage();
    }

    /// configures appropriate device id and stores it into a hashmap with
    /// vendor+product id being the keys
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

                        let vendor = u16::from_str_radix(collection[0], 16).expect("failed to parse vendor id").to_string();
                        let product = u16::from_str_radix(collection[1], 16).expect("failed to parse product id").to_string();

                        self.match_device_info.insert((vendor, product), path_is.to_string());
                    }
                }
            }
        }
    }

    /// start the storage writer in an async thread
    fn activate_storage(&mut self) {
        let device_name = &self.camera.g_name();
        while self.is_active {
            let mut to_continue = check_storage_capacity(&self.save_to, &self.storage_capacity);
            let obj= match self.recv.recv() {
                Ok(frame) => frame,
                Err(_) => { to_continue = false; Mat::default() }
            };
            if to_continue {
                let frame_size = opencv::core::Size::new(320, 240);
                let fourcc = VideoWriter::fourcc('A' as i8, 'V' as i8, 'I' as i8, '1' as i8).unwrap();

                let video_writer = VideoWriter::new(self.save_to.clone().as_str(), fourcc, 30 as f64, frame_size, true);
                match video_writer {
                    Ok(_) => { println!("Initialized video writer for {:?}", device_name); },
                    Err(_) => {
                        println!("Failed to initialize video writer for device {:?}", device_name);
                        return
                    }
                };
                let _ = video_writer.unwrap().write(&obj);
            }
        }
    }
}
