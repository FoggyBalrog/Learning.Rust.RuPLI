#[derive(Debug)]
pub enum Token {
    In,
    Out,
    Identifier(String),
    Number(i64),
    Equals,
    Plus,
    Minus,
    NewLine,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' => {
                chars.next(); // Skip
            }
            '\n' => {
                tokens.push(Token::NewLine);
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_numeric() {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            'a'..='z' | '_' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "in" => tokens.push(Token::In),
                    "out" => tokens.push(Token::Out),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            '=' => {
                tokens.push(Token::Equals);
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
            _ => panic!("Unexpected character: {}", c),
        }
    }
    tokens
}
