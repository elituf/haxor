#![allow(clippy::module_inception)]

mod handle;
mod memory;
mod module;
mod process;
mod snapshot;

pub use module::Module;
pub use process::{Identifier, Process};

pub type ProcessSnapshot = snapshot::Process;
pub type ModuleSnapshot = snapshot::Module;
