use crate::error::Error;
use std::ops::Deref;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ, PROCESS_VM_WRITE},
};

#[derive(Debug, Default, Clone)]
pub struct Handle(pub HANDLE);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let Err(why) = unsafe { CloseHandle(**self) } {
            log::error!("failed to close handle: {why}");
        }
    }
}

impl Handle {
    pub fn from_pid(pid: u32) -> Result<Self, Error> {
        let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid) }
            .or_else(|_| unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE, false, pid) })
            .map_err(|why| {
                Error::ObtainHandleError(format!(
                    "failed to open process with needed access: {why}",
                ))
            })?;
        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::ObtainHandleError(
                "failed to get a valid handle".to_string(),
            ));
        }
        Ok(Self(handle))
    }
}
