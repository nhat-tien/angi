use core_type::Type;
use schema::get_root_schema;

use crate::{compiler::ast::Expr, diagnostic::{Diagnostic, DiagnosticEngine, Severity, Span}};

pub mod schema;
pub mod core_type;

pub fn type_checking(expr: &Expr, diagnostic: &mut DiagnosticEngine) {
    let core_schema = get_root_schema();
    check("<root>", expr, &core_schema, diagnostic);
}

fn infer(expr: &Expr) -> Type {
    match expr {
        Expr::Number(_) => Type::Number,
        Expr::LiteralString(_) => Type::String,
        Expr::Table { fields: _ }  => Type::TableDynamic,
        Expr::List { items: _ }  => Type::ListDynamic,
        Expr::FunctionDeclare { params: _, body: _ } => Type::FunctionDynamic,
        Expr::Boolean(_) => Type::Boolean,
        Expr::LetIn { let_part: _, in_part } => {
            infer(in_part)
        }
        _ => Type::Unknown
    }
}

fn check(attribute_name: &str, expr: &Expr, expected: &Type, diagnostic: &mut DiagnosticEngine) {
    match expected {
        Type::Number => {
            if !matches!(expr, Expr::Number(_)) {
                report_error(diagnostic, format!("The {} expect {:?}, but found {:?}", attribute_name, expected, infer(expr)));
            }
        }
        Type::String => {
            if !matches!(expr, Expr::LiteralString(_)) {
                report_error(diagnostic, format!("The {} expect {:?}, but found {:?}", attribute_name, expected, infer(expr)));
            }
        }
        Type::Boolean => {
            if !matches!(expr, Expr::Boolean(_)) {
                report_error(diagnostic, format!("The {} expect {:?}, but found {:?}", attribute_name, expected, infer(expr)));
            }
        }
        Type::Table(field_schema) => {
            if !matches!(infer(expr), Type::TableDynamic) {
                report_error(diagnostic, format!("The {} expect {:?}, but found {:?}", attribute_name, "Table", infer(expr)));
            };

            if let Expr::Table { fields: fields_in_expr } = expr {
                for (name, type_check) in field_schema {
                    match fields_in_expr.get(name) {
                        Some(expr_from_expr) => {
                            check(&format!("{}.{}", attribute_name, name), expr_from_expr, type_check, diagnostic);
                        },
                        None => {
                            report_error(diagnostic, format!("The {} attribute not found in {}", name, attribute_name));
                        }
                    }

                }
            }
        }
        Type::List(type_schema) => {
            if !matches!(infer(expr), Type::ListDynamic) {
                report_error(diagnostic, format!("The {} expect {:?}, but found {:?}", attribute_name, "List", infer(expr)));
            };

            if let Expr::List { items: items_in_expr } = expr {
                for (index, item) in items_in_expr.iter().enumerate() {
                    check(&format!("{}[{}]", attribute_name, index), item, type_schema, diagnostic);
                }
            }
        }
        Type::Function { params_type: _, return_type: _ } => {
            todo!()
        }
        _ => panic!("The {:?} is not support in type checking right now", expected)
    }
}

pub fn report_error(diagnostic: &mut DiagnosticEngine, message: String) {
    diagnostic.report(Diagnostic {
        severity: Severity::Error,
        message,
        span: Span {
            line: 0_usize,
            column: 0_usize,
        },
        span_len: 1,
        help: None,
        notes: vec![],
    });
}

