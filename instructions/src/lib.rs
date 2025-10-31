#[macro_use]
mod macros;

pub const VERSION: u32 = 0x00000001; // "0.0.1"
pub const METADATA_BYTES: u32 = 32;
pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const OPCODE_BITS: u32 = 8;
pub const REG_BITS: u32 = 4;
pub const CONST_BITS: u32 = 20;

pub const OPCODE_OFFSET: u32 = 24;

pub const OPCODE_MASK: u32 = (1 << OPCODE_BITS) - 1; // 00000000 00000000 00000000 1111_1111
pub const REG_MASK: u32 = (1 << REG_BITS) - 1;   //     00000000 00000000 00000000 0000_1111
pub const CONST_MASK: u32 = (1 << CONST_BITS) - 1; //   00000000 0000_1111 11111111 11111111

pub enum Operand {
    RegAddr,
    ConstIdx,
}

define_opcodes! {
    LDC = { code = 1, layout = [RegAddr,ConstIdx] },            // Load Const
    LIM = { code = 2, layout = [] },                            // Load immediately
    MTB = { code = 3, layout = [RegAddr] },                     // Make Table
    SAT = { code = 4, layout = [RegAddr,RegAddr,RegAddr] },     // Set Attribute
    MTK = { code = 5, layout = [RegAddr,ConstIdx] },            // Make thunk
    RET = { code = 6, layout = [RegAddr] },                     // Return 
    MLI = { code = 7, layout = [RegAddr] },                     // Make List
    ADL = { code = 8, layout = [RegAddr,RegAddr] }              // Add to List
}

pub fn extract_opcode(byte: u32) -> Option<OpCode> {
    let bit = byte >> OPCODE_OFFSET;
    OpCode::from_u8(bit as u8)
}
