use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to obtain handle")]
    HandleError(String),
    #[error("failed to create instance")]
    ProcessError(String),
    #[error("failed to create snapshot")]
    SnapshotError(String),
    #[error("failed to convert integer")]
    IntegerConversionError(#[from] std::num::TryFromIntError),
    #[error("failed to access process memory")]
    MemoryError(String),
}
