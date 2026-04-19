
type LocationError = (u32, u32); // (line, column)

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: String,
    pub location: LocationError
}


#[allow(dead_code)]
#[derive(Debug,PartialEq)]
pub struct LexicalError {
    pub error: String,
    pub location: (u32, u32),
}


#[derive(Debug)]
pub enum CompilationError {
    IOError {
        message: String
    },
    ParseError(ParseError),
    BytecodeGenerationError(BytecodeGenerationError),
    MacroCheckingError,
    ArchiveError,
    UnexpectedError
}

#[derive(Debug)]
pub enum BytecodeGenerationError {
    UnexpectExpr {
        message: String
    },
    NotFoundVariable {},
    NotFoundFunction {}
}

