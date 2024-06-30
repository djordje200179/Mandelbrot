use num_complex::Complex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
	pub width: usize,
	pub height: usize,
}

impl Size {
	pub fn new(width: usize, height: usize) -> Size {
		Size { width, height }
	}

	pub fn area(&self) -> usize {
		self.width * self.height
	}

	pub fn calc_point(
		self, pixel: (usize, usize),
		upper_left: Complex<f64>, 
		lower_right: Complex<f64>
	) -> Complex<f64> {
		let (width, height) = (
			lower_right.re - upper_left.re,
			upper_left.im - lower_right.im
		);

		Complex {
			re: upper_left.re + pixel.0 as f64 * width / self.width as f64,
			im: upper_left.im - pixel.1 as f64 * height / self.height as f64
		}
	}
}