use mandelbrot_core::{Image, Renderer, Size, MAX_ITERATIONS};
use num_complex::Complex;
use rayon::prelude::*;
use std::convert::Infallible;

const ESCAPE_RADIUS_SQUARED: f32 = 4.0;

pub struct CpuRenderer;

impl Renderer for CpuRenderer {
    type Error = Infallible;

    async fn render(
        &self,
        size: Size,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<Image, Self::Error> {
        Ok(render(size, upper_left, lower_right))
    }
}

fn render(size: Size, upper_left: Complex<f32>, lower_right: Complex<f32>) -> Image {
    let mut pixels = vec![0; size.area()];
    if size.width == 0 || size.height == 0 {
        return Image::from_pixels(size, pixels);
    }

    let real_step = (lower_right.re - upper_left.re) / size.width as f32;
    let imaginary_step = (upper_left.im - lower_right.im) / size.height as f32;

    pixels
        .par_chunks_mut(size.width)
        .enumerate()
        .for_each(|(y, row)| {
            let imaginary = upper_left.im - y as f32 * imaginary_step;

            for (x, pixel) in row.iter_mut().enumerate() {
                let real = upper_left.re + x as f32 * real_step;
                *pixel = pixel_intensity(escape_time(real, imaginary));
            }
        });

    Image::from_pixels(size, pixels)
}

fn pixel_intensity(escape_time: Option<u32>) -> u8 {
    escape_time.map(|iterations| iterations as u8).unwrap_or(0)
}

fn escape_time(start_real: f32, start_imaginary: f32) -> Option<u32> {
    let mut current_real = start_real;
    let mut current_imaginary = start_imaginary;

    for iteration in 0..MAX_ITERATIONS {
        let real_squared = current_real * current_real;
        let imaginary_squared = current_imaginary * current_imaginary;

        if real_squared + imaginary_squared > ESCAPE_RADIUS_SQUARED {
            return Some(iteration);
        }

        current_imaginary = 2.0 * current_real * current_imaginary + start_imaginary;
        current_real = real_squared - imaginary_squared + start_real;
    }

    None
}
