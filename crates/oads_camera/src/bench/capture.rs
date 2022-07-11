
use std::thread;
use std::time::{Duration, SystemTime};
use crossbeam::channel::Sender;
use v4l::io::traits::CaptureStream;
use crate::camera::{FrameBuff, Streamer};

pub fn timeit<F: Fn() -> T, T>(f: F) -> T {
	let start = SystemTime::now();
	let result = f();
	let end = SystemTime::now();
	let duration = end.duration_since(start).unwrap();
	println!("Time taken by function: {} ms", duration.as_micros());
	result
}

pub fn frame_transmission() {
	let (tx, rx) = crossbeam::channel::bounded(8);
	thread::spawn(move || begin_capture(tx));

	thread::sleep(Duration::from_millis(500));
	for _ in 0..1 {
		let _ = match rx.recv() {
			Ok(obj) => obj.buffer(),
			Err(_) => vec![],
		};
	};
}

fn begin_capture(tx: Sender<FrameBuff>) {
	let mut streamer = Streamer::new().unwrap();
	loop {
		let (buf, meta) = streamer.stream.next().unwrap();
		let buffer = FrameBuff::new(buf);
		tx.send(buffer).unwrap();
	}
}
