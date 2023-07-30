use std::iter::Peekable;

use crate::token::{Token, TokenType};

pub fn parse(code: &str) -> Vec<Token> {
    let mut chars = code.chars().enumerate().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 1;
    
    while let Some((i, c)) = chars.next() {
        let t = match c {
            '(' => Some(Token {
                token_type: TokenType::LeftParen,
                content: &code[i..i+1],
                line,
            }),
            ')' => Some(Token {
                token_type: TokenType::RightParen,
                content: &code[i..i+1],
                line,
            }),
            'a'..='z' | 'A'..='Z' | '_' => parse_atom(&mut chars, code, i, line),
            '0'..='9' => parse_number(&mut chars, code, i, line),
            '"' => parse_string(&mut chars, code, i, line),
            '\n' => {
                line += 1;
                None
            }
            _ => None
        };

        if let Some(t) = t {
            tokens.push(t);
        }
    }

    tokens
}

fn parse_atom<'a, I: Iterator<Item = (usize, char)>>(chars: &mut Peekable<I>, code: &'a str, start: usize, line: usize) -> Option<Token<'a>> {
    while let Some(&(j, c)) = chars.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                chars.next();
            }
            _ => {
                return Some(Token {
                    token_type: TokenType::Atom,
                    content: &code[start..j],
                    line,
                });
            }
        }
    }

    None
}

fn parse_number<'a, I: Iterator<Item = (usize, char)>>(chars: &mut Peekable<I>, code: &'a str, start: usize, line: usize) -> Option<Token<'a>> {
    let mut end = 0;
    let mut num_type = TokenType::Int;

    while let Some(&(j, c)) = chars.peek() {
        match c {
            '0'..='9' => {
                chars.next();
            }
            _ => {
                end = j;
                break;
            }
        }
    }

    if let Some(&(_, c)) = chars.peek() {
        if c == '.' {
            chars.next();
            num_type = TokenType::Float;

            while let Some(&(j, c)) = chars.peek() {
                match c {
                    '0'..='9' => {
                        chars.next();
                    }
                    _ => {
                        end = j;
                        break;
                    }
                }
            }
        }
    }

    Some(Token {
        token_type: num_type,
        content: &code[start..end],
        line,
    })
}

fn parse_string<'a, I: Iterator<Item = (usize, char)>>(chars: &mut Peekable<I>, code: &'a str, start: usize, line: usize) -> Option<Token<'a>> {
    while let Some(&(j, c)) = chars.peek() {
        chars.next();
        if c == '"' {
            return Some(Token {
                token_type: TokenType::String,
                content: &code[start+1..j],
                line,
            });
        }
    }

    None
}
