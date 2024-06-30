pub mod image;

use image::{Image, Point, Size};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let image = Image::create(
		Size::new(20000, 20000),
		Point::new(-2.0, 1.25),
		Point::new(0.5, -1.25),
	);

	image.write("mandelbrot.png")?;

	Ok(())
}
