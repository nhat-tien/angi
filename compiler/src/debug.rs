use std::{fs::File, io::{BufReader, Read}};
const PADDING: usize = 16;

pub fn debug(r: &mut BufReader<File>) {

    let (magic_code,_) = read_u32(r);
    println!("{:<PADDING$}{} : ANGI", "MAGIC CODE", magic_code);

    let ( version, _) = read_u32(r);
    println!("{:<PADDING$}{}", "VERSION", version);

    let ( const_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "CONST OFFSET", const_offset);

    let ( const_size, const_size_num) = read_u32(r);
    println!("{:<PADDING$}{}", "CONST SIZE", const_size);

    let ( code_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "CODE OFFSET", code_offset);

    let ( code_size, code_size_num) = read_u32(r);
    println!("{:<PADDING$}{}", "CODE SIZE", code_size);

    read_const(r, const_size_num);
    read_instruction(r, code_size_num);
}

fn read_const(r: &mut BufReader<File>, mut const_size: u32) {
    while const_size > 0 {
        let ( const_type, const_type_u32) = read_u8(r);
        println!("{:<PADDING$}{}", "CONST TYPE", const_type);
        
        match const_type_u32 {
            0_u8 => { 
                let (number_in_b, num) = read_i64(r);
                println!("{:<PADDING$}{} : {}", "INT", number_in_b, num);
            },
            // STRING
            1_u8 => {
                let ( str_len_in_b, mut str_len) = read_u32(r);
                println!("{:<PADDING$}{}", "STRING LEN", str_len_in_b);
                print!("{:<PADDING$}", "STRING");
                while str_len > 0 {
                    let (char, _) = read_u8(r);
                    print!("{} ", char);
                    str_len -= 1;
                }
                println!();
            },
            _ => panic!("Not implent const_type")
        }

        const_size -= 1;
    }
}

fn read_instruction(r: &mut BufReader<File>, mut code_size: u32) {
    while code_size > 0 {
        let ( ins , _) = read_u32(r);
        println!("{:<PADDING$}{}", "INS", ins);
       code_size -= 1; 
    }
}

fn read_i64(r: &mut BufReader<File>) -> (String, i64){
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf).expect("Error in read byte");
    let num = i64::from_be_bytes(buf);
    (u8_slice_to_binary_string(&buf), num)
}

fn read_u32(r: &mut BufReader<File>) -> (String, u32){
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf).expect("Error in read byte");
    let num = u32::from_be_bytes(buf);
    (u8_slice_to_binary_string(&buf), num)
}

fn read_u8(r: &mut BufReader<File>) -> (String, u8){
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
