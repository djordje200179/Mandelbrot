use crate::render_row;
use mandelbrot_core::{Image, Renderer, Size};
use num_complex::Complex;
use rayon::prelude::*;
use std::convert::Infallible;

pub struct ParallelRenderer;

impl Renderer for ParallelRenderer {
    type Error = Infallible;

    async fn render(
        &self,
        size: Size,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<Image, Self::Error> {
        let mut pixels = vec![0; size.area()];
        if size.width == 0 || size.height == 0 {
            return Ok(Image::from_pixels(size, pixels));
        }

        let real_step = (lower_right.re - upper_left.re) / size.width as f32;
        let imaginary_step = (upper_left.im - lower_right.im) / size.height as f32;

        pixels
            .par_chunks_mut(size.width)
            .enumerate()
            .for_each(|(y, row)| {
                render_row(row, y, upper_left, real_step, imaginary_step);
            });

        Ok(Image::from_pixels(size, pixels))
    }
}
