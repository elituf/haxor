#[derive(Debug, Default)]
pub struct Module {
    pub process_id: u32,
    pub name: String,
    pub path: String,
    pub base_address: usize,
    pub base_size: usize,
}
