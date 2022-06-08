use std::fs;
use chrono::{Local};
use opencv::core::Mat;
use opencv::core::Size;
use opencv::prelude::*;
use opencv::videoio::VideoWriter;

fn update_last_saved(last_saved: String) -> String {
    let local: String = Local::now().date().to_string()
        .replace('-', "")
        .replace(':', "");

    if local == last_saved {
        return last_saved
    }
    local
}

pub struct Container {
    name:           String,
    id:             String,
    save_root:      String,
    save_to:        String,
    video_writer:   VideoWriter,
    frames:         Vec<Mat>,
}

impl Container {
    pub fn new(name: String, id: String) -> Container {
        let last_saved = update_last_saved(String::new());

        let mut root_storage = String::from("/var/system/openads/storage/");
        root_storage.push_str("/");
        root_storage.push_str(id.as_str());

        fs::create_dir_all(root_storage.to_string()).expect("failed to create dir");
        let save_to: String = vec![root_storage.to_string(),
                                   "/".to_string(),
                                   last_saved.to_string(),
                                   ".avi".to_string()]
            .join("");

        let frame_size = Size::new(320, 240);
        let fourcc = VideoWriter::fourcc('A' as i8, 'V' as i8,
                                         'I' as i8, '1' as i8).unwrap();
        let video_writer = VideoWriter::new(&*save_to, fourcc,
                             30 as f64,  frame_size, true).unwrap();
        Container {
            name,
            id,
            save_root: root_storage,
            save_to: save_to.to_string(),
            video_writer,
            frames: vec![]
        }
    }
}

