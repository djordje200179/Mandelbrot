use mandelbrot_core::benchmark;
use mandelbrot_cpu::SequentialRenderer;
use std::error::Error;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    benchmark(&SequentialRenderer, "mandelbrot-cpu-sequential.png").await
}
