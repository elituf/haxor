use super::handle::Handle;

use std::ffi::c_void;

use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

pub fn read_process_memory<T>(
    handle: &Handle,
    address: usize,
    value: &mut T,
) -> Result<(), crate::Error> {
    unsafe {
        match ReadProcessMemory(
            handle.0,
            address as *const c_void,
            value as *mut T as *mut c_void,
            size_of::<T>(),
            None,
        ) {
            Ok(()) => return Ok(()),
            Err(why) => {
                return Err(crate::Error::MemoryError(format!(
                    "failed to read memory: {why}"
                )));
            }
        };
    }
}

pub fn write_process_memory<T>(
    handle: &Handle,
    address: usize,
    value: &mut T,
) -> Result<(), crate::Error> {
    unsafe {
        match WriteProcessMemory(
            handle.0,
            address as *const c_void,
            value as *mut T as *mut c_void,
            size_of::<T>(),
            None,
        ) {
            Ok(()) => return Ok(()),
            Err(why) => {
                return Err(crate::Error::MemoryError(format!(
                    "failed to write memory: {why}"
                )));
            }
        };
    }
}
