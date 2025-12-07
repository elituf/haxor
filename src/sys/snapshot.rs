use crate::error::Error;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, MODULEENTRY32W, Module32FirstW, Module32NextW, PROCESSENTRY32W,
    Process32FirstW, Process32NextW, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};

#[derive(Debug)]
pub struct ProcessSnapshot {
    pub id: u32,
    pub name: String,
}

impl ProcessSnapshot {
    pub fn get_processes() -> Result<Vec<Self>, Error> {
        let snapshot =
            unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.map_err(|why| {
                Error::CreateSnapshotError(format!("failed to create process snapshot: {why}"))
            })?;
        let mut process_entry_32_w = PROCESSENTRY32W {
            dwSize: u32::try_from(size_of::<PROCESSENTRY32W>())?,
            ..Default::default()
        };
        unsafe { Process32FirstW(snapshot, &raw mut process_entry_32_w) }.map_err(|why| {
            Error::CreateSnapshotError(format!("failed to get first process from snapshot: {why}"))
        })?;
        let mut processes = Vec::new();
        loop {
            let name = String::from_utf16_lossy(&process_entry_32_w.szExeFile)
                .trim_end_matches('\0')
                .to_string();
            let process = Self {
                id: process_entry_32_w.th32ProcessID,
                name,
            };
            processes.push(process);
            if unsafe { Process32NextW(snapshot, &raw mut process_entry_32_w) }.is_err() {
                break;
            }
        }
        Ok(processes)
    }
}

#[derive(Clone, Debug)]
pub struct ModuleSnapshot {
    pub name: String,
    pub path: String,
    pub base_address: usize,
    pub base_size: usize,
}

impl ModuleSnapshot {
    pub fn get_modules(pid: u32) -> Result<Vec<Self>, Error> {
        let snapshot =
            unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) }
                .map_err(|why| {
                    Error::CreateSnapshotError(format!("failed to create module snapshot: {why}"))
                })?;
        let mut module_entry_32_w = MODULEENTRY32W {
            dwSize: u32::try_from(size_of::<MODULEENTRY32W>())?,
            ..Default::default()
        };
        unsafe { Module32FirstW(snapshot, &raw mut module_entry_32_w) }.map_err(|why| {
            Error::CreateSnapshotError(format!("failed to get first module from snapshot: {why}"))
        })?;
        let mut modules = Vec::new();
        loop {
            let name = String::from_utf16_lossy(&module_entry_32_w.szModule)
                .trim_end_matches('\0')
                .to_string();
            let path = String::from_utf16_lossy(&module_entry_32_w.szExePath)
                .trim_end_matches('\0')
                .to_string();
            let module = Self {
                name,
                path,
                base_address: module_entry_32_w.modBaseAddr as usize,
                base_size: module_entry_32_w.modBaseSize as usize,
            };
            modules.push(module);
            if unsafe { Module32NextW(snapshot, &raw mut module_entry_32_w).is_err() } {
                break;
            }
        }
        Ok(modules)
    }
}
