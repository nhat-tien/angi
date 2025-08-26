mod lexing;
mod command;
mod ast_parser;

use command::handle_command;

fn main() {
    handle_command();
}
