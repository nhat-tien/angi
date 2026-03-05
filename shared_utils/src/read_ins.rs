use instructions::extract_opcode;
use crate::{log::Log, log::LogLevel::{ERROR, DEBUG}, read_from_buf_reader::u8_slice_to_binary_string};

const PADDING: usize = 16;

pub fn read_ins(ins: u32) {
    let str_ins = u8_slice_to_binary_string(&ins.to_be_bytes());
    let opcode = extract_opcode(ins);
    match opcode {
        None => Log::write(ERROR, &format!("(in read_ins) Cant get opcode {}", ins)),
        Some(op) => {
            let param = op.decode(ins);
            let str_params = param.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Log::write(DEBUG, &format!("{:<PADDING$}{}: {:?} {}", "INS", str_ins, op, str_params));
        }
    }
}
