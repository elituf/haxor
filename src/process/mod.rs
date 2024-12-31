#![allow(clippy::module_inception)]
#![allow(clippy::module_name_repetitions)]

mod handle;
mod memory;
mod module;
mod process;
mod snapshot;

pub use module::Module;
pub use process::{Identifier, Process};

type ProcessSnapshot = snapshot::Process;
type ModuleSnapshot = snapshot::Module;
