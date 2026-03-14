use super::error::{CompilationError, ParseError};

pub fn handle_error(result : Result<Vec<u8>, CompilationError>) -> Result<Vec<u8>, CompilationError> {

    if let Err(e) = &result {
        match e {
            CompilationError::ParseError(parse_error) => handle_parse_error(parse_error),
            _ => todo!()
        }
    };
    result
}

fn handle_parse_error(parse_error: &ParseError) {
    println!("{:?}", parse_error);
}
