use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Literal,
    Atom,
    Int,
    Float,
    String,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::Literal => write!(f, "Literal"),
            TokenType::Atom => write!(f, "Atom"),
            TokenType::Int => write!(f, "Int"),
            TokenType::Float => write!(f, "Float"),
            TokenType::String => write!(f, "String"),
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub content: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, content: &'a str, line: usize) -> Token<'a> {
        Token {
            token_type,
            content,
            line
        }
    }
}

/*
pub fn debug_tokens(tokens: &Vec<Token>) {
    let mut line = 0;

    for t in tokens {
        while t.line > line {
            line += 1;
            if line > 1 {
                println!();
            }
            print!("{line:<4}: ");
        }

        match t.token_type {
            TokenType::LeftParen | TokenType::RightParen => print!("{} ", t.token_type),
            _ => print!("{}[{}] ", t.token_type, t.content),
        }
    }
}
*/
