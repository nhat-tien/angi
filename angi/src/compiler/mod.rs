use bytecode::{load_global, BytecodeGen};
use error::CompilationError;
use lexer::Lexer;
use parser::parse;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod optimization;
pub mod parser;
pub mod token;
pub mod bytecode;
pub mod handle_error;

pub fn compile_with_handle_error(src: String) -> Result<Vec<u8>, CompilationError> {
    handle_error::handle_error(compile(src))
}

pub fn compile(src: String) -> Result<Vec<u8>, CompilationError> {

    let mut lexer = Lexer::new(src.chars());

    let mut ast = parse(&mut lexer).map_err(|err| { CompilationError::ParseError(err) })?;

    let global_func = load_global();

    let mut bytecode_genaration = BytecodeGen::new()
          .with_global_func(global_func);

    optimization::optimization(&mut ast);

    let byte = bytecode_genaration.get_binary(ast).map_err(|err| {
        CompilationError::BytecodeGenerationError(err)
    })?;

    Ok(byte)
}

