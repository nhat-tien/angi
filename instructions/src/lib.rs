macro_rules! define_opcodes {
    (
        $( $name:ident = $val:expr, )*
    ) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum OpCode {
            $(
                $name = $val,
            )*
        }

        impl OpCode {
            pub fn from_u8(n: u8) -> Option<Self> {
                match n {
                    $(
                        $val => Some(OpCode::$name),
                    )*
                    _ => None,
                }
            }

            pub fn to_u8(self) -> u8 {
                self as u8
            }

            pub fn to_u32(self) -> u32 {
                self as u32
            }
        }

    };
}


define_opcodes! {
    LDC = 1,     // Load Const
    LIM = 2,     // Load immediately
    MTB = 3,     // Make Table
    SAT = 4,     // Set Attribute
    MTK = 5,     // Make thunk
    RET = 6,     // Return 
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
    | ((key_r & REG_MASK) << REG_BITS)
    | (value_r & REG_MASK);

    bit.to_be_bytes()
}


pub fn encode_mtk(dr: u32, thunk_idx: u32) -> [u8; 4] {
    let bit = ((OpCode::MTK.to_u32() & OPCODE_MASK) << OPCODE_OFFSET)
    | ((dr & REG_MASK) << (CONST_BITS))
    | (thunk_idx & CONST_MASK);

    bit.to_be_bytes()
}

pub fn encode_ret(dr: u32) -> [u8; 4] {
    let bit = ((OpCode::RET.to_u32() & OPCODE_MASK) << OPCODE_OFFSET)
    | (dr & REG_MASK);

    bit.to_be_bytes()
}

pub fn extract_opcode(byte: u32) -> Option<OpCode> {
    let bit = byte >> OPCODE_OFFSET;
    OpCode::from_u8(bit as u8)
}

