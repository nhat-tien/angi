use std::fs;

use clap::{Parser, Subcommand};

use crate::ast_parser::parser::parse;
use crate::lexing::lexer::Lexer;

/// A toy compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run program
    Run { file_path: Option<String>}
}


pub fn handle_command() {

    let args = CliArgs::parse();

     match &args.command {
        Some(Commands::Run { file_path }) => {
            let file_path = file_path.as_ref().expect("File path missing");

            let content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(err) => panic!("Cannot open file {err:?}")
            };

            let mut lexer = Lexer::new(content.chars());
            println!("{:?}",parse(&mut lexer));
        }
        None => {
            println!("Command not exist");
        }
    }
}
