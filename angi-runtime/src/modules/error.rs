#[derive(Debug)]
pub enum ForeignFnError {
    MismatchParams,
    Unexpected { message: String }
}

