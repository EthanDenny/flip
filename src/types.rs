use std::fmt;

pub struct Buffer(String);

impl Buffer {
    pub fn new() -> Buffer {
        Buffer(String::new())
    }

    pub fn emit(&mut self, text: &str) {
        self.0.push_str(text);
    }

    pub fn emit_instr(&mut self, text: String) {
        self.emit(&format!("    {text}\n"));
    }

    pub fn get<'a>(self) -> String {
        self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Equals,
    Comma,
    Dot,
    Colon,
    Arrow,
    Fn,
    
    String,
    Integer,
    Float,
    Literal
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),

            TokenType::Equals => write!(f, "="),
            TokenType::Dot => write!(f, "."),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::Arrow => write!(f, "->"),
            TokenType::Fn => write!(f, "fn"),

            TokenType::String => write!(f, "String"),
            TokenType::Integer => write!(f, "Integer"),
            TokenType::Float => write!(f, "Float"),
            TokenType::Literal => write!(f, "Literal")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, content: String, line: usize) -> Token {
        Token { token_type,content, line }
    }
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Call(String, Vec<ASTNode>),
    Int(i32),
    Bool(bool),
}

// Types of function arguments and returns
#[derive(PartialEq)]
pub enum T<'a> {
    Int,
    Bool,
    Generic(&'a str)
}
