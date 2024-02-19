use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::tokens::{Token, TokenType};

struct Scanner<'a> {
    code: &'a str,
    chars: Peekable<Enumerate<Chars<'a>>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn new(code: &'a str) -> Self {
        Scanner {
            code,
            chars: code.chars().enumerate().peekable(),
            line: 1,
        }
    }
}

pub fn get_tokens(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner = Scanner::new(code);
    
    while let Some((i, c)) = scanner.chars.next() {
        tokens.push(
            match c {
                // Single-character tokens
                '(' => one_char_token(TokenType::LeftParen, &mut scanner, i),
                ')' => one_char_token(TokenType::RightParen, &mut scanner, i),
                '[' => one_char_token(TokenType::LeftBracket, &mut scanner, i),
                ']' => one_char_token(TokenType::RightBracket, &mut scanner, i),
                '{' => one_char_token(TokenType::LeftBrace, &mut scanner, i),
                '}' => one_char_token(TokenType::RightBrace, &mut scanner, i),
                ',' => one_char_token(TokenType::Comma, &mut scanner, i),
                '.' => one_char_token(TokenType::Dot, &mut scanner, i),
                ':' => one_char_token(TokenType::Colon, &mut scanner, i),

                // Whitespace
                '\n' | '\r' => {
                    scanner.line += 1;
                    continue;
                },
                ' ' | '\t' => {
                    continue;
                }

                // Cool stuff
                '0'..='9' => scan_int(&mut scanner, i),
                _ => {
                    if i + 3 <= scanner.code.len() && &scanner.code[i..i+3] == "let" {
                        for _ in 0..2 { scanner.chars.next(); }
                        Token::new(
                            TokenType::Let,
                            String::from(&scanner.code[i..i+3]),
                            scanner.line
                        )
                    } else if i + 4 <= scanner.code.len() && &scanner.code[i..i+4] == "true" {
                        for _ in 0..3 { scanner.chars.next(); }
                        Token::new(
                            TokenType::True,
                            String::from(&scanner.code[i..i+4]),
                            scanner.line
                        )
                    } else if i + 5 <= scanner.code.len() && &scanner.code[i..i+5] == "false" {
                        for _ in 0..4 { scanner.chars.next(); }
                        Token::new(
                            TokenType::False,
                            String::from(&scanner.code[i..i+5]),
                            scanner.line
                        )
                    } else {
                        scan_literal(&mut scanner, i)
                    }
                }
            }
        );
    }

    tokens
}

fn one_char_token<'a>(token_type: TokenType, scanner: &mut Scanner<'a>, start: usize) -> Token {
    Token::new(token_type, String::from(&scanner.code[start..start+1]), scanner.line)
}

fn scan_literal<'a>(scanner: &mut Scanner<'a>, start: usize) -> Token {
    let mut end = 0;

    while let Some(&(j, c)) = scanner.chars.peek() {
        match c {
            '(' | ')' | '[' | ']' | '{' | '}' |
            '.' | ':' | '-' |
            '0'..='9' | '\'' | '\"' |
            ',' | ' ' |
            '\n' | '\r' | '\t' => {
                end = j;
                break;
            }
            _ => { scanner.chars.next(); }
        }
    }

    Token::new(TokenType::Literal, String::from(&scanner.code[start..end]), scanner.line)
}

fn scan_int<'a>(scanner: &mut Scanner<'a>, start: usize) -> Token {
    let mut end = 0;

    while let Some(&(j, c)) = scanner.chars.peek() {
        if c.is_ascii_digit() {
            scanner.chars.next();
        } else {
            end = j;
            break;
        }
    }

    Token::new(TokenType::Integer, String::from(&scanner.code[start..end]), scanner.line)
}
