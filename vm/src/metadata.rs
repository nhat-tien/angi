#[derive(Copy, Clone, Default, Debug)]
pub struct MetaData {
    pub magic_code: u32,
    pub version: u32,
    pub const_offset: u32,
    pub const_size: u32,
    pub thunk_offset: u32,
    pub thunk_size: u32,
    pub code_offset: u32,
    pub code_size: u32
}
