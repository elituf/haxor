#![allow(clippy::module_inception)]
#![allow(clippy::module_name_repetitions)]

mod handle;
mod memory;
mod module;
mod process;
mod snapshot;

pub use module::Module;
pub use process::{Identifier, Process};

pub type ProcessSnapshot = snapshot::Process;
pub type ModuleSnapshot = snapshot::Module;
