use super::{handle::Handle, memory, module::Module, ModuleSnapshot, ProcessSnapshot};

use derive_more::derive::Display;

#[derive(Debug, Default, Clone)]
pub struct Process {
    pub name: String,
    pub id: u32,
    pub base_address: usize,
    pub handle: Handle,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

#[derive(Display)]
pub enum Identifier {
    Id(u32),
    Name(String),
}

impl Process {
    pub fn from(identifier: &Identifier) -> Result<Self, crate::Error> {
        let snapshot = match identifier {
            Identifier::Id(pid) => ProcessSnapshot::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.id == *pid),
            Identifier::Name(ref name) => ProcessSnapshot::get_processes()?
                .into_iter()
                .find(|snapshot| snapshot.name == *name),
        };
        let Some(snapshot) = snapshot else {
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

    pub fn with_pid(pid: u32) -> Result<Self, crate::Error> {
        Self::from(&Identifier::Id(pid))
    }

    pub fn with_name(name: &str) -> Result<Self, crate::Error> {
        Self::from(&Identifier::Name(name.to_string()))
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

    pub fn read_mem<T: Default>(&self, address: usize) -> Result<T, crate::Error> {
        let mut value = Default::default();
        memory::read(&self.handle, address, &mut value)?;
        Ok(value)
    }

    pub fn read_mem_from_ptr_chain<T: Default>(&self, chain: &[usize]) -> Result<T, crate::Error> {
        let mut chain = chain.to_vec();
        let mut address = chain.remove(0);
        while chain.len() > 1 {
            address += chain.remove(0);
            address = self.read_mem(address)?;
        }
        let value = self.read_mem(address + chain.remove(0))?;
        Ok(value)
    }

    pub fn addr_from_ptr_chain(&self, chain: &[usize]) -> Result<usize, crate::Error> {
        let mut chain = chain.to_vec();
        let mut address = chain.remove(0);
        while chain.len() > 1 {
            address += chain.remove(0);
            address = self.read_mem(address)?;
        }
        Ok(address + chain.remove(0))
    }

    pub fn write_mem<T: Default>(&self, address: usize, mut value: T) -> Result<(), crate::Error> {
        memory::write(&self.handle, address, &mut value)?;
        Ok(())
    }
}
