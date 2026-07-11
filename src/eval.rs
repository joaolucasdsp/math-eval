use crate::{lexer::Token, parser::Expr};

pub fn eval(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::BinaryOp { op, left, right } => {
            let l = eval(left)?;
            let r = eval(right)?;

            match op {
                Token::Plus => Ok(l + r),
                Token::Minus => Ok(l - r),
                Token::Multiply => Ok(l * r),
                Token::Divide => {
                    if r == 0.0 {
                        Err("division by zero".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
                Token::Power => Ok(l.powf(r)),
                _ => Err("invalid operator: {op}".into()),
            }
        }
        Expr::FunctionCall { name, args } => {
            let values: Result<Vec<f64>, String> = args.iter().map(eval).collect();
            let values = values?;

            match name.as_str() {
                "sqrt" if values.len() == 1 => Ok(values[0].sqrt()),
                "abs" if values.len() == 1 => Ok(values[0].abs()),
                "sin" if values.len() == 1 => Ok(values[0].sin()),
                "cos" if values.len() == 1 => Ok(values[0].cos()),

                "max" if values.len() == 2 => Ok(values[0].max(values[1])),
                "min" if values.len() == 2 => Ok(values[0].min(values[1])),
                "pow" if values.len() == 2 => Ok(values[0].powf(values[1])),

                _ => Err(format!("Invalid function call: {name}")),
            }
        }
    }
}
