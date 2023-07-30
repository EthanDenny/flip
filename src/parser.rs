use std::iter::Peekable;

use crate::token::{Token, TokenType};

struct Parser<'a, I>
where
    I: Iterator<Item = (usize, char)>
{
    code: &'a str,
    chars: Peekable<I>,
    line: usize,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = (usize, char)>,
{
    fn new(code: &'a str, chars: I) -> Self {
        Parser {
            code,
            chars: chars.peekable(),
            line: 1,
        }
    }
}

pub fn parse(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut parser = Parser::new(code, code.chars().enumerate());
    
    while let Some((i, c)) = parser.chars.next() {
        let t = match c {
            '(' => Token::new(TokenType::LeftParen, &code[i..i+1], parser.line),
            ')' => Token::new(TokenType::RightParen, &code[i..i+1], parser.line),
            'a'..='z' | 'A'..='Z' | '_' => parse_atom(&mut parser, i),
            '0'..='9' => parse_number(&mut parser, i),
            '"' => parse_string(&mut parser, i),
            '\n' => {
                parser.line += 1;
                continue;
            }
            _ => {
                continue;
            }
        };

        tokens.push(t);
    }

    tokens
}

fn parse_atom<'a, I>(parser: &mut Parser<'a, I>, start: usize) -> Token<'a>
where
    I: Iterator<Item = (usize, char)>
{
    let mut end = 0;

    while let Some(&(j, c)) = parser.chars.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            parser.chars.next();
        } else {
            end = j;
            break;
        }
    }

    Token::new(TokenType::Atom, &parser.code[start..end], parser.line)
}

fn parse_number<'a, I>(parser: &mut Parser<'a, I>, start: usize) -> Token<'a>
where
    I: Iterator<Item = (usize, char)>
{
    let mut end = 0;
    let mut num_type = TokenType::Int;

    while let Some(&(j, c)) = parser.chars.peek() {
        if c.is_ascii_digit() {
            parser.chars.next();
        } else {
            end = j;
            break;
        }
    }

    if let Some(&(_, c)) = parser.chars.peek() {
        if c == '.' {
            parser.chars.next();
            num_type = TokenType::Float;

            while let Some(&(j, c)) = parser.chars.peek() {
                if c.is_ascii_digit() {
                    parser.chars.next();
                } else {
                    end = j;
                    break;
                }
            }
        }
    }

    Token::new(num_type, &parser.code[start..end], parser.line)
}

fn parse_string<'a, I: Iterator<Item = (usize, char)>>(parser: &mut Parser<'a, I>, start: usize) -> Token<'a> {
    while let Some(&(j, c)) = parser.chars.peek() {
        parser.chars.next();
        match c {
            '"' => {
                return Token::new(TokenType::String, &parser.code[start+1..j], parser.line);
            }
            '\n' => { break; }
            _ => {}
        }
    }

    panic!("String unterminated")
}
