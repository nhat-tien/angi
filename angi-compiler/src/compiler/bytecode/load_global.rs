use std::{collections::HashMap, fs, path::Path};

use crate::compiler::{ast::Expr, lexer::Lexer, parser::parse};

use super::function::Function;

pub fn load_global() -> HashMap<String, Function> {

    let mut result: HashMap<String, Function> = HashMap::new();

    let root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(root).join("native_lib/global.ag");
    
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => panic!("Cannot open file {err:?}"),
    };

    let mut lexer = Lexer::new(content.chars());

    let ast = match parse(&mut lexer) {
        Ok(ast) => ast,
        Err(err) => panic!("Err in parse {err:?}"),
    };

    if let Expr::Table { fields } = ast {
        for (key, value) in fields {
            if let Some(function) = Function::from_epxr(value) {
                result.insert(key, function);
            }
        }
    }

    result
}
