
#[derive(Debug)]
pub struct LexicalError {
    error: String,
    location: (i32, i32),
}

