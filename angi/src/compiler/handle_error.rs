use super::error::CompilationError;
use crate::diagnostic::{Diagnostic, DiagnosticEngine, Severity, Span};

/// Handle compilation errors using the DiagnosticEngine
pub fn handle_error(
    result: Result<Vec<u8>, CompilationError>,
    source: &str,
    filename: &str,
) -> Result<Vec<u8>, CompilationError> {
    match result {
        Ok(bytecode) => Ok(bytecode),
        Err(e) => {
            let mut engine = DiagnosticEngine::new();
            convert_error(&e, &mut engine);
            engine.emit(source, filename);
            Err(e)
        }
    }
}

/// Convert a CompilationError into one or more diagnostics
fn convert_error(error: &CompilationError, engine: &mut DiagnosticEngine) {
    match error {
        CompilationError::IOError { message } => {
            // IO errors don't have source spans, so we report at the beginning
            engine.report(Diagnostic {
                severity: Severity::Error,
                message: format!("I/O error: {}", message),
                span: Span { line: 1, column: 0 },
                span_len: 1,
                help: None,
                notes: vec![],
            });
        }
        CompilationError::ParseError(parse_error) => {
            let line = parse_error.location.0 as usize;
            let column = parse_error.location.1 as usize;

            engine.report(Diagnostic {
                severity: Severity::Error,
                message: parse_error.error.clone(),
                span: Span { line, column },
                span_len: 1,
                help: None,
                notes: vec![],
            });
        }
        CompilationError::BytecodeGenerationError(bytecode_error) => {
            match bytecode_error {
                super::error::BytecodeGenerationError::UnexpectExpr { message } => {
                    engine.report(Diagnostic {
                        severity: Severity::Error,
                        message: format!("Unexpected expression: {}", message),
                        span: Span { line: 1, column: 0 },
                        span_len: 1,
                        help: Some("Check the syntax of your expression".to_string()),
                        notes: vec![],
                    });
                }
                super::error::BytecodeGenerationError::NotFoundVariable {} => {
                    engine.report(Diagnostic {
                        severity: Severity::Error,
                        message: "Variable not found in current scope".to_string(),
                        span: Span { line: 1, column: 0 },
                        span_len: 1,
                        help: Some("Make sure the variable is defined before use".to_string()),
                        notes: vec![],
                    });
                }
                super::error::BytecodeGenerationError::NotFoundFunction {} => {
                    engine.report(Diagnostic {
                        severity: Severity::Error,
                        message: "Function not found".to_string(),
                        span: Span { line: 1, column: 0 },
                        span_len: 1,
                        help: Some("Check function name or import the function".to_string()),
                        notes: vec![],
                    });
                }
            }
        }
        CompilationError::ArchiveError => {
            engine.report(Diagnostic {
                severity: Severity::Error,
                message: "Failed to archive bytecode".to_string(),
                span: Span { line: 1, column: 0 },
                span_len: 1,
                help: None,
                notes: vec![],
            });
        }
    }
}
