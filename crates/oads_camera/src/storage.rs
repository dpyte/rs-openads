use std::{fs, thread};
use log::{debug, info, warn};
use filesize::file_real_size;
use std::collections::HashMap;
use opencv::core::MatTraitConst;
use opencv::videoio::VideoWriter;
use crossbeam::channel::{Sender, Receiver};
use opencv::prelude::{Mat, VideoWriterTrait};

static SYS_DEV_PATH: &str = "/sys/class/video4linux";
static STORAGE_CAPACITY: &str = "/var/system/openads/config/storage/capacity";


fn check_storage_capacity(to_check: &String, sky: &u64) -> bool {
	let file = file_real_size(to_check).unwrap_or(1 << 30);
	*sky < file
}

fn deserialize_ids(line: String) -> (String, String) {
	// Guaranteed to have '=' in the string ...
	let first_split: Vec<&str> = line.split("=").collect();
	let collection: Vec<&str> = first_split[1].split("/").collect();

	let vendor = u16::from_str_radix(collection[0], 16).expect("failed to parse vendor id").to_string();
	let product = u16::from_str_radix(collection[1], 16).expect("failed to parse product id").to_string();

	(vendor, product)
}

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

		let contents = crate::read_lines(file).expect("Unable to open file");
		for line in contents {
			if let Ok(line) = line {
				if line.starts_with("PRODUCT") {
					let (vendor, product) = deserialize_ids(line);
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
	let contents = crate::read_lines(STORAGE_CAPACITY);
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
fn update_last_saved(last_saved: String) -> String {
	let local: String = chrono::Local::now().date().to_string().replace('-', "").replace(':', "");
	if local == last_saved { return last_saved; }
	local
}

/// sets up last saved location
fn set_save_directory(id: &String, save_location: &String) -> String {
	let save_root = vec![String::from("/var/system/openads/storage/"), id.to_string()].join("");

	let _ = fs::create_dir_all(save_root.clone()).unwrap();
	let save_to = vec![save_root, "/".to_string(), save_location.to_string(), ".avi".to_string()].join("");
	info!("setting save location for {} to {}", id, save_to);
	save_to
}

pub struct Channel<T> {
	tx: Sender<T>,
	rx: Receiver<T>,
}

#[derive(Clone)]
pub struct Storage {
	save_to:        String,
	device_is:      String,
	last_saved:     String,
	device_name:    String,
	storage_capacity: u64,

	// Controls activity status of this service - Online/ Offline
}

unsafe impl Send for Storage {}

impl Storage {
	/// create a new instance of this structure
	pub fn new(camera_id: String, device_name: String, vendor_id: String, product_id: String) -> Self {
		let storage_capacity = update_storage_capacity();

		let last_saved = update_last_saved(String::new());
		let save_to = set_save_directory(&camera_id, &last_saved.to_string());
		let mut device_is = String::from("/dev/");

		let match_device_info = get_device_serial_port();
		if match_device_info.contains_key(&(vendor_id.clone(), product_id.clone())) {
			let path_is = match_device_info.get(&(vendor_id, product_id));
			let fetch_last: Vec<&str> = path_is.unwrap().split("/").collect();
			device_is.push_str(fetch_last[fetch_last.len() - 1]);
		}
		warn!("{} => setting save location to {}", device_name, save_to);
		Self { save_to, device_is, last_saved, device_name, storage_capacity }
	}

	/// activates the internal async writer
	pub fn enable_storage_pool(&mut self, mut rx: Receiver<Mat>) {
		let save_size = opencv::core::Size::new(320, 240);
		let fourcc = VideoWriter::fourcc('A' as i8, 'V' as i8, 'I' as i8, '1' as i8).unwrap();
		let mut video_writer = VideoWriter::new(self.save_to.as_str(), fourcc, 30 as f64, save_size, true).unwrap();

		loop {
			let frame = rx.recv().unwrap();
			if !frame.empty() { let _ = video_writer.write(&frame); }
		}
	}

	pub fn g_capture_device(&self) -> String { self.device_is.clone() }
}
