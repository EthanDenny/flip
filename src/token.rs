#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Atom,
    Int,
    Float,
    String,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub content: &'a str,
    pub line: usize,
}

pub fn debug_tokens(tokens: &Vec<Token>) {
    let mut line = 1;

    for t in tokens {
        while t.line > line {
            print!("\n{line:<4}: ");
            line += 1;
        }
        if t.token_type == TokenType::LeftParen || t.token_type == TokenType::RightParen {
            print!("{} ", t.content);
        } else {
            print!("{:?}[{}] ", t.token_type, t.content);
        }
    }
}
