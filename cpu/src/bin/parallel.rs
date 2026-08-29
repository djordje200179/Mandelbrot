use mandelbrot_core::{Renderer, Size};
use mandelbrot_cpu::ParallelRenderer;
use num_complex::Complex;
use std::{error::Error, time::Instant};
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let image = ParallelRenderer
        .render(
            Size::new(100_000, 100_000),
            Complex::new(-2.0, 1.25),
            Complex::new(0.5, -1.25),
        )
        .await?;
    println!("Parallel CPU render: {:?}", started.elapsed());

    let started = Instant::now();
    image.write("mandelbrot-cpu-parallel.png")?;
    println!("Parallel CPU PNG write: {:?}", started.elapsed());

    Ok(())
}
