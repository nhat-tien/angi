use crate::register::Register;

struct MetaData {
    magic_code: u32,
    version: u32,
    const_offset: u32,
    const_size: u32,
    thunk_offset: u32,
    thunk_size: u32,
    code_offset: u32,
    code_size: u32
}

pub struct VM {
    registers: Register,
    bytes: Vec<u8>,
    cursor: usize
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: Register::new(),
            bytes: vec![],
            cursor: 0
        }
    }

    pub fn load(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }

    pub fn eval(&self, attribute: String) {
        let attr_arr = attribute.split('.');
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        if let Some(slice) = self.bytes.get(self.cursor..self.cursor+8).and_then(|s| s.try_into().ok()) {
            let arr_of_bytes: [u8; 8] = slice;
            self.cursor += 8;
            Some(i64::from_be_bytes(arr_of_bytes))
        } else {
            None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if let Some(slice) = self.bytes.get(self.cursor..self.cursor+8).and_then(|s| s.try_into().ok()) {
            let arr_of_bytes: [u8; 4] = slice;
            self.cursor += 4;
            Some(u32::from_be_bytes(arr_of_bytes))
        } else {
            None
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if let Some(slice) = self.bytes.get(self.cursor..self.cursor+8).and_then(|s| s.try_into().ok()) {
            let arr_of_bytes: [u8; 1] = slice;
            self.cursor += 4;
            Some(u8::from_be_bytes(arr_of_bytes))
        } else {
            None
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
