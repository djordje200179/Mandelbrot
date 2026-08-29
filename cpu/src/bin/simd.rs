use mandelbrot_core::benchmark;
use mandelbrot_cpu::SimdRenderer;
use std::error::Error;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    benchmark(&SimdRenderer, "mandelbrot-cpu-simd.png").await
}
