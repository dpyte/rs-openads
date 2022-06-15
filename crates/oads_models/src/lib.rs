use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub mod elephant;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
