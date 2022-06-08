
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

pub mod storage;
pub mod containers;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
