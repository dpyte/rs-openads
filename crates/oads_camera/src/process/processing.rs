use opencv::{imgproc, types};
use opencv::core::Mat;

/// Detect any facial presence in the frame using the opencv backend
#[inline]
pub fn detect_facial_presence(frame: &Mat) {
	let mut gray = Mat::default();
	let _ = imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0);

	let mut faces = types::VectorOfRect2d::new();
}
