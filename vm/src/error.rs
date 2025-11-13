use std::fmt;

#[derive(Debug)]
pub enum VmError {
    ValueTypeMismatch { message: String },
    UnexpectedError { message: String },
    InstructionExecution { message: String },
    NotFoundFunction { message: String },
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VmError::ValueTypeMismatch { message } => write!(f, "[ValueTypeMismatch] {message}"),
            VmError::UnexpectedError { message } => write!(f, "[UnexpectedError] {message}"),
            VmError::InstructionExecution { message } => {
                write!(f, "[InstructionExecution] {message}")
            }
            VmError::NotFoundFunction { message } => {
                write!(f, "[NotFoundFunction] {message}")
            }
        }
    }
}
