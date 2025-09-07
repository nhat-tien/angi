use std::fs::File;
use std::io::Write;
use std::{env, fs};
use crate::code_gen::BytecodeGen;
use crate::parser::parse;
use crate::lexer::Lexer;


pub fn handle_command() {

    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    };

    match args[1].as_str() {
        "run" => {
            let file_path = &args[2];

            let content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}")
            };

            let mut lexer = Lexer::new(content.chars());
            println!("{:?}",parse(&mut lexer));
        },
        "compile" => {
            let source_file_path = &args[2];
            let dist_file_path = &args[3];

            let content = match fs::read_to_string(source_file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}")
            };

            let mut lexer = Lexer::new(content.chars());

            let ast = match parse(&mut lexer) {
                Ok(ast) => ast,
                Err(err) => panic!("Err in parse {err:?}")
            };

            let bytecode_genaration = BytecodeGen::new();

            let content = bytecode_genaration.get_binary(&ast);

            let mut file = match File::create(dist_file_path) {
                Ok(file) => file,
                Err(err) => panic!("Cannot create file {err:?}")
            };
            let _ = file.write_all(&content);
        },
        _ => {
            println!("Command not exist");
        }
    };
}


fn print_help() {
    println!(r#"
angi help page
USAGE: 
COMMAND:
"#);
}
