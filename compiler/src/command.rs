use std::fs::{set_permissions, File};
use std::io::{self, BufReader, Write};
use std::path::Path;
use std::{env, fs};

use vm::vm::VM;

use crate::code_gen::BytecodeGen;
use crate::debug;
use crate::lexer::Lexer;
use crate::parser::parse;
use crate::optimization::optimization;

static SERVER: &[u8] = include_bytes!(env!("RUNTIME_PATH"));

pub fn handle_command() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    };

    match args[1].as_str() {
        "run" => {
            let source_file_path = &args[2];

            let mut vm = match VM::new_from_file(source_file_path) {
                Ok(vm) => vm,
                Err(err) => panic!("{}", err.message)
            };

            match vm.eval_table("response.handler.response") {
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

            let mut ast = match parse(&mut lexer) {
                Ok(ast) => ast,
                Err(err) => panic!("Err in parse {err:?}"),
            };

            let mut bytecode_genaration = BytecodeGen::new();

            optimization(&mut ast);

            let content = bytecode_genaration.get_binary(ast);

            let mut file = match File::options()
                    .create(true)
                    .append(true)
                    .open(dist_file_path) {
                Ok(file) => file,
                Err(err) => panic!("Err in open file {}", err)
            };

            file.write_all(SERVER).expect("Fail to write file");
            file.write_all(&content).expect("Fail to write file");
            file.flush().expect("Fail to flush");
            
            let path = Path::new(&dist_file_path);
            make_file_executable(path).expect("Fail to make file executable");

        }
        "debug" => {
            let source_file_path = &args[2];

            let f = File::open(source_file_path).expect("Cant open file");

            let mut r = BufReader::new(f);

            debug::debug(&mut r);
        }
        "compile-file" => {
            let source_file_path = &args[2];
            let dist_file_path = &args[3];

            let content = match fs::read_to_string(source_file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}"),
            };

            let mut lexer = Lexer::new(content.chars());

            let mut ast = match parse(&mut lexer) {
                Ok(ast) => ast,
                Err(err) => panic!("Err in parse {err:?}"),
            };

            let mut bytecode_genaration = BytecodeGen::new();

            optimization(&mut ast);

            let content = bytecode_genaration.get_binary(ast);

            let mut file = match File::create(dist_file_path) {
                Ok(file) => file,
                Err(err) => panic!("Cannot create file {err:?}"),
            };
            let _ = file.write_all(&content);
        }
        _ => {
            println!("Command not exist");
        }
    };
}

fn make_file_executable(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        set_permissions(path, perms)?;
    }

    #[cfg(windows)]
    {
        if path.extension().is_none() {
            eprintln!("⚠️  On Windows, consider naming the file with `.exe`");
        }
    }

    Ok(())
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
