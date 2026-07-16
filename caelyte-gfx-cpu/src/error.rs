use softbuffer::SoftBufferError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaelyteError {
    #[error("Failed to initialize softbuffer: {0}")]
    SoftBuffer(#[from] SoftBufferError),

    #[error("The window size provided was invalid: {width}x{height}")]
    InvalidWindowSize { width: u32, height: u32 },
}
