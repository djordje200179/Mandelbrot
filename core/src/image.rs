use crate::{Size, WriteResult};
use png::{BitDepth, ColorType, Encoder};
use std::fs::File;

pub struct Image {
    size: Size,
    pixels: Vec<u8>,
}

impl Image {
    pub fn from_pixels(size: Size, pixels: Vec<u8>) -> Self {
        Self { size, pixels }
    }

    pub fn write(&self, filename: &str) -> WriteResult {
        let mut encoder = Encoder::new(
            File::create(filename)?,
            self.size.width as u32,
            self.size.height as u32,
        );

        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }
}
