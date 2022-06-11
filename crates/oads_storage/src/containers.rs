use std::fs;
use std::sync::{Arc, Mutex};
use chrono::{Local};
use opencv::core::Mat;
use opencv::core::Size;
use opencv::dnn::print;
use opencv::prelude::*;
use opencv::videoio::VideoWriter;

type ArcMutVW = Arc<Mutex<VideoWriter>>;


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

        let mut save_root = String::from("/var/system/openads/storage/");
        save_root.push_str("/");
        save_root.push_str(id.as_str());

        fs::create_dir_all(save_root.to_string()).expect("failed to create dir");
        let save_to: String = vec![save_root.to_string(),
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
            save_root,
            save_to: save_to.to_string(),
            video_writer,
            frames: Vec::new()
        }
    }

    pub fn writer(&self) -> &VideoWriter {  &self.video_writer }

    /// Write data to the file
    pub fn write(&mut self) {
        if self.frames.is_empty() {
            return
        }

        let front = self.frames.first().unwrap();
        let _ = match self.video_writer.write(front) {
            _ => ()
        };
        self.frames.pop();
    }
}

