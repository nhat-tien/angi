use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    Number,
    String,
    Boolean,
    Table(HashMap<String, Type>),
    List(Box<Type>),
    Function {
        params_type: Vec<Type>,
        return_type: Box<Type>
    },
    Any,
    TableDynamic,
    ListDynamic,
    FunctionDynamic,
    Unknown
}
