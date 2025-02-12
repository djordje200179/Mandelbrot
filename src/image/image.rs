use std::fs::File;
use num_complex::Complex;
use png::{BitDepth, ColorType, Encoder};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use super::{WriteResult, Size};

pub struct Image {
	size: Size,
	pixels: Vec<u8>,
}

pub type Point = Complex<f64>;

impl Image {
	pub fn create(
		size: Size,
		upper_left: Point,
		lower_right: Point,
	) -> Image {
		let pixels: Vec<u8> = (0..size.area()).into_par_iter().map(|i| {
			let x = i % size.width;
			let y = i / size.width;

			let point = size.calc_point((x, y), upper_left, lower_right);
			let escape_time = escape_time(point);

			match escape_time {
				Some(i) => ((i * u8::MAX as usize) / ITERATIONS) as u8,
				None => 0
			}
		}).collect();

		Image { size, pixels }
	}

	pub fn write(&self, filename: &str) -> WriteResult {
		let mut encoder = Encoder::new(
			File::create(filename)?,
			self.size.width as u32, self.size.height as u32
		);
	
		encoder.set_color(ColorType::Grayscale);
		encoder.set_depth(BitDepth::Eight);
	
		let mut writer = encoder.write_header()?;
		writer.write_image_data(&self.pixels)?;
	
		return Ok(());
	}
}

const ITERATIONS: usize = u8::MAX as usize / 2;

fn escape_time(start_point: Complex<f64>) -> Option<usize> {
    let mut curr_point = start_point;

    for i in 0..ITERATIONS {
        if curr_point.norm_sqr() > 4.0 {
            return Some(i);
        }

        curr_point = curr_point * curr_point + start_point;
    }

    return None;
}