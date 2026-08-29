use mandelbrot_core::benchmark;
use mandelbrot_cpu::ParallelRenderer;
use std::error::Error;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    benchmark(&ParallelRenderer, "mandelbrot-cpu-parallel.png").await
}
