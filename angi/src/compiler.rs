use code_gen::BytecodeGen;
use error::CompilationError;
use lexer::Lexer;
use parser::parse;

pub mod ast;
pub mod code_gen;
pub mod error;
pub mod lexer;
pub mod optimization;
pub mod parser;
pub mod token;

pub fn compile(src: String) -> Result<Vec<u8>, CompilationError> {

    let mut lexer = Lexer::new(src.chars());

    let mut ast = parse(&mut lexer).map_err(|err| { CompilationError::ParseError(err) })?;

    let mut bytecode_genaration = BytecodeGen::new();

    optimization::optimization(&mut ast);

    Ok(bytecode_genaration.get_binary(ast))
}
