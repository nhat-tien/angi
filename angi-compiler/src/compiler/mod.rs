use bytecode::{load_global, BytecodeGen};
use error::{BytecodeGenerationError, CompilationError, ParseError};
use lexer::Lexer;
use parser::parse_with_engine;
use crate::{diagnostic::DiagnosticEngine, macro_function::MacroRegistry, type_checking};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod optimization;
pub mod parser;
pub mod token;
pub mod bytecode;

pub fn compile(src: &str, filename: &str) -> Result<Vec<u8>, CompilationError> {
    let mut engine = DiagnosticEngine::new();

    let mut lexer = Lexer::new(src.chars());
    let mut ast = match parse_with_engine(&mut lexer, &mut engine) {
        Some(ast) => ast,
        None => {
            engine.emit(src, filename);
            return Err(CompilationError::ParseError(ParseError {
                error: "Parsing failed with errors".to_string(),
                location: (0, 0),
            }));
        }
    };

    // let global_func = load_global();

    let mut macro_registry = MacroRegistry::new();
    match macro_registry.expand_expr_inplace(&mut ast) {
        Ok(_) => {},
        Err(_) => {
            engine.report(crate::diagnostic::Diagnostic {
                severity: crate::diagnostic::Severity::Error,
                message: "Somthing wrong in macro expand".into(),
                span: crate::diagnostic::Span { line: 1, column: 0 },
                span_len: 1,
                help: Some("Pray to god".into()),
                notes: vec![],
            });
            engine.emit(src, filename);
            return Err(CompilationError::MacroCheckingError);
        }
    };

    let mut bytecode_genaration = BytecodeGen::new();
          // .with_global_func(global_func);

    optimization::optimization(&mut ast);

    let byte = match bytecode_genaration.get_binary(ast) {
        Ok(byte) => byte,
        Err(err) => {
            let (message, help) = match &err {
                BytecodeGenerationError::UnexpectExpr { message } => {
                    (format!("Unexpected expression: {}", message), None)
                }
                BytecodeGenerationError::NotFoundVariable {} => {
                    ("Variable not found in current scope".to_string(),
                     Some("Make sure the variable is defined before use".to_string()))
                }
                BytecodeGenerationError::NotFoundFunction {} => {
                    ("Function not found".to_string(),
                     Some("Check function name or import the function".to_string()))
                }
            };
            engine.report(crate::diagnostic::Diagnostic {
                severity: crate::diagnostic::Severity::Error,
                message,
                span: crate::diagnostic::Span { line: 1, column: 0 },
                span_len: 1,
                help,
                notes: vec![],
            });
            engine.emit(src, filename);
            return Err(CompilationError::BytecodeGenerationError(err));
        }
    };

    engine.emit(src, filename);

    if engine.has_error() {
        Err(CompilationError::UnexpectedError)
    } else {
        Ok(byte)
    }
}

pub fn compile_and_type_checking(src: &str, filename: &str) -> Result<Vec<u8>, CompilationError> {
    let mut engine = DiagnosticEngine::new();

    let mut lexer = Lexer::new(src.chars());
    let mut ast = match parse_with_engine(&mut lexer, &mut engine) {
        Some(ast) => ast,
        None => {
            engine.emit(src, filename);
            return Err(CompilationError::ParseError(ParseError {
                error: "Parsing failed with errors".to_string(),
                location: (0, 0),
            }));
        }
    };

    let global_func = load_global();

    let mut bytecode_genaration = BytecodeGen::new()
          .with_global_func(global_func);

    optimization::optimization(&mut ast);

    type_checking::type_checking(&ast, &mut engine);

    let byte = match bytecode_genaration.get_binary(ast) {
        Ok(byte) => byte,
        Err(err) => {
            let (message, help) = match &err {
                BytecodeGenerationError::UnexpectExpr { message } => {
                    (format!("Unexpected expression: {}", message), None)
                }
                BytecodeGenerationError::NotFoundVariable {} => {
                    ("Variable not found in current scope".to_string(),
                     Some("Make sure the variable is defined before use".to_string()))
                }
                BytecodeGenerationError::NotFoundFunction {} => {
                    ("Function not found".to_string(),
                     Some("Check function name or import the function".to_string()))
                }
            };
            engine.report(crate::diagnostic::Diagnostic {
                severity: crate::diagnostic::Severity::Error,
                message,
                span: crate::diagnostic::Span { line: 1, column: 0 },
                span_len: 1,
                help,
                notes: vec![],
            });
            engine.emit(src, filename);
            return Err(CompilationError::BytecodeGenerationError(err));
        }
    };

    engine.emit(src, filename);

    if engine.has_error() {
        Err(CompilationError::UnexpectedError)
    } else {
        Ok(byte)
    }
}
