pub mod audio;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FitzgeraldError {
    //get errors from either symph or hound in audio, so combine
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Audio decoding failed: {0}")]
    Symphonia(#[from] symphonia::core::errors::Error),

    #[error("WAV encoding failed: {0}")]
    Hound(#[from] hound::Error),

    #[error("Invalid parameters: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, FitzgeraldError>;
