use crate::read_lines;
use crate::containers::Container;

fn update_storage_capacity() -> u64 {
    let contents = read_lines("/var/system/openads/config/storage/capacity");
    for line in contents.expect("unable to open file") {
        if let Ok(line) = line {
            if !line.starts_with("#") {
                let raw_literal: Vec<&str> = line.split("=").collect();
                return raw_literal[1].to_string().parse::<u64>().unwrap_or(4 << 30);
            }
        }
    }
    4 << 30
}

pub struct Storage {
    container: Container,
    save_location: String,
    storage_capacity: u64,
    main_is_active: bool,
}

impl Storage {
    pub fn activate_storage_pool(&mut self) {
        if self.main_is_active {
            return
        }
        self.main_is_active = true;
    }
}
