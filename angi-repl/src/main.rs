use angi::compiler::compile;
use angi_runtime::vm::VM;
use std::{
    env, fs,
    io::{self, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let source_file_path = &args[1];

    let source = fs::read_to_string(source_file_path).unwrap();

    let bytecode = compile(&source, source_file_path).unwrap();

    let mut vm = VM::new_from_bytes(bytecode).unwrap();

    loop {
        print!("angi> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "exit" => break,
            "cls" => {
                print!("\x1B[2J\x1B[H");
                continue;
            }
            "help" => {
                println!(r#"
COMMAND

exit         Exit REPL
cls          Clear screen
help         Print help page
                "#);
                continue;
            },
            i => {
                match run(&mut vm, i) {
                    Ok(result) => println!("{:?}", result),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }


    }
}

fn run(vm: &mut VM, input: &str) -> Result<String, String> {
    let mut command = input.split_whitespace();

    match command.next() {
        Some("eval") => {
            let result = vm.eval_value(command.next().unwrap());
            match result {
                Ok(val) => Ok(format!("{:?}", val)),
                Err(vm_err) => Err(vm_err.to_string()),
            }
        }
        _ => Ok(input.to_string()),
    }
}
