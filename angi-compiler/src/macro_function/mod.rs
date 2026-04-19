use std::collections::HashMap;

use crate::{
    compiler::ast::{Expr, InterpolatedPart},
    macro_function::{
        angi_macro::response::{html, html_template, json},
        error::MacroError,
    },
    register_macros,
};

mod angi_macro;
mod error;
mod macros;

pub struct MacroContext {}

type MacroFn = fn(Vec<Expr>, &mut MacroContext) -> Result<Expr, MacroError>;

pub struct MacroRegistry {
    macros: HashMap<String, MacroFn>,
    ctx: MacroContext,
}

impl MacroRegistry {
    pub fn new() -> Self {
        let macros = register_macros! {
            "htmlTemplate" => html_template,
            "html" => html,
            "json" => json
        };

        Self {
            macros,
            ctx: MacroContext {},
        }
    }

    pub fn expand(&mut self, name: &str, args: Vec<Expr>) -> Result<Option<Expr>, MacroError> {
        if let Some(f) = self.macros.get(name) {
            let result = f(args, &mut self.ctx)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    pub fn expand_expr(&mut self, expr: Expr) -> Result<Expr, MacroError> {
        match expr {
            Expr::Unary { op, rhs } => {
                let rhs = Box::new(self.expand_expr(*rhs)?);
                Ok(Expr::Unary { op, rhs })
            }

            Expr::Binary { op, lhs, rhs } => {
                let lhs = Box::new(self.expand_expr(*lhs)?);
                let rhs = Box::new(self.expand_expr(*rhs)?);
                Ok(Expr::Binary { op, lhs, rhs })
            }

            Expr::Pipe { lhs, rhs } => {
                let lhs = Box::new(self.expand_expr(*lhs)?);
                let rhs = Box::new(self.expand_expr(*rhs)?);
                Ok(Expr::Pipe { lhs, rhs })
            }

            Expr::Table { fields } => {
                let fields = fields
                    .into_iter()
                    .map(|(k, v)| Ok((k, self.expand_expr(v)?)))
                    .collect::<Result<HashMap<_, _>, _>>()?;

                Ok(Expr::Table { fields })
            }

            Expr::List { items } => {
                let items = items
                    .into_iter()
                    .map(|e| self.expand_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Expr::List { items })
            }

            Expr::LetIn { let_part, in_part } => {
                let let_part = let_part
                    .into_iter()
                    .map(|(k, v)| Ok((k, self.expand_expr(v)?)))
                    .collect::<Result<HashMap<_, _>, _>>()?;

                let in_part = Box::new(self.expand_expr(*in_part)?);

                Ok(Expr::LetIn { let_part, in_part })
            }

            Expr::FunctionDeclare { params, body } => {
                let body = Box::new(self.expand_expr(*body)?);
                Ok(Expr::FunctionDeclare { params, body })
            }

            Expr::FunctionCall { name, args } => {
                let args = args
                    .into_iter()
                    .map(|e| self.expand_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(f) = self.macros.get(&name) {
                    return f(args, &mut self.ctx);
                }

                Ok(Expr::FunctionCall { name, args })
            }

            Expr::InterpolatedString(parts) => {
                let parts = parts
                    .into_iter()
                    .map(|p| match p {
                        InterpolatedPart::Expr(e) => {
                            Ok(InterpolatedPart::Expr(self.expand_expr(e)?))
                        }
                        other => Ok(other),
                    })
                    .collect::<Result<Vec<_>, MacroError>>()?;

                Ok(Expr::InterpolatedString(parts))
            }

            other => Ok(other),
        }
    }

      pub fn expand_expr_inplace(&mut self, expr: &mut Expr) -> Result<(), MacroError> {
        match expr {
            Expr::Unary { rhs, .. } => {
                self.expand_expr_inplace(rhs)?;
            }

            Expr::Binary { lhs, rhs, .. } => {
                self.expand_expr_inplace(lhs)?;
                self.expand_expr_inplace(rhs)?;
            }

            Expr::Pipe { lhs, rhs } => {
                self.expand_expr_inplace(lhs)?;
                self.expand_expr_inplace(rhs)?;
            }

            Expr::Table { fields } => {
                for v in fields.values_mut() {
                    self.expand_expr_inplace(v)?;
                }
            }

            Expr::List { items } => {
                for item in items {
                    self.expand_expr_inplace(item)?;
                }
            }

            Expr::LetIn { let_part, in_part } => {
                for v in let_part.values_mut() {
                    self.expand_expr_inplace(v)?;
                }
                self.expand_expr_inplace(in_part)?;
            }

            Expr::FunctionDeclare { body, .. } => {
                self.expand_expr_inplace(body)?;
            }

            Expr::InterpolatedString(parts) => {
                for part in parts {
                    if let InterpolatedPart::Expr(e) = part {
                        self.expand_expr_inplace(e)?;
                    }
                }
            }

            Expr::FunctionCall { name, args } => {
                let args_into = args.clone();

                for arg in args {
                    self.expand_expr_inplace(arg)?;
                }

                if let Some(f) = self.macros.get(name) {
                    let new_expr = f(args_into, &mut self.ctx)?;
                    *expr = new_expr;

                    self.expand_expr_inplace(expr)?;
                }
            }

            _ => {}
        }

        Ok(())
    }
}

impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}
