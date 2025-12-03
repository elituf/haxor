#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod error;
/// types and methods to ease the r/w of a process's memory
pub mod process;
mod sys;

pub use error::Error;
