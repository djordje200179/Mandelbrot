use mandelbrot_core::{Renderer, Size};
use mandelbrot_gpu::GpuRenderer;
use num_complex::Complex;
use std::{error::Error, time::Instant};
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let renderer = GpuRenderer::new().await?;

    let started = Instant::now();
    let image = renderer
        .render(
            Size::new(100_000, 100_000),
            Complex::new(-2.0, 1.25),
            Complex::new(0.5, -1.25),
        )
        .await?;
    println!("GPU render: {:?}", started.elapsed());

    let started = Instant::now();
    image.write("mandelbrot-gpu.png")?;
    println!("GPU PNG write: {:?}", started.elapsed());

    Ok(())
}
