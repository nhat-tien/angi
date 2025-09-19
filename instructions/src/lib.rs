
#[repr(u8)]
pub enum OpCode {
    LDC = 1, // Load Const
    LIM,     // Load immediately
    MTB,     // Make Table
    SAT,     // Set Attribute
    MTK,     // Make thunk
    RET,     // Return 
}

impl OpCode {
    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(op: u32) -> Option<OpCode> {
        match op {
            1 => Some(OpCode::LDC),
            _ => None
        }
    }
}

// pub const MAGIC_NUMBER_LENGTH: u8 = 32;
// pub const VERSION_LENGTH: u8 = 32;
// pub const CONST_OFFSET_DECLARE_LENGTH: u8 = 32;
// pub const CODE_OFFSET_DECLARE_LENGTH: u8 = 32;
// pub const INSTRUCTION_LENGTH: u8 = 32;

pub const OPCODE_BITS: u32 = 8;
pub const REG_BITS: u32 = 4;
pub const CONST_BITS: u32 = 20;

pub const OPCODE_OFFSET: u32 = 24;

pub const OPCODE_MASK: u32 = (1 << OPCODE_BITS) - 1; // 00000000 00000000 00000000 1111_1111
pub const REG_MASK: u32 = (1 << REG_BITS) - 1;   //     00000000 00000000 00000000 0000_1111
pub const CONST_MASK: u32 = (1 << CONST_BITS) - 1; //   00000000 0000_1111 11111111 11111111

pub fn encode_ldc(dr: u32, const_offset: u32) -> [u8; 4] {
    let bit = ((OpCode::LDC.to_u32() & OPCODE_MASK) << OPCODE_OFFSET)
    | ((dr & REG_MASK) << (CONST_BITS))
    | (const_offset & CONST_MASK);

    bit.to_be_bytes()
}

pub fn encode_mtb(dr: u32) -> [u8; 4] {
    let bit = ((OpCode::MTB.to_u32() & OPCODE_MASK) << OPCODE_OFFSET)
    | (dr & REG_MASK);

    bit.to_be_bytes()
}


pub fn encode_sat(dr: u32, key_r: u32, value_r: u32) -> [u8; 4] {
    let bit = ((OpCode::SAT.to_u32() & OPCODE_MASK) << OPCODE_OFFSET)
    | ((dr & REG_MASK) << (REG_BITS*2))
    | ((key_r & REG_MASK) << CONST_BITS)
    | (value_r & REG_MASK);

    bit.to_be_bytes()
}


pub fn encode_mtk(dr: u32, thunk_idx: u32) -> [u8; 4] {
    let bit = ((OpCode::MTK.to_u32() & OPCODE_MASK) << OPCODE_OFFSET)
    | ((dr & REG_MASK) << (CONST_BITS))
    | (thunk_idx & CONST_MASK);

    bit.to_be_bytes()
}

