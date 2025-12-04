use thiserror::Error;

#[derive(Debug, Error)]
/// errors that can occur during interaction with a process
pub enum Error {
    /// there was a failure in obtaining the handle to a process
    #[error("failed to obtain handle")]
    ObtainHandleError(String),
    /// there was a failure when reading or writing the process's memory
    #[error("failed to access process memory")]
    AccessMemoryError(String),
    /// there was a failure in creating a snapshot of processes or modules
    #[error("failed to create snapshot")]
    CreateSnapshotError(String),
    /// there was a failure in building the Process or Module struct
    #[error("failed to get process/module")]
    ProcessError(String),
    /// there was a failure in resolving a pointer chain
    #[error("failed to resolve pointer chain")]
    ResolvePointerChainError(String),
    /// there was a failure in conversion between integers
    #[error("failed to convert integer")]
    ConvertIntegerError(#[from] std::num::TryFromIntError),
}
