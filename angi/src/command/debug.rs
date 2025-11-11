use std::fs::{self, File};
use std::io::{BufReader, Read, Write};

use instructions::extract_opcode;
use vm::value::{List, Table};
use vm::vm::VM;

use crate::compiler::bytecode::load_global;
use crate::compiler::{bytecode::BytecodeGen, lexer::Lexer, parser::parse};
use crate::compiler::optimization::optimization;
const PADDING: usize = 16;


pub fn index(args: &[String]) {
    match args[2].as_str() {
        "lex" => {
            let source_file_path = &args[3];

            let content = match fs::read_to_string(source_file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}"),
            };

            for lex in Lexer::new(content.chars()) {
                println!("{:?}", lex);
            }
        }
        "ast" => {
            let source_file_path = &args[3];

            let content = match fs::read_to_string(source_file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}"),
            };

            let mut lexer = Lexer::new(content.chars());

            let ast = match parse(&mut lexer) {
                Ok(ast) => ast,
                Err(err) => panic!("Err in parse {err:?}"),
            };

            print!("{:?}", ast);
        }
        "run" => {
            let source_file_path = &args[3];

            let bytes = fs::read(source_file_path).expect("Cant open file");

            let mut vm = match VM::new_from_bytes(bytes) {
                Ok(vm) => vm,
                Err(err) => panic!("{}", err)
            };

            let list_routes = vm.eval::<List<Table>>("routes").expect("ckdsn");
            let list_routes_iter = list_routes.iter().unwrap();

            for route in list_routes_iter {
                let path = route.get::<String>("path").unwrap();
                let message = route.get::<String>("handler").unwrap();
                println!("path {path}");
                println!("message {message}");
            };
        }
        "writebc" => {
            let source_file_path = &args[3];
            let dist_file_path = &args[4];

            let content = match fs::read_to_string(source_file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}"),
            };

            let mut lexer = Lexer::new(content.chars());

            let mut ast = match parse(&mut lexer) {
                Ok(ast) => ast,
                Err(err) => panic!("Err in parse {err:?}"),
            };

            let global_func = load_global();

            let mut bytecode_genaration = BytecodeGen::new()
                  .with_global_func(global_func);

            optimization(&mut ast);

            let content = bytecode_genaration.get_binary(ast);

            let mut file = match File::create(dist_file_path) {
                Ok(file) => file,
                Err(err) => panic!("Cannot create file {err:?}"),
            };
            let _ = file.write_all(&content);
        }
        "readbc" => {
            let source_file_path = &args[3];
            let f = File::open(source_file_path).expect("Cant open file");
            let mut r = BufReader::new(f);
            print_bytecode(&mut r);
        }
        _ => {
            println!("Command not exist");
        }
    }
}

pub fn print_bytecode(r: &mut BufReader<File>) {

    let (magic_code,_) = read_u32(r);
    println!("{:<PADDING$}{} : ANGI", "MAGIC CODE", magic_code);

    let ( version, _) = read_u32(r);
    println!("{:<PADDING$}{}", "VERSION", version);

    let ( const_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "CONST OFFSET", const_offset);

    let ( const_size, const_size_num) = read_u32(r);
    println!("{:<PADDING$}{}", "CONST SIZE", const_size);

    let ( thunk_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "THUNK OFFSET", thunk_offset);

    let ( thunk_size, thunk_size_num) = read_u32(r);
    println!("{:<PADDING$}{}", "THUNK SIZE", thunk_size);

    let ( function_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "FUNC OFFSET", function_offset);

    let ( function_size, function_size_num) = read_u32(r);
    println!("{:<PADDING$}{}", "FUNC SIZE", function_size);

    let ( global_func_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "G-FUNC OFFSET", global_func_offset);

    let ( global_func_size, global_func_size_num) = read_u32(r);
    println!("{:<PADDING$}{}: {}", "G-FUNC SIZE", global_func_size, global_func_size_num);

    let ( code_offset, _) = read_u32(r);
    println!("{:<PADDING$}{}", "CODE OFFSET", code_offset);

    let ( code_size, code_size_num) = read_u32(r);
    println!("{:<PADDING$}{}", "CODE SIZE", code_size);

    read_const(r, const_size_num);
    read_thunk(r, thunk_size_num);
    read_function(r, function_size_num);
    read_global_function(r, global_func_size_num);
    read_instruction(r, code_size_num);

    let ( total_byte, total_byte_num) = read_u32(r);
    println!("{:<PADDING$}{}: {}", "TOTAL BYTE", total_byte, total_byte_num);
}

fn read_const(r: &mut BufReader<File>, mut const_size: u32) {
    while const_size > 0 {
        let ( const_type, const_type_u32) = read_u8(r);
        print!("{:<PADDING$}{}", "CONST TYPE", const_type);
        
        match const_type_u32 {
            0_u8 => { 
                println!(": int");
                let (number_in_b, num) = read_i64(r);
                println!("{:<PADDING$}{} : {}", "INT", number_in_b, num);
            },
            // STRING
            1_u8 => {
                println!(": string");
                let ( str_len_in_b, mut str_len) = read_u32(r);
                let mut string = String::from("");
                println!("{:<PADDING$}{}: {}", "STRING LEN", str_len_in_b, str_len);
                print!("{:<PADDING$}", "STRING");
                while str_len > 0 {
                    let (char, char_u8) = read_u8(r);
                    print!("{} ", char);
                    str_len -= 1;
                    string.push(char_u8 as char);
                }
                println!(": {}", string);
            },
            _ => panic!("Not implent const_type")
        }

        const_size -= 1;
    }
}

fn read_thunk(r: &mut BufReader<File>, mut thunk_size: u32) {
    while thunk_size > 0 {
        let ( thunk, thunk_num) = read_u32(r);
        println!("{:<PADDING$}{}: {}", "THUNK", thunk, thunk_num);
        thunk_size -= 1;
    }
}

fn read_function(r: &mut BufReader<File>, mut function_size: u32) {
    while function_size > 0 {
        let ( nargs, nargs_num) = read_u32(r);
        println!("{:<PADDING$}{}: {}", "FUNC NARG", nargs, nargs_num);
        let ( body_addr, body_addr_num) = read_u32(r);
        println!("{:<PADDING$}{}: {}", "FUNC BODY", body_addr, body_addr_num);
        function_size-= 1;
    }
}

fn read_global_function(r: &mut BufReader<File>, mut global_function_size: u32) {
    while global_function_size > 0 {
        let ( g_func, _) = read_u32(r);
        println!("{:<PADDING$}{}", "G-FUNC NAME", g_func);
        let ( g_func_idx, _) = read_u32(r);
        println!("{:<PADDING$}{}", "G-FUNC IDX", g_func_idx);
        global_function_size -= 1;
    }
}

fn read_instruction(r: &mut BufReader<File>, mut code_size: u32) {
    while code_size > 0 {
        let ( ins , number ) = read_u32(r);
        let opcode = extract_opcode(number).unwrap();
        println!("{:<PADDING$}{}: {:?}", "INS", ins, opcode);
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
