use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

#[derive(Debug, Default)]
pub struct Process {
    pub id: u32,
    pub name: String,
}

impl Process {
    pub fn get_processes() -> Result<Vec<Self>, crate::Error> {
        let snapshot = unsafe {
            match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
                Ok(snapshot) => snapshot,
                Err(why) => {
                    return Err(crate::Error::SnapshotError(format!(
                        "couldn't create snapshot: {why}"
                    )))
                }
            }
        };
        let mut process_entry_32_w = PROCESSENTRY32W {
            dwSize: u32::try_from(size_of::<PROCESSENTRY32W>())?,
            ..Default::default()
        };
        unsafe {
            match Process32FirstW(snapshot, &mut process_entry_32_w) {
                Ok(()) => {}
                Err(why) => {
                    return Err(crate::Error::SnapshotError(format!(
                        "couldn't get first process from snapshot: {why}"
                    )))
                }
            };
        }
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
            unsafe {
                if Process32NextW(snapshot, &mut process_entry_32_w).is_err() {
                    break;
                };
            }
        }
        Ok(processes)
    }
}
