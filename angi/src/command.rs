mod debug;
mod build;

use std::env;

pub fn handle_command() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        return;
    };

    match args[1].as_str() {
        "build" => {
            let _ = build::index(&args);
        }
        "debug" => {
            debug::index(&args);
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

