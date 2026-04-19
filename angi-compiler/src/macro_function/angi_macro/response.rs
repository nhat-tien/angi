use std::collections::HashMap;

use crate::{
    compiler::ast::Expr,
    macro_function::{MacroContext, error::MacroError},
};

#[allow(unused_variables)]
pub fn html_template(params: Vec<Expr>, ctx: &mut MacroContext) -> Result<Expr, MacroError> {
    if let Some(Expr::LiteralString(str)) = params.first() {
        Ok(Expr::Table {
            fields: HashMap::from([
                (
                    String::from("type"),
                    Expr::LiteralString(String::from("htmlTemplate")),
                ),
                (String::from("path"), Expr::LiteralString(str.clone())),
            ]),
        })
    } else {
        Err(MacroError::MismatchParams)
    }
}

#[allow(unused_variables)]
pub fn html(params: Vec<Expr>, ctx: &mut MacroContext) -> Result<Expr, MacroError> {
    if let Some(Expr::LiteralString(str)) = params.first() {
        Ok(Expr::Table {
            fields: HashMap::from([
                (
                    String::from("type"),
                    Expr::LiteralString(String::from("html")),
                ),
                (String::from("html"), Expr::LiteralString(str.clone())),
            ]),
        })
    } else {
        Err(MacroError::MismatchParams)
    }
}

#[allow(unused_variables)]
pub fn json(params: Vec<Expr>, ctx: &mut MacroContext) -> Result<Expr, MacroError> {
    if let Some(Expr::LiteralString(str)) = params.first() {
        Ok(Expr::Table {
            fields: HashMap::from([
                (
                    String::from("type"),
                    Expr::LiteralString(String::from("json")),
                ),
                (String::from("body"), Expr::LiteralString(str.clone())),
            ]),
        })
    } else {
        Err(MacroError::MismatchParams)
    }
}
