use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to obtain handle")]
    ObtainHandleError(String),
    #[error("failed to access process memory")]
    AccessMemoryError(String),
    #[error("failed to create process")]
    CreateProcessError(String),
    #[error("failed to create snapshot")]
    CreateSnapshotError(String),
    #[error("failed to convert integer")]
    ConvertIntegerError(#[from] std::num::TryFromIntError),
}
