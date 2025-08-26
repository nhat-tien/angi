
#[allow(dead_code)]
#[derive(Debug,PartialEq)]
pub struct LexicalError {
    error: String,
    location: (i32, i32),
}

