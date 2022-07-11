use std::fmt;
use std::io::Error;
use v4l::buffer::Type;
use v4l::video::Capture;
use std::fmt::Formatter;
use v4l::io::mmap::Stream;
use v4l::{Device, Format, FourCC};

pub struct FrameBuff {
	buff:       Vec<u8>
}

impl FrameBuff {
	///
	/// Create frame buffer obtained from v4l
	///
	pub fn new(raw_bytes: &[u8]) -> Self {
		let buff = raw_bytes.to_vec();
		Self { buff }
	}

	pub fn buffer(&self) -> Vec<u8> {
		let buf = self.buff.clone();
		buf
	}
}

///
/// Streamer: Contains information about the device
///
pub struct Streamer<'a> {
	pub stream: Stream<'a>,
	pub fmt: Format,
	device: Device,
}

impl Streamer<'_> {
	pub fn new() -> Result<Self, Error> {
		let mut device = Device::new(0)?;
		let mut fmt = device.format()?;

		fmt.width = 1280;
		fmt.height = 720;
		fmt.fourcc = FourCC::new(b"AVI1");
		device.set_format(&fmt)?;

		let mut stream = Stream::with_buffers(&mut device, Type::VideoCapture, 8)?;
		Ok(Self { device, fmt, stream })
	}
}

// Debug
impl fmt::Debug for Streamer<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{:#?}\n", self.fmt)
	}
}
