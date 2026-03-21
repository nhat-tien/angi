use std::collections::HashMap;

use super::core_type::Type;

pub fn get_root_schema() -> Type  {
    Type::Table(HashMap::from([
        ("port".to_string(), Type::Number),
        ("routes".to_string(), Type::List(
            Box::new(Type::Table(HashMap::from([
                ("path".to_string(), Type::String),
                ("handler".to_string(), Type::Function {
                    params_type: vec![],
                    return_type: Box::new(Type::TableDynamic),
                }),
            ]))),
        )),
    ]))
}
