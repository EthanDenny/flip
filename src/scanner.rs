use std::iter::Peekable;

use crate::token::{Token, TokenType};

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

pub fn scan(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanner = Scanner::new(code, code.chars().enumerate());
    
    while let Some((i, c)) = scanner.chars.next() {
        let t = match c {
            '(' => Token::new(TokenType::LeftParen, &code[i..i+1], scanner.line),
            ')' => Token::new(TokenType::RightParen, &code[i..i+1], scanner.line),
            '\'' => scan_literal(&mut scanner, i),
            'a'..='z' | 'A'..='Z' | '-' => scan_atom(&mut scanner, i),
            '0'..='9' => scan_number(&mut scanner, i),
            '"' => scan_string(&mut scanner, i),
            '\n' => {
                scanner.line += 1;
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

fn find_atom_end<I>(scanner: &mut Scanner<I>) -> usize
where
    I: Iterator<Item = (usize, char)>
{
    while let Some(&(j, c)) = scanner.chars.peek() {
        if c.is_ascii_alphanumeric() || c == '-' {
            scanner.chars.next();
        } else {
            return j;
        }
    }

    0
}

fn scan_literal<'a, I>(scanner: &mut Scanner<'a, I>, start: usize) -> Token<'a>
where
    I: Iterator<Item = (usize, char)>
{
    scanner.chars.next();

    let end = find_atom_end(scanner);

    Token::new(TokenType::Literal, &scanner.code[start + 1..end], scanner.line)
}

fn scan_atom<'a, I>(scanner: &mut Scanner<'a, I>, start: usize) -> Token<'a>
where
    I: Iterator<Item = (usize, char)>
{
    let end = find_atom_end(scanner);

    Token::new(TokenType::Atom, &scanner.code[start..end], scanner.line)
}

fn scan_number<'a, I>(scanner: &mut Scanner<'a, I>, start: usize) -> Token<'a>
where
    I: Iterator<Item = (usize, char)>
{
    let mut end = 0;
    let mut num_type = TokenType::Int;

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

    Token::new(num_type, &scanner.code[start..end], scanner.line)
}

fn scan_string<'a, I: Iterator<Item = (usize, char)>>(scanner: &mut Scanner<'a, I>, start: usize) -> Token<'a> {
    while let Some(&(j, c)) = scanner.chars.peek() {
        scanner.chars.next();
        if c == '"' {
            return Token::new(TokenType::String, &scanner.code[start+1..j], scanner.line);
        }
    }

    panic!("String unterminated")
}
