use std::fmt;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::error::{throw, throw_at};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
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

pub struct TokensList {
    tokens: Peekable<IntoIter<Token>>
}

impl TokensList {
    pub fn from(tokens: Vec<Token>) -> TokensList {
        TokensList { tokens: tokens.into_iter().peekable() }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn consume(&mut self) -> Token {
        if let Some(token) = self.tokens.next() {
            token
        } else {
            throw("Expected another token, but reached end");
        }
    }

    pub fn expect(&mut self, expected: TokenType) -> Token {
        let token = self.consume();

        if token.token_type == expected {
            return token;
        } else {
            throw_at(&format!("Unexpected token {}, expected {expected:?}", token.content), token.line);
        }
    }
}