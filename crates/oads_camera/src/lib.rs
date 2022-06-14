extern crate core;

use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

pub mod info;
pub mod scan;
pub mod read;
pub mod vision;
mod camera;
mod storage;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
