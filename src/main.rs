use crate::{eval::eval, lexer::tokenize, parser::Parser};

mod eval;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let expression = "(2 + 4) / 2";

    let tokens = tokenize(expression)?;
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    let result = eval(&expr)?;

    println!("{result}");

    Ok(())
}
