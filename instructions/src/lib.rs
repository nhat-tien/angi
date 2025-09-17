
pub enum OpCode {
    LDC = 1, // Load Const
    LIM,     // Load immediately
    MTB,     // Make Table
    SAT,     // Set Attribute
    RET,     // Return 
}

pub const MAGIC_NUMBER_LENGTH: u8 = 32;
pub const VERSION_LENGTH: u8 = 32;
pub const CONST_OFFSET_DECLARE_LENGTH: u8 = 32;
pub const CODE_OFFSET_DECLARE_LENGTH: u8 = 32;

pub const INSTRUCTION_LENGTH: u8 = 32;
