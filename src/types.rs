use std::fmt;

use crate::symbols::Symbol;

pub struct Buffer {
    content: String
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { content: String::new() }
    }

    pub fn emit(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn emit_instr(&mut self, text: &str) {
        self.content.push_str(&format!("    {text}\n"));
    }

    pub fn get<'a>(self) -> String {
        self.content
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

    Comma,
    Dot,
    Colon,
    Let,
    
    True,
    False,
    Integer,
    Literal,
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

            TokenType::Dot => write!(f, "."),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::Let => write!(f, "let"),

            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::Integer => write!(f, "Integer"),
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
    Fn(String, Vec<Symbol>, T, Vec<ASTNode>),
    Call(String, Vec<ASTNode>),
    Let(Symbol, Box<ASTNode>),
    Var(Symbol),
    Int(i32),
    Bool(bool),
}

impl ASTNode {
    pub fn imm_repr(&self) -> String {
        match self {
            ASTNode::Int(v) => format!("{v}"),
            ASTNode::Bool(v) => if *v { String::from("1") } else { String::from("0") }
            ASTNode::Var(s) => s.name.clone(),
            _ => String::new()
        }
    }
}

// Types of function arguments and returns
#[derive(Debug, PartialEq, Clone)]
pub enum T {
    Int,
    Bool,
    Fn,
    None,
    Generic(String)
}

impl fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            T::Int => write!(f, "Int"),
            T::Bool => write!(f, "Bool"),
            T::Fn => write!(f, "Fn"),
            T::None => write!(f, "None"),
            T::Generic(generic_name) => write!(f, "Generic({generic_name})"),
        }
    }
}

impl<'a> T {
    pub fn gen(name: &'a str) -> T {
        T::Generic(name.to_string())
    }
}
