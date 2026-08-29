use thiserror::Error;
use wgpu::{BufferAsyncError, PollError, RequestAdapterError, RequestDeviceError};

#[derive(Debug, Error)]
pub enum GpuError {
    #[error("no compatible GPU adapter was found")]
    Adapter(#[from] RequestAdapterError),
    #[error("failed to create GPU device: {0}")]
    Device(#[from] RequestDeviceError),
    #[error("image dimensions exceed GPU renderer limits")]
    ImageTooLarge,
    #[error("GPU buffer mapping failed: {0}")]
    BufferMap(#[from] BufferAsyncError),
    #[error("GPU stopped before buffer mapping completed")]
    MappingChannelClosed,
    #[error("GPU execution failed: {0}")]
    Execution(#[from] PollError),
}
