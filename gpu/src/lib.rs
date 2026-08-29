mod error;
mod shader;

use bytemuck::{bytes_of, cast_slice};
use mandelbrot_core::{Image, Renderer, Size, MAX_ITERATIONS};
use num_complex::Complex;
use shader::{
    create_render_pipeline_embed_source, RenderParameters, WgpuBindGroup0, WgpuBindGroup0Entries,
    WgpuBindGroup0EntriesParams, RENDER_WORKGROUP_SIZE,
};
use std::sync::mpsc::channel;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferBinding, BufferDescriptor, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipeline, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, MapMode, MemoryHints, PollType, PowerPreference, Queue,
    RequestAdapterOptions, Trace,
};

pub use error::GpuError;

const TARGET_TILE_BYTES: u64 = 64 * 1024 * 1024;

pub struct GpuRenderer {
    device: Device,
    queue: Queue,
    pipeline: ComputePipeline,
    max_buffer_size: u64,
}

impl GpuRenderer {
    pub async fn new() -> Result<Self, GpuError> {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await?;
        let adapter_limits = adapter.limits();
        let required_limits = Limits {
            max_storage_buffer_binding_size: adapter_limits.max_storage_buffer_binding_size,
            max_buffer_size: adapter_limits.max_buffer_size,
            ..Limits::default()
        };
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Mandelbrot device"),
                required_features: Features::empty(),
                required_limits,
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await?;
        let pipeline = create_render_pipeline_embed_source(&device);

        Ok(Self {
            device,
            queue,
            pipeline,
            max_buffer_size: adapter_limits
                .max_buffer_size
                .min(adapter_limits.max_storage_buffer_binding_size as u64),
        })
    }

    async fn render_image(
        &self,
        size: Size,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<Image, GpuError> {
        let width = u32::try_from(size.width).map_err(|_| GpuError::ImageTooLarge)?;
        let height = u32::try_from(size.height).map_err(|_| GpuError::ImageTooLarge)?;
        let mut pixels = vec![0; size.area()];
        if width == 0 || height == 0 {
            return Ok(Image::from_pixels(size, pixels));
        }

        let bytes_per_row = u64::from(width) * size_of::<u32>() as u64;
        let tile_bytes = TARGET_TILE_BYTES.min(self.max_buffer_size);
        let tile_height = (tile_bytes / bytes_per_row).clamp(1, u64::from(height)) as u32;
        let step = [
            (lower_right.re - upper_left.re) / size.width as f32,
            (upper_left.im - lower_right.im) / size.height as f32,
        ];

        for tile_y in (0..height).step_by(tile_height as usize) {
            let current_height = tile_height.min(height - tile_y);
            let parameters = RenderParameters::new(
                [width, height],
                tile_y,
                current_height,
                [upper_left.re, upper_left.im],
                step,
                MAX_ITERATIONS,
            );
            let start = tile_y as usize * size.width;
            let end = start + current_height as usize * size.width;
            self.render_tile(parameters, &mut pixels[start..end])
                .await?;
        }

        Ok(Image::from_pixels(size, pixels))
    }

    async fn render_tile(
        &self,
        parameters: RenderParameters,
        pixels: &mut [u8],
    ) -> Result<(), GpuError> {
        let output_size = (pixels.len() * size_of::<u32>()) as u64;
        let parameter_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Mandelbrot parameters"),
            contents: bytes_of(&parameters),
            usage: BufferUsages::UNIFORM,
        });
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Mandelbrot output"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let staging_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Mandelbrot staging"),
            size: output_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let bind_group = WgpuBindGroup0::from_bindings(
            &self.device,
            WgpuBindGroup0Entries::new(WgpuBindGroup0EntriesParams {
                parameters: entire_buffer_binding(&parameter_buffer),
                pixels: entire_buffer_binding(&output_buffer),
            }),
        );

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Mandelbrot command encoder"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Mandelbrot compute pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            bind_group.set(&mut pass);
            pass.dispatch_workgroups(
                parameters.image_size[0].div_ceil(RENDER_WORKGROUP_SIZE[0]),
                parameters.tile_height.div_ceil(RENDER_WORKGROUP_SIZE[1]),
                1,
            );
        }
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_size);
        self.queue.submit([encoder.finish()]);

        let slice = staging_buffer.slice(..);
        let (sender, receiver) = channel();
        slice.map_async(MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        self.device.poll(PollType::Wait)?;
        receiver
            .recv()
            .map_err(|_| GpuError::MappingChannelClosed)??;

        let mapped = slice.get_mapped_range();
        let values: &[u32] = cast_slice(&mapped);
        for (pixel, value) in pixels.iter_mut().zip(values) {
            *pixel = *value as u8;
        }
        drop(mapped);
        staging_buffer.unmap();

        Ok(())
    }
}

impl Renderer for GpuRenderer {
    type Error = GpuError;

    async fn render(
        &self,
        size: Size,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<Image, Self::Error> {
        self.render_image(size, upper_left, lower_right).await
    }
}

fn entire_buffer_binding(buffer: &Buffer) -> BufferBinding<'_> {
    BufferBinding {
        buffer,
        offset: 0,
        size: None,
    }
}
