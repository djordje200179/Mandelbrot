pub mod image;

use image::{Image, Point, Size};

fn main() {
	let image = Image::create(
		Size::new(20000, 20000),
		Point::new(-2.0, 1.25),
		Point::new(0.5, -1.25),
	);

	image.write("mandelbrot.png").expect("Failed to write image!");
}
