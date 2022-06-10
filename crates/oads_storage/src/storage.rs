use opencv::prelude::{VideoWriterTrait};

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
    container:          Vec<Container>,
    storage_capacity:   u64,
    main_is_active:     bool,
}

impl Storage {

    pub fn new(container: Vec<Container>) -> Storage {
        Storage {
            container,
            storage_capacity: update_storage_capacity(),
            main_is_active: true
        }
    }

    pub fn activate_storage_pool(&mut self) {
        if self.main_is_active {
            return
        }
        self.main_is_active = true;

        let containers = &*self.container;
        let container_size = containers.len();
        while self.main_is_active {
            // let processing_intensity = (jobs_in_queue / self.container.len()) as u32;
            // access containers
        }
    }

    fn count_jobs_in_queue(&self) -> usize {
        let mut count = 0;
        let containers = &*self.container;
        for x in containers {
            count += x.frame_count();
        }
        count
    }
}
