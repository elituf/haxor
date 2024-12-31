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
    pub base_address: usize,
    /// the module base size (modBaseSize)
    pub base_size: usize,
}
