use crate::{Image, Size};
use num_complex::Complex;
use std::error::Error;

pub const MAX_ITERATIONS: u32 = u8::MAX as u32;

pub trait Renderer {
    type Error: Error;

    async fn render(
        &self,
        size: Size,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<Image, Self::Error>;
}
