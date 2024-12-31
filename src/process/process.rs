use super::{handle::Handle, memory, module::Module, ModuleSnapshot, ProcessSnapshot};

use derive_more::derive::Display;

#[derive(Display)]
pub enum Identifier {
    Pid(u32),
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
pub struct Process {
    pub name: String,
    pub id: u32,
    pub base_address: usize,
    pub handle: Handle,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

impl Process {
    pub fn from<T: Into<Identifier>>(identifier: T) -> Result<Self, crate::Error> {
        let identifier = identifier.into();
        let Some(snapshot) = (match identifier {
            Identifier::Pid(pid) => ProcessSnapshot::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.id == pid),
            Identifier::Name(ref name) => ProcessSnapshot::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.name == *name),
        }) else {
            return Err(crate::Error::ProcessError(format!(
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

    pub fn module(&self, name: &str) -> Result<Module, crate::Error> {
        let Some(snapshot) = ModuleSnapshot::get_modules(self.id)?
            .into_iter()
            .find(|snapshot| snapshot.name == name)
        else {
            return Err(crate::Error::ProcessError(format!(
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

    pub fn resolve_pointer_chain(&self, chain: &[usize]) -> Result<usize, crate::Error> {
        let mut chain = chain.to_vec();
        let mut address = chain.remove(0);
        while chain.len() > 1 {
            address += chain.remove(0);
            address = self.read_mem(address)?;
        }
        Ok(address + chain.remove(0))
    }

    pub fn read_mem<T: Default>(&self, address: usize) -> Result<T, crate::Error> {
        let mut value = Default::default();
        memory::read(&self.handle, address, &mut value)?;
        Ok(value)
    }

    pub fn write_mem<T: Default>(&self, address: usize, mut value: T) -> Result<(), crate::Error> {
        memory::write(&self.handle, address, &mut value)?;
        Ok(())
    }
}
