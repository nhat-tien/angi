mod lexing;
mod command;
mod parser;

use command::handle_command;

fn main() {
    handle_command();
}
