use super::{handle::Handle, snapshot};

use derive_more::derive::Display;

#[derive(Debug, Default)]
pub struct Process {
    name: String,
    id: u32,
    base_address: usize,
    handle: Handle,
}

#[derive(Display)]
pub enum Identifier {
    Id(u32),
    Name(String),
}

impl Process {
    pub fn from(identifier: &Identifier) -> Result<Self, crate::Error> {
        let snapshot = match identifier {
            Identifier::Id(pid) => snapshot::Process::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.id == *pid),
            Identifier::Name(ref name) => snapshot::Process::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.name == *name),
        };
        let Some(snapshot) = snapshot else {
            return Err(crate::Error::ProcessError(format!(
                "couldn't find a process with identifier `{identifier}`",
            )));
        };
        let process = Self {
            name: snapshot.name,
            id: snapshot.id,
            base_address: 0,
            handle: Handle::from_pid(snapshot.id)?,
        };
        Ok(process)
    }

    pub fn with_pid(pid: u32) -> Result<Self, crate::Error> {
        Self::from(&Identifier::Id(pid))
    }

    pub fn with_name(name: &str) -> Result<Self, crate::Error> {
        Self::from(&Identifier::Name(name.to_string()))
    }
}
