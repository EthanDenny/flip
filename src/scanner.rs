use std::iter::Peekable;

use crate::types::{Token, TokenType};

struct Scanner<'a, I>
where
    I: Iterator<Item = (usize, char)>
{
    code: &'a str,
    chars: Peekable<I>,
    line: usize,
}

impl<'a, I> Scanner<'a, I>
where
    I: Iterator<Item = (usize, char)>,
{
    fn new(code: &'a str, chars: I) -> Self {
        Scanner {
            code,
            chars: chars.peekable(),
            line: 1,
        }
    }
}

pub fn get_tokens(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner = Scanner::new(code, code.chars().enumerate());
    
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
                '=' => one_char_token(TokenType::Equals, &mut scanner, i),
                ',' => one_char_token(TokenType::Comma, &mut scanner, i),
                '.' => one_char_token(TokenType::Dot, &mut scanner, i),
                ':' => one_char_token(TokenType::Colon, &mut scanner, i),
                
                // Two-character tokens
                '-' => {
                    if let Some(&(_, c)) = scanner.chars.peek() {
                        if c == '>' {
                            scanner.chars.next();
                            Token::new(
                                TokenType::Arrow,
                                String::from(&scanner.code[i..i+2]),
                                scanner.line
                            )
                        }
                        else {
                            scan_literal(&mut scanner, i)
                        }
                    } else {
                        // Can't be anything else
                        Token::new(
                            TokenType::Literal,
                            String::from(&scanner.code[i..i+1]),
                            scanner.line
                        )
                    }
                }

                'f' => {
                    if let Some(&(_, c)) = scanner.chars.peek() {
                        if c == 'n' {
                            scanner.chars.next();
                            Token::new(
                                TokenType::Fn,
                                String::from(&scanner.code[i..i+2]),
                                scanner.line
                            )
                        }
                        else {
                            scan_literal(&mut scanner, i)
                        }
                    } else {
                        // Can't be anything else
                        Token::new(
                            TokenType::Literal,
                            String::from(&scanner.code[i..i+1]),
                            scanner.line
                        )
                    }
                }

                // Whitespace
                '\n' | '\r' => {
                    scanner.line += 1;
                    continue;
                },
                ' ' | '\t' => {
                    continue;
                }

                // Cool stuff
                '\'' | '\"' => scan_string(&mut scanner, i),
                '0'..='9' => scan_number(&mut scanner, i),
                _ => scan_literal(&mut scanner, i),
            }
        );
    }

    tokens
}

fn one_char_token<'a, I>(token_type: TokenType, scanner: &mut Scanner<'a, I>, start: usize) -> Token
where
    I: Iterator<Item = (usize, char)>
{
    Token::new(token_type, String::from(&scanner.code[start..start+1]), scanner.line)
}

fn scan_literal<'a, I>(scanner: &mut Scanner<'a, I>, start: usize) -> Token
where
    I: Iterator<Item = (usize, char)>
{
    let mut arrow_count = 0;
    let mut end = 0;

    while let Some(&(j, c)) = scanner.chars.peek() {
        match c {
            '<' => {
                arrow_count += 1;
            }
            '>' => {
                arrow_count -= 1;
            }
            ',' | ' ' => {
                if arrow_count == 0 {
                    end = j;
                    break;
                }
            }
            '(' | ')' | '[' | ']' | '{' | '}' |
            '=' | '.' | ':' | '-' |
            '0'..='9' | '\'' | '\"' |
            '\n' | '\r' | '\t' => {
                if arrow_count == 0 {
                    end = j;
                    break;
                } else {
                    panic!("Type literal mangled (Line {})", scanner.line);
                }
            }
            _ => {}
        }

        scanner.chars.next();
    }

    Token::new(TokenType::Literal, String::from(&scanner.code[start..end]), scanner.line)
}

fn scan_number<'a, I>(scanner: &mut Scanner<'a, I>, start: usize) -> Token
where
    I: Iterator<Item = (usize, char)>
{
    let mut end = 0;
    let mut num_type = TokenType::Integer;

    while let Some(&(j, c)) = scanner.chars.peek() {
        if c.is_ascii_digit() {
            scanner.chars.next();
        } else {
            end = j;
            break;
        }
    }

    if let Some(&(_, c)) = scanner.chars.peek() {
        if c == '.' {
            scanner.chars.next();
            num_type = TokenType::Float;

            while let Some(&(j, c)) = scanner.chars.peek() {
                if c.is_ascii_digit() {
                    scanner.chars.next();
                } else {
                    end = j;
                    break;
                }
            }
        }
    }

    Token::new(num_type, String::from(&scanner.code[start..end]), scanner.line)
}

fn scan_string<'a, I: Iterator<Item = (usize, char)>>(scanner: &mut Scanner<'a, I>, start: usize) -> Token {
    while let Some(&(j, c)) = scanner.chars.peek() {
        scanner.chars.next();
        if c == '"' {
            return Token::new(TokenType::String, String::from(&scanner.code[start+1..j]), scanner.line);
        }
    }

    panic!("String unterminated (Line {})", scanner.line);
}
