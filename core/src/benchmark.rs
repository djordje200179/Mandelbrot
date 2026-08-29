use crate::{Renderer, Size};
use num_complex::Complex;
use std::{error::Error, time::Instant};

pub async fn benchmark(
    renderer: &impl Renderer<Error: 'static>,
    output_filename: &str,
) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let image = renderer
        .render(
            Size::new(100_000, 100_000),
            Complex::new(-2.0, 1.25),
            Complex::new(0.5, -1.25),
        )
        .await?;
    println!("Render: {:?}", started.elapsed());

    let started = Instant::now();
    image.write(output_filename)?;
    println!("PNG write: {:?}", started.elapsed());

    Ok(())
}
