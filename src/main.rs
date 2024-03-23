use std::fs::File;
use std::thread;
use num_complex::Complex;
use png::EncodingError;

const ITERATIONS: usize = 63;

fn escape_time(start_point: Complex<f64>) -> Option<usize> {
    let mut curr_point = start_point;

    for i in 0..ITERATIONS {
        if curr_point.norm() > 2.0 {
            return Some(i);
        }

        curr_point = curr_point * curr_point + start_point;
    }

    return None;
}

#[inline(always)]
fn pixel_to_point(
    image_size: (usize, usize),
    pixel: (usize, usize),
    upper_left_point: Complex<f64>,
    lower_right_point: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (
        lower_right_point.re - upper_left_point.re,
        upper_left_point.im - lower_right_point.im
    );

    return Complex {
        re: upper_left_point.re + pixel.0 as f64 * width / image_size.0 as f64,
        im: upper_left_point.im - pixel.1 as f64 * height / image_size.1 as f64
    }
}

fn render_image(
    image_size: (usize, usize),
    pixels: &mut [u8],
    upper_left_point: Complex<f64>,
    lower_right_point: Complex<f64>
) {
    debug_assert_eq!(pixels.len(), image_size.0 * image_size.1);

    let chunks = pixels.chunks_mut(image_size.0);

    thread::scope(|spawner| {
        for (y, row) in chunks.enumerate() {
            spawner.spawn(move || {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let point = pixel_to_point(image_size, (x, y), upper_left_point, lower_right_point);
                    let escape_time = escape_time(point);

                    *pixel = match escape_time {
                        Some(i) => ((i * u8::MAX as usize) / ITERATIONS) as u8,
                        None => 0
                    };
                }
            });
        }
    });
}

fn write_image(
    image_size: (usize, usize),
    pixels: &[u8],
    filename: &str
) -> Result<(), EncodingError> {
    let mut encoder = png::Encoder::new(
        File::create(filename).unwrap(),
        image_size.0 as u32, image_size.1 as u32
    );

    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(pixels)?;

    return Ok(());
}

fn main() {
    let size = (10000, 10000);

    let mut pixels = vec![0; size.0 * size.1];
    render_image(
        size,
        &mut pixels,
        Complex { re: -2.0, im: 1.0 },
        Complex { re: 0.0, im: -1.0 }
    );

    write_image(size, &pixels, "mandelbrot.png").expect("Failed to write image!");
}
