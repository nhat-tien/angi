use std::fmt;

use archive::ExtractorError;

#[derive(Debug)]
pub enum VmError {
    ValueTypeMismatch { message: String },
    UnexpectedError { message: String },
    InstructionExecution { message: String },
    NotFoundFunction { message: String },
    ErrorInGetOpcode {
        message: String,
        ins: u32,
        cursor: usize
    },
    ExtractorError { err: ExtractorError }
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
            VmError::ErrorInGetOpcode { message, ins, cursor } => {
                write!(f, "[ErrorInGetOpcode] {message}, ins: {ins}, cursor: {cursor}")
            }
            _e => {
                write!(f, "Error not implement display yet")
            }
        }
    }
}
