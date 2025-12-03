use crate::{
    process::{handle::Handle, memory, snapshot, Module},
    Error,
};
use derive_more::derive::Display;

#[derive(Display)]
/// an identifier for searching for a process
pub enum Identifier {
    /// the process id
    Pid(u32),
    /// the process name
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
    /// the base address of the module with the same name as `name` (modBaseAddr)
    pub base_address: usize,
    /// the process handle (HANDLE)
    pub handle: Handle,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

impl Process {
    /// initialize a `Process` from a pid or a process name
    pub fn from<T: Into<Identifier>>(identifier: T) -> Result<Self, Error> {
        let identifier = identifier.into();
        let Some(snapshot) = (match identifier {
            Identifier::Pid(pid) => snapshot::ProcessSnapshot::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.id == pid),
            Identifier::Name(ref name) => snapshot::ProcessSnapshot::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.name == *name),
        }) else {
            return Err(Error::CreateProcessError(format!(
                "failed to find a process with identifier `{identifier}`",
            )));
        };
        let mut process = Self {
            name: snapshot.name,
            id: snapshot.id,
            base_address: 0,
            handle: Handle::from_pid(snapshot.id)?,
        };
        process.base_address = process.module(&process.name)?.base_address;
        Ok(process)
    }

    /// get the `Module` of a `Process` by name
    pub fn module(&self, name: &str) -> Result<Module, Error> {
        let Some(snapshot) = snapshot::ModuleSnapshot::get_modules(self.id)?
            .into_iter()
            .find(|snapshot| snapshot.name == name)
        else {
            return Err(Error::CreateProcessError(format!(
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

    /// follow a pointer chain to the end and return an address
    pub fn resolve_pointer_chain(&self, chain: &[usize]) -> Result<usize, Error> {
        let mut chain = chain.to_vec();
        let mut address = chain.remove(0);
        while chain.len() > 1 {
            address += chain.remove(0);
            address = self.read_mem(address)?;
        }
        Ok(address + chain.remove(0))
    }

    /// read a given `address` of process's memory
    pub fn read_mem<T: Default>(&self, address: usize) -> Result<T, Error> {
        let mut value = Default::default();
        memory::read(&self.handle, address, &mut value)?;
        Ok(value)
    }

    /// write a `value` at given `address` of process's memory
    pub fn write_mem<T: Default>(&self, address: usize, mut value: T) -> Result<(), Error> {
        memory::write(&self.handle, address, &mut value)?;
        Ok(())
    }
}
