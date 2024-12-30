use super::handle::Handle;

use std::{ffi::c_void, ptr};

use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

pub fn read<T>(handle: &Handle, address: usize, value: &mut T) -> Result<(), crate::Error> {
    unsafe {
        match ReadProcessMemory(
            handle.0,
            address as *const c_void,
            (ptr::from_mut::<T>(value)).cast::<c_void>(),
            size_of::<T>(),
            None,
        ) {
            Ok(()) => Ok(()),
            Err(why) => Err(crate::Error::MemoryError(format!(
                "failed to read memory: {why}"
            ))),
        }
    }
}

pub fn write<T>(handle: &Handle, address: usize, value: &mut T) -> Result<(), crate::Error> {
    unsafe {
        match WriteProcessMemory(
            handle.0,
            address as *const c_void,
            (ptr::from_mut::<T>(value)).cast::<c_void>(),
            size_of::<T>(),
            None,
        ) {
            Ok(()) => Ok(()),
            Err(why) => Err(crate::Error::MemoryError(format!(
                "failed to write memory: {why}"
            ))),
        }
    }
}
