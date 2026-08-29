use crate::{escape_time, pixel_intensity, ESCAPE_RADIUS_SQUARED};
use mandelbrot_core::{Image, Renderer, Size, MAX_ITERATIONS};
use num_complex::Complex;
use rayon::prelude::*;
use std::convert::Infallible;
use wide::{f32x8, u32x8};

const SIMD_LANES: usize = 8;

pub struct SimdRenderer;

impl Renderer for SimdRenderer {
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
                render_simd_row(row, y, upper_left, real_step, imaginary_step);
            });

        Ok(Image::from_pixels(size, pixels))
    }
}

fn render_simd_row(
    row: &mut [u8],
    y: usize,
    upper_left: Complex<f32>,
    real_step: f32,
    imaginary_step: f32,
) {
    let imaginary = upper_left.im - y as f32 * imaginary_step;
    let simd_width = row.len() / SIMD_LANES * SIMD_LANES;
    let (simd_pixels, tail) = row.split_at_mut(simd_width);

    for (batch, pixels) in simd_pixels.chunks_exact_mut(SIMD_LANES).enumerate() {
        let first_x = batch * SIMD_LANES;
        let real = f32x8::new(std::array::from_fn(|lane| {
            upper_left.re + (first_x + lane) as f32 * real_step
        }));
        let intensities = escape_intensities(real, f32x8::splat(imaginary));
        pixels.copy_from_slice(&intensities);
    }

    for (tail_x, pixel) in tail.iter_mut().enumerate() {
        let x = simd_width + tail_x;
        let point = Complex::new(upper_left.re + x as f32 * real_step, imaginary);
        *pixel = pixel_intensity(escape_time(point));
    }
}

fn escape_intensities(start_real: f32x8, start_imaginary: f32x8) -> [u8; SIMD_LANES] {
    let mut current_real = start_real;
    let mut current_imaginary = start_imaginary;
    let mut iterations = u32x8::ZERO;
    let mut active = f32x8::splat(f32::from_bits(u32::MAX));

    for _ in 0..MAX_ITERATIONS {
        let magnitude_squared = current_real * current_real + current_imaginary * current_imaginary;
        active &= magnitude_squared.simd_le(f32x8::splat(ESCAPE_RADIUS_SQUARED));
        if active.none() {
            break;
        }

        iterations = active.select(iterations + u32x8::ONE, iterations);
        let real_squared = current_real * current_real;
        let imaginary_squared = current_imaginary * current_imaginary;
        current_imaginary =
            current_real * current_imaginary + current_real * current_imaginary + start_imaginary;
        current_real = real_squared - imaginary_squared + start_real;
    }

    iterations
        .to_array()
        .map(|iteration| pixel_intensity((iteration < MAX_ITERATIONS).then_some(iteration)))
}
