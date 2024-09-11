use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("COM Runtime initialization process failed with error code 0x{0:X}")] 
  ComRuntimeInitFailed(i32),

  #[error("Invalid operation: {0}")]
  InvalidOperation(String),

  #[error("Internal error: {0}")]
  Internal(String),

  #[error("WinAPI error: {0}")]
  WinApi(#[from] windows::core::Error),
}

pub type Result<T> = std::result::Result<T, Error>;