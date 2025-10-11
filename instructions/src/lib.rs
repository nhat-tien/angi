pub enum Operand {
    RegAddr,
    ConstIdx,
}

pub const MAGIC_NUMBER: u32 = 0x414E4749; // "ANGI"
pub const OPCODE_BITS: u32 = 8;
pub const REG_BITS: u32 = 4;
pub const CONST_BITS: u32 = 20;

pub const OPCODE_OFFSET: u32 = 24;

pub const OPCODE_MASK: u32 = (1 << OPCODE_BITS) - 1; // 00000000 00000000 00000000 1111_1111
pub const REG_MASK: u32 = (1 << REG_BITS) - 1;   //     00000000 00000000 00000000 0000_1111
pub const CONST_MASK: u32 = (1 << CONST_BITS) - 1; //   00000000 0000_1111 11111111 11111111

macro_rules! define_opcodes {
    (
       $(
            $name:ident = {
                code = $val:expr,
                layout = [ $( $layout:ident ),* $(,)? ]
            }
        ),* $(,)?
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

            pub fn layout(&self) -> &'static [Operand] {
                match self {
                    $(
                        OpCode::$name => &[$( Operand::$layout ),*],
                    )*
                }
            }

            pub fn encode(&self, mut operand_params: Vec<u32>) -> [u8; 4] {
                let mut bit = ((self.to_u32() & OPCODE_MASK) << OPCODE_OFFSET);
                let mut offset_bit_from_right = 0;

                for operand in self.layout().iter().rev() {
                     match operand {
                         Operand::RegAddr => {
                             bit |= (operand_params.pop().unwrap() & REG_MASK) << offset_bit_from_right;
                             offset_bit_from_right += REG_BITS;
                         }
                         Operand::ConstIdx => {
                             bit |= (operand_params.pop().unwrap() & CONST_MASK) << offset_bit_from_right;
                             offset_bit_from_right += CONST_BITS;
                         }
                     }
                }

                bit.to_be_bytes()
            }

            pub fn decode(&self, byte: u32) -> Vec<u32> {
                let mut offset_bit_from_right = 0;
                let mut result = vec![];

                for operand in self.layout().iter().rev() {
                     match operand {
                         Operand::RegAddr => {
                             let number = ((byte >> offset_bit_from_right) & REG_MASK);
                             result.insert(0, number);
                             offset_bit_from_right += REG_BITS;
                         }
                         Operand::ConstIdx => {
                             let number = ((byte >> offset_bit_from_right) & CONST_MASK);
                             result.insert(0, number);
                             offset_bit_from_right += CONST_BITS;
                         }
                     }
                }

                result
            }
        }

    };
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

