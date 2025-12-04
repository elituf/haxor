use crate::{
    Error,
    sys::{handle::Handle, memory, snapshot},
};
use derive_more::{Debug, derive::Display};

#[derive(Display)]
/// an identifier for searching for a process
pub enum Identifier {
    /// process id to search for
    Pid(u32),
    /// process name to search for
    Name(String),
}

impl From<u32> for Identifier {
    fn from(value: u32) -> Self {
        Self::Pid(value)
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self::Name(value.to_string())
    }
}

#[derive(Debug, Default, Clone)]
/// a process running on the system
pub struct Process {
    /// the process name (szExeFile)
    pub name: String,
    /// the process id  (th32ProcessID)
    pub id: u32,
    /// the base address of the module with the name `name` (modBaseAddr)
    #[debug("0x{base_address:X}")]
    pub base_address: usize,
    /// the process handle (HANDLE)
    pub handle: Handle,
}

impl Process {
    /// initialize a `Process` from a pid or a process name
    pub fn find<T: Into<Identifier>>(identifier: T) -> Result<Self, Error> {
        let identifier = identifier.into();
        let snapshot = snapshot::ProcessSnapshot::get_processes()?
            .into_iter()
            .find(|snapshot| match identifier {
                Identifier::Pid(pid) => snapshot.id == pid,
                Identifier::Name(ref name) => snapshot.name == *name,
            })
            .ok_or_else(|| {
                Error::ProcessError(format!(
                    "failed to find a process with identifier `{identifier}`",
                ))
            })?;
        let mut process = Self {
            name: snapshot.name,
            id: snapshot.id,
            base_address: 0,
            handle: Handle::from_pid(snapshot.id)?,
        };
        process.base_address = process.module(&process.name)?.base_address;
        Ok(process)
    }

    /// get a `Module` of a `Process` by name (case-insensitive)
    pub fn module(&self, name: &str) -> Result<Module, Error> {
        let Some(snapshot) = snapshot::ModuleSnapshot::get_modules(self.id)?
            .into_iter()
            .find(|snapshot| name.eq_ignore_ascii_case(&snapshot.name))
        else {
            return Err(Error::ProcessError(format!(
                "failed to find a module with identifier `{name}`",
            )));
        };
        let module = Module {
            process_id: self.id,
            name: snapshot.name,
            path: snapshot.path,
            base_address: snapshot.base_address,
            base_size: snapshot.base_size,
        };
        Ok(module)
    }

    /// get all `Module`s of a `Process`
    pub fn modules(&self) -> Result<Vec<Module>, Error> {
        Ok(snapshot::ModuleSnapshot::get_modules(self.id)?
            .iter()
            .cloned()
            .map(|snapshot| Module {
                process_id: self.id,
                name: snapshot.name,
                path: snapshot.path,
                base_address: snapshot.base_address,
                base_size: snapshot.base_size,
            })
            .collect())
    }

    /// follow a pointer chain to the end and return an address
    pub fn resolve_pointer_chain(&self, chain: &[usize]) -> Result<usize, Error> {
        if chain.is_empty() {
            return Err(Error::ResolvePointerChainError("chain was empty".into()));
        }
        let mut address = chain[0];
        for &offset in &chain[1..(chain.len() - 1)] {
            address += offset;
            address = self.read_mem(address)?;
        }
        Ok(address + chain.last().expect("chain should have a last element"))
    }

    /// read a given `address` of process's memory
    pub fn read_mem<T: Default>(&self, address: usize) -> Result<T, Error> {
        let mut value = Default::default();
        memory::read(&self.handle, address, &mut value)?;
        Ok(value)
    }

    /// write a `value` at given `address` of process's memory
    pub fn write_mem<T>(&self, address: usize, mut value: T) -> Result<(), Error> {
        memory::write(&self.handle, address, &mut value)?;
        Ok(())
    }
}

#[derive(Debug, Default)]
/// a module running within a process
pub struct Module {
    /// the parent process id (th32ProcessID)
    pub process_id: u32,
    /// the module name (szModule)
    pub name: String,
    /// the module executable path (szExePath)
    pub path: String,
    /// the module base address (modBaseAddr)
    #[debug("0x{base_address:X}")]
    pub base_address: usize,
    /// the module base size (modBaseSize)
    #[debug("0x{base_size:X}")]
    pub base_size: usize,
}
