use super::tree::Tree;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Table(Box<Tree<Value>>),
    Thunk(u32),
    None
}

impl Clone for Value {

    fn clone(&self) -> Self {
        match self {
            Self::Int(arg0) => Self::Int(*arg0),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Table(arg0) => Self::Table(arg0.clone()),
            Self::Thunk(arg0) => Self::Thunk(*arg0),
            Self::None => Self::None,
        }
    }
} 


