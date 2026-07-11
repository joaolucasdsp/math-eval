use crate::lexer::Token;

pub enum Expr {
    Number(f64),
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;

        if self.peek().is_some() {
            return Err("Unexpected token after expression".to_string());
        }

        Ok(expr)
    }

    // expr := term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while matches!(self.peek(), Some(Token::Plus | Token::Minus)) {
            let op = match self.peek() {
                Some(Token::Plus) => Token::Plus,
                Some(Token::Minus) => Token::Minus,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // term := factor ('*' | '/' factor)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;

        while matches!(self.peek(), Some(Token::Multiply | Token::Divide)) {
            let op = match self.peek() {
                Some(Token::Multiply) => Token::Multiply,
                Some(Token::Divide) => Token::Divide,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // power  := unary ('^' power)?
    fn parse_power(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary()?;

        if let Some(Token::Power) = self.peek() {
            self.advance();

            let right = self.parse_power()?;

            Ok(Expr::BinaryOp {
                op: Token::Power,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    // unary   := '-' unary | primary
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::BinaryOp {
                    op: Token::Minus,
                    left: Box::new(Expr::Number(0.0)),
                    right: Box::new(expr),
                })
            }

            _ => self.parse_primary(),
        }
    }

    // primary := NUMBER | IDENT | IDENT '(' expr ')' | '(' expr ')'
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n as f64))
            }

            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();

                match self.peek() {
                    Some(Token::LParen) => {
                        self.advance();

                        let args = self.parse_function_args()?;

                        Ok(Expr::FunctionCall { name, args })
                    }

                    _ => match name.as_str() {
                        "pi" => Ok(Expr::Number(std::f64::consts::PI)),
                        "e" => Ok(Expr::Number(std::f64::consts::E)),
                        _ => Err(format!("unknown constant: {name}")),
                    },
                }
            }

            Some(Token::LParen) => {
                self.advance();

                let expr = self.parse_expr()?;

                match self.peek() {
                    Some(Token::RParen) => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err("Expected ')'".to_string()),
                }
            }

            _ => Err("Expected a number or '('".to_string()),
        }
    }

    fn parse_function_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if matches!(self.peek(), Some(Token::RParen)) {
            self.advance();
            return Ok(args);
        }

        loop {
            let expr = self.parse_expr()?;
            args.push(expr);

            match self.peek() {
                Some(Token::Comma) => {
                    self.advance();
                }
                Some(Token::RParen) => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected ',' or ')' in function arguments".to_string()),
            }
        }

        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::eval;
    use crate::lexer::tokenize;

    fn parse_and_eval(input: &str) -> f64 {
        let tokens = tokenize(input).unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        eval(&expr).unwrap()
    }

    #[test]
    fn parses_addition() {
        assert_eq!(parse_and_eval("2 + 3"), 5.0);
    }

    #[test]
    fn parses_subtraction() {
        assert_eq!(parse_and_eval("10 - 4"), 6.0);
    }

    #[test]
    fn parses_multiplication() {
        assert_eq!(parse_and_eval("5 * 6"), 30.0);
    }

    #[test]
    fn respects_operator_precedence() {
        assert_eq!(parse_and_eval("2 + 3 * 4"), 14.0);
    }

    #[test]
    fn parses_left_associative_addition_and_subtraction() {
        assert_eq!(parse_and_eval("10 - 3 - 2"), 5.0);
    }

    #[test]
    fn returns_error_for_empty_input() {
        let tokens = tokenize("").unwrap();
        let mut parser = Parser::new(tokens);

        assert!(parser.parse_expr().is_err());
    }

    #[test]
    fn returns_error_for_operator_without_right_side() {
        let tokens = tokenize("2 +").unwrap();
        let mut parser = Parser::new(tokens);

        assert!(parser.parse_expr().is_err());
    }

    #[test]
    fn parses_division() {
        assert_eq!(parse_and_eval("8 / 2"), 4.0);
    }

    #[test]
    fn parses_parentheses() {
        assert_eq!(parse_and_eval("(2 + 3) * 4"), 20.0);
    }

    #[test]
    fn parses_negative_number() {
        assert_eq!(parse_and_eval("-3 + 5"), 2.0);
    }

    #[test]
    fn parses_negative_number_after_operator() {
        assert_eq!(parse_and_eval("2 * -3"), -6.0);
    }

    #[test]
    fn parses_power() {
        assert_eq!(parse_and_eval("2 ^ 3"), 8.0);
    }

    #[test]
    fn power_is_right_associative() {
        assert_eq!(parse_and_eval("2 ^ 3 ^ 2"), 512.0);
    }

    #[test]
    fn parses_pi_constant() {
        assert_float_eq(parse_and_eval("pi"), std::f64::consts::PI);
    }

    #[test]
    fn parses_expression_with_pi() {
        assert_float_eq(parse_and_eval("2 * pi"), 2.0 * std::f64::consts::PI);
    }

    fn assert_float_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1e-10);
    }

    #[test]
    fn parses_sqrt_function() {
        assert_eq!(parse_and_eval("sqrt(9)"), 3.0);
    }

    #[test]
    fn parses_abs_function() {
        assert_eq!(parse_and_eval("abs(-3)"), 3.0);
    }

    #[test]
    fn parses_function_inside_expression() {
        assert_eq!(parse_and_eval("2 + sqrt(9)"), 5.0);
    }

    #[test]
    fn parses_nested_function_call() {
        assert_eq!(parse_and_eval("sqrt(abs(-9))"), 3.0);
    }

    #[test]
    fn rejects_extra_tokens_after_expression() {
        let tokens = tokenize("2 3").unwrap();
        let mut parser = Parser::new(tokens);

        assert!(parser.parse().is_err());
    }

    #[test]
    fn parses_max_function() {
        assert_eq!(parse_and_eval("max(2, 5)"), 5.0);
    }

    #[test]
    fn parses_min_function() {
        assert_eq!(parse_and_eval("min(2, 5)"), 2.0);
    }

    #[test]
    fn parses_pow_function() {
        assert_eq!(parse_and_eval("pow(2, 3)"), 8.0);
    }

    #[test]
    fn parses_nested_multi_arg_function() {
        assert_eq!(parse_and_eval("max(2, min(10, 4))"), 4.0);
    }
}

