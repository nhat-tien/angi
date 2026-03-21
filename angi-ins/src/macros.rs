#[macro_export]
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

