use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("couldn't get handle to process")]
    HandleError(String),
    #[error("couldn't create instance of process")]
    ProcessError(String),
    #[error("couldn't create snapshot")]
    SnapshotError(String),
    #[error("failed to convert integer")]
    IntegerConversionError(#[from] std::num::TryFromIntError),
}
