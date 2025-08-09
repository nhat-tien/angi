use clap::{Parser, Subcommand};

use crate::parser;

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
            parser::parse(file_path);
        }
        None => {
            println!("Default subcommand");
        }
    }
}
