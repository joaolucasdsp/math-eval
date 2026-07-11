#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Power,
    Ident(String),
    Comma,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Power);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '0'..='9' => {
                let mut digit_str = String::new();
                let mut seen_dot = false;

                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        digit_str.push(digit);
                        chars.next();
                    } else if digit == '.' && !seen_dot {
                        seen_dot = true;
                        digit_str.push('.');
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(
                    digit_str
                        .parse::<f64>()
                        .map_err(|_| format!("invalid number: {digit_str}"))?,
                ));
            }
            'A'..='Z' | 'a'..='z' => {
                let mut ident = String::new();

                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Ident(ident));
            }
            _ => return Err(format!("unexpectable character: {c}")),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_plus() {
        let tokens = tokenize("2 + 3").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Number(2.0), Token::Plus, Token::Number(3.0),]
        );
    }

    #[test]
    fn tokenizes_minus() {
        let tokens = tokenize("10 - 4").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Number(10.0), Token::Minus, Token::Number(4.0),]
        );
    }

    #[test]
    fn tokenizes_multiply() {
        let tokens = tokenize("5 * 6").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Number(5.0), Token::Multiply, Token::Number(6.0),]
        );
    }

    #[test]
    fn tokenizes_divide() {
        let tokens = tokenize("8 / 2").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Number(8.0), Token::Divide, Token::Number(2.0),]
        );
    }

    #[test]
    fn tokenizes_parentheses() {
        assert_eq!(
            tokenize("(2 + 3) * 4").unwrap(),
            vec![
                Token::LParen,
                Token::Number(2.0),
                Token::Plus,
                Token::Number(3.0),
                Token::RParen,
                Token::Multiply,
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn tokenizes_float() {
        let tokens = tokenize("8.2 + 2.1").unwrap();

        assert_eq!(
            tokens,
            vec![Token::Number(8.2), Token::Plus, Token::Number(2.1),]
        );
    }
}
