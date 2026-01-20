use std::{fs::File, io::{BufReader, Read}};

pub fn read_i64(r: &mut BufReader<File>) -> (String, i64){
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf).expect("Error in read byte");
    let num = i64::from_be_bytes(buf);
    (u8_slice_to_binary_string(&buf), num)
}

pub fn read_u32(r: &mut BufReader<File>) -> (String, u32){
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf).expect("Error in read byte");
    let num = u32::from_be_bytes(buf);
    (u8_slice_to_binary_string(&buf), num)
}

pub fn read_u8(r: &mut BufReader<File>) -> (String, u8){
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf).expect("Error in read byte");
    let num = u8::from_be_bytes(buf);
    (u8_slice_to_binary_string(&buf), num)
}

fn u8_slice_to_binary_string(bytes: &[u8]) -> String {
    let mut binary_strings = Vec::new();
    for byte in bytes {
        binary_strings.push(format!("{:08b}", byte));
    }
    binary_strings.join(" ")
}
