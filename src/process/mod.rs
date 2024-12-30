#![allow(clippy::module_inception)]

mod handle;
mod memory;
mod process;
mod snapshot;

pub use process::{Identifier, Process};
