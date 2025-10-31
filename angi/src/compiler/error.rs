
type LocationError = (u32, u32); // (line, column)

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: String,
    pub location: LocationError
}


#[allow(dead_code)]
#[derive(Debug,PartialEq)]
pub struct LexicalError {
    error: String,
    location: (i32, i32),
}


pub enum CompilationError {
    IOError {
        message: String
    },
    ParseError(ParseError)
}

