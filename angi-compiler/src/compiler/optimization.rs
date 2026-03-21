use super::ast::{Expr, Operator};

pub fn optimization(ast: &mut Expr) {
    match ast {
        Expr::Table { fields } => {
            for field in fields.values_mut() {
                optimization(field);
            }
        },
        Expr::Binary { .. } => {
            *ast = Expr::Number(calculate_expr(ast));
        },
        _ => ()
    }
}

fn calculate_expr(ast: &Expr) -> i32 {
    match ast {
        Expr::Number(num) => *num,
        Expr::Unary { op, rhs } => {
            let rhs_num = calculate_expr(rhs);
            match op {
                Operator::Add => { rhs_num },
                Operator::Sub => { - rhs_num },
                _ => rhs_num
            }
        }
        Expr::Binary { op, lhs, rhs } => {
                let lhs_num = calculate_expr(lhs);
                let rhs_num = calculate_expr(rhs);
                match op {
                    Operator::Add => { lhs_num + rhs_num },
                    Operator::Sub => { lhs_num - rhs_num },
                    Operator::Mul => { lhs_num * rhs_num },
                    Operator::Div => { lhs_num / rhs_num },
                }
        },
        _ => 0
    }
}

