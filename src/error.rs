use thiserror::Error;

/// Error type for Renga API.
#[derive(Error, Debug)]
pub enum Error {
  /// COM Runtime initialization process failed.
  /// 
  /// Can occur only when `CoInitialize` fails.
  #[error("COM Runtime initialization process failed with error code 0x{0:X}")] 
  ComRuntimeInitFailed(i32),

  #[error("Invalid operation: {0}")]
  InvalidOperation(String),

  /// Already opened.
  #[error("Already opened: {0}")]
  AlreadyOpened(String),

  #[error("Internal error: {0}")]
  Internal(String),

  /// Filesystem path does not exist.
  #[error("Nonexistent path: {0}")]
  NonexistentPath(String),

  #[error("Invalid entity type: {0}")]
  InvalidCategory(crate::EntityTypes),

  #[error("No active transaction. Start new transaction first.")]
  NoActiveTransaction,

  /// Error parsing value from string.
  #[error("Error parsing value from string: {0}")]
  ParseError(String),

  /// WinAPI error.
  /// 
  /// Can occur in cases where WinAPI functions fail. 
  #[error("WinAPI error: {0}")]
  WinApi(#[from] windows::core::Error),
}

/// Result type for Renga API.
pub type Result<T> = std::result::Result<T, Error>;