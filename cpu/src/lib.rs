mod parallel;
mod sequential;
mod simd;

use mandelbrot_core::MAX_ITERATIONS;
use num_complex::Complex;

pub use parallel::ParallelRenderer;
pub use sequential::SequentialRenderer;
pub use simd::SimdRenderer;

const ESCAPE_RADIUS_SQUARED: f32 = 4.0;

pub(crate) fn render_row(
    row: &mut [u8],
    y: usize,
    upper_left: Complex<f32>,
    real_step: f32,
    imaginary_step: f32,
) {
    let imaginary = upper_left.im - y as f32 * imaginary_step;

    for (x, pixel) in row.iter_mut().enumerate() {
        let point = Complex::new(upper_left.re + x as f32 * real_step, imaginary);
        *pixel = pixel_intensity(escape_time(point));
    }
}

fn pixel_intensity(escape_time: Option<u32>) -> u8 {
    escape_time.map(|iterations| iterations as u8).unwrap_or(0)
}

fn escape_time(start_point: Complex<f32>) -> Option<u32> {
    let mut current_point = start_point;

    for iteration in 0..MAX_ITERATIONS {
        if current_point.norm_sqr() > ESCAPE_RADIUS_SQUARED {
            return Some(iteration);
        }

        current_point = current_point * current_point + start_point;
    }

    None
}
