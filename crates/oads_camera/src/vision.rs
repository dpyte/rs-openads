use chrono::Local;
use std::{fs, u16};
use v4l::buffer::Type;
use v4l::video::Capture;
use jpeg_decoder::Decoder;
use v4l::io::mmap::Stream;
use log::{debug, info, warn};
use filesize::file_real_size;
use v4l::prelude::MmapStream;
use std::collections::HashMap;
use v4l::{Device, Format, FourCC};
use v4l::io::traits::CaptureStream;

use crate::read_lines;
use crate::info::CameraInfo;
use crate::scan::IdInformation;

static SYS_DEV_PATH: &str = "/sys/class/video4linux";
static STORAGE_CAPACITY: &str = "/var/system/openads/config/storage/capacity";

/// configures appropriate device id and stores it into a hashmap with
/// vendor+product id being the keys
fn get_device_serial_port() -> HashMap<(String, String), String> {
    let mut mapped_devices: HashMap<(String, String), String> = HashMap::new();
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

                    mapped_devices.insert((vendor, product), path_is.to_string());
                }
            }
        }
    }
    mapped_devices
} // get_device_serial_port

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
    let save_root = vec![String::from("/var/system/openads/storage/"), id.to_string()].join("");
    let save_to = vec![save_root, "/".to_string(), save_location.to_string(), ".avi".to_string()].join("");
    println!("setting save location for {} to {}", id, save_to);
    save_to
}

fn update_last_saved(last_saved: String) -> String {
    let local: String = Local::now().date().to_string().replace('-', "").replace(':', "");
    if local == last_saved { return last_saved }
    local
}

fn check_storage_capacity(to_check: &String, sky: &u64) -> bool {
    let file = file_real_size(to_check).unwrap_or(1 << 30);
    *sky < file
}

pub struct Vision {
    camera:             CameraInfo,
    last_saved:         String,
    device_is:          String,
    save_to:            String,
    storage_capacity:   u64,
    match_device_info:  HashMap<(String, String), String>,
    // Torch Model
}

impl Vision {
    pub fn new(camera: CameraInfo) -> Self {
        let camera = camera.clone();

        let storage_capacity = update_storage_capacity();
        let match_device_info = get_device_serial_port();

        let last_saved = update_last_saved(String::new());
        let save_to = set_save_directory(&camera.g_id(), &last_saved.to_string());

        let mut device_is = String::from("/dev/");

        let vend_id = camera.g_vendor_id();
        let prod_id = camera.g_product_id();
        if match_device_info.contains_key(&(vend_id.to_string(), prod_id.to_string())) {
            let path_is = match_device_info.get(&(vend_id.to_string(), prod_id.to_string()));
            let fetch_last: Vec<&str> = path_is.unwrap().split("/").collect();
            device_is.push_str(fetch_last[fetch_last.len() - 1]);
        }

        warn!(" {} => setting save location to {}", camera.g_name(), save_to);
        Self {
            camera,
            last_saved,
            device_is,
            save_to,
            storage_capacity,
            match_device_info,
        }
    }

    /// initializes and executes the opencv-based camera backend
    /// * update[:?] -> storage backend is now build into the function.
    /// * update[Jun 11, 2022] -> system uses libv4l as video processing backend
    pub fn execute(&mut self) {
        let use_defined_capture_device = true;
        let (stream_device, format) = self.setup_stream_device(320, 240, use_defined_capture_device);
        self.setup_stream_buffer(stream_device, use_defined_capture_device, format);
    }

    /// Prepare device to stream data via webcam
    /// TODO: Current model maps device buffer to stream data directly from the webcam.
    ///       Additional features to enable device to stream data from:
    ///         - Network/ IP Camera
    /// Returns _capture-device_
    fn setup_stream_device(&self, width: u32, height: u32, mut use_defined_device: bool) -> (Device, v4l::Format) {
        // let device_index =self.device_is.chars().nth(self.device_is.len() - 1).unwrap();
        // let capture_device = Device::new(device_index.to_digit(10).unwrap() as usize).expect("Failed to open specified device");
        let capture_device = match Device::with_path(self.device_is.to_string()) {
            Ok(capture_device) => {
                use_defined_device = true;
                capture_device
            },
            Err(_) => {
                warn!("failed to initialize capture device for {:?} ... resolving to default _capture_device_", self.camera.g_name());
                use_defined_device = false;
                Device::new(0).expect("Failed to setup default capture buffer")
            }
        };

        // Set default frame to 320x240 ...
        let mut fmt = capture_device.format().expect("Failed to read format");
        fmt.width = width;
        fmt.height = height;
        fmt.fourcc = FourCC::new(b"AVI1");

        if use_defined_device {
            capture_device.set_format(&fmt).expect("Failed to write format");
            info!("using config-defined format: {:?}\n", fmt);
        }
        (capture_device, fmt)
    }

    /// enables capture device to process incoming data.
    /// *this function **should be launched as an async
    fn setup_stream_buffer(&mut self, mut capture_device: Device, use_defined_device: bool, format: Format) {
        let mut stream = if use_defined_device {
            info!("Initializing user defined streamer");
            MmapStream::with_buffers(&capture_device, Type::VideoCapture, 4).unwrap()
        } else {
            info!("Initializing fallback streamer");
            Stream::with_buffers(&mut capture_device, Type::VideoCapture, 4).expect("Failed to create buffer stream")
        };

        loop {
            let (buffer, _) = stream.next().unwrap();

            // user-defined streamer will generate an avi1 output
            //                          /-- Writer
            // TODO: send this data to -
            //                          \-- Model
            //
            let _data = match &format.fourcc.repr {
                b"RGB3" => buffer.to_vec(),
                b"MJPG" => {
                    let mut decoder = Decoder::new(buffer);
                    decoder.decode().expect("failed to decode JPEG")
                },
                b"AVI1" => buffer.to_vec(),
                _ => Vec::new()
            };
        }
    }
}
