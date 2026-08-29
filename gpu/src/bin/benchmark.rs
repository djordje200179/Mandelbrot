use mandelbrot_core::benchmark;
use mandelbrot_gpu::GpuRenderer;
use std::error::Error;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let renderer = GpuRenderer::new().await?;
    benchmark(&renderer, "mandelbrot-gpu.png").await
}
