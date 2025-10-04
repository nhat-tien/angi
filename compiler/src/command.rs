use std::fs::File;
use std::io::{BufReader, Write};
use std::{env, fs};

use vm::error::RuntimeError;
use vm::vm::VM;

use crate::code_gen::BytecodeGen;
use crate::debug;
use crate::lexer::Lexer;
use crate::parser::parse;

pub fn handle_command() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    };

    match args[1].as_str() {
        "run" => {
            // let file_path = &args[2];
            //
            // let content = match fs::read_to_string(file_path) {
            // Ok(content) => content,
            // Err(err) => panic!("Cannot open file {err:?}")
            // };
            //
            // let mut lexer = Lexer::new(content.chars());
            //
            // let ast = match parse(&mut lexer) {
            // Ok(ast) => ast,
            // Err(err) => panic!("Err in parse {err:?}")
            // };
            //
            // let mut bytecode_genaration = BytecodeGen::new();
            //
            // bytecode_genaration.visit_expr(&ast);
            //

            let source_file_path = &args[2];

            let f = fs::read(source_file_path).expect("Cant open file");

            let mut vm = VM::new();

            if let Err(RuntimeError { message }) = vm.load(f) {
                panic!("{}", message);
            };

            match vm.eval() {
                Ok(value) => println!("VM: {:?}", value),
                Err(err) => panic!("{:?}", err.message),
            }
        }
        "compile" => {
            let source_file_path = &args[2];
            let dist_file_path = &args[3];

            let content = match fs::read_to_string(source_file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}"),
            };

            let mut lexer = Lexer::new(content.chars());

            let ast = match parse(&mut lexer) {
                Ok(ast) => ast,
                Err(err) => panic!("Err in parse {err:?}"),
            };

            let mut bytecode_genaration = BytecodeGen::new();

            let content = bytecode_genaration.get_binary(ast);

            let mut file = match File::create(dist_file_path) {
                Ok(file) => file,
                Err(err) => panic!("Cannot create file {err:?}"),
            };
            let _ = file.write_all(&content);
        }
        "debug" => {
            let source_file_path = &args[2];

            let f = File::open(source_file_path).expect("Cant open file");

            let mut r = BufReader::new(f);

            debug::debug(&mut r);
        }
        _ => {
            println!("Command not exist");
        }
    };
}

fn print_help() {
    println!(
        r#"
    angi help page
    USAGE:
    COMMAND:
    "#
    );
}
