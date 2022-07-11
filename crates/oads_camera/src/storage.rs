use crate::camera::FrameBuff;
use crossbeam::channel::Receiver;
use log::{debug, info, warn};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

static STORAGE_CAPACITY: &str = "/var/system/openads/config/storage/capacity";

/// sets up last saved location
fn set_save_directory(id: &String, save_location: &String) -> String {
    let save_root = vec![String::from("/var/system/openads/storage/"), id.to_string()].join("");
    // Create root directory in case it doesn't exist
    let _ = fs::create_dir_all(save_root.clone()).unwrap();

    let save_to = vec![
        save_root,
        "/".to_string(),
        save_location.to_string(),
        ".avi".to_string(),
    ]
    .join("");
    info!("setting save location for {} to {}", id, save_to);
    save_to
}

/// Manages save locations
#[derive(Clone)]
struct SaveLocation {
    save_counter: usize,
    current_save: String,
    save_root: String,
}

impl SaveLocation {
    /// Create an instance of *self based on data stored in registry.toml
    pub fn new(id: &String) -> Self {
        Self {
            save_counter: 0,
            current_save: "".to_string(),
            save_root: "".to_string(),
        }
    }

    /// creates a save target using the camera id and the last saved location
    fn update_last_saved(last_saved: String) -> String {
        let local: String = chrono::Local::now()
            .date()
            .to_string()
            .replace('-', "")
            .replace(':', "");
        if local == last_saved {
            return last_saved;
        }
        local
    }
}

type ImageBuffer = Vec<Vec<u8>>;
#[derive(Clone)]
pub struct Storage {
    save_location: SaveLocation,
    device_name: String,
    img_buffer: ImageBuffer,
    reset_after: usize,
    // Controls activity status of this service - Online/ Offline
}

impl Storage {
    /// create a new instance of this structure
    pub fn new(
        camera_id: String,
        device_name: String,
        vendor_id: String,
        product_id: String,
    ) -> Self {
        let save_location = SaveLocation::new(&camera_id);
        let img_buffer = ImageBuffer::new();
        let storage_capacity = Self::required_storage_size();
        Self {
            save_location,
            device_name,
            img_buffer,
            reset_after: 24,
        }
    }

    /// activates the internal async writer
    /// * rx: channel from which it will be receiving data from
    pub fn enable_storage_pool(&mut self, rx: Receiver<FrameBuff>) {
        loop {
            let frame = match rx.recv() {
                Ok(obj) => obj.buffer(),
                Err(_) => vec![],
            };

            let new_file = File::create(&Path::new("Test.png")).unwrap();
            let mut writer = BufWriter::new(new_file);
            writer.write(&frame[..]).unwrap();
        }
    }

    const fn required_storage_size() -> usize {
        256 << 30
    }
}
