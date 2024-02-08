use std::iter::Peekable;

use crate::error::{throw, throw_at};
use crate::symbols::{Symbol, SymbolTable};
use crate::types::{ASTNode, Token, TokenType, T};

pub fn build_ast(tokens: Vec<Token>, symbols: &mut SymbolTable) -> Vec<ASTNode> {
    let mut tokens = tokens.into_iter().peekable();
    let mut tree: Vec<ASTNode> = vec![];

    while let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::Fn => {
                tree.push(consume_fn_dec(&mut tokens, symbols));
            }
            TokenType::Literal => {
                tree.push(consume_fn_call(token.content, &mut tokens, symbols));
            }
            _ => {}
        }
    }

    tree
}

fn consume_fn_dec<'a, I>(tokens: &mut Peekable<I>, symbols: &mut SymbolTable) -> ASTNode
where
    I: Iterator<Item = Token>
{
    let mut name = String::from("fn_");
    let name_token = expect(tokens, TokenType::Literal);
    name.push_str(&name_token.content);

    expect(tokens, TokenType::LeftParen);

    let args = consume_args(tokens);
    let arg_types = args.iter()
        .map(|arg| arg.symbol_type.clone())
        .collect();

    expect(tokens, TokenType::RightParen);

    // No guaruntee this is actually what is returned by the function,
    // currently, the programmer must be trusted
    let return_type = consume_return(tokens);

    let mut scoped_symbols = symbols.clone();
    scoped_symbols.insert_vec(&args);

    let body = consume_body(tokens, &mut scoped_symbols);

    symbols.insert(Symbol::new_fn(&name_token.content, arg_types, return_type.clone()));
    ASTNode::Fn(name, args, return_type, body)
}

fn consume_return<'a, I>(tokens: &mut Peekable<I>) -> T
where
    I: Iterator<Item = Token>
{
    if let Some(token) = tokens.peek() {
        if token.token_type == TokenType::Arrow {
            consume(tokens);
            let type_name = expect(tokens, TokenType::Literal).content;
            parse_type(type_name)
        } else {
            T::None
        }
    } else {
        throw("Expected return type or block, got end")
    }
}

fn consume_args<'a, I>(tokens: &mut Peekable<I>) -> Vec<Symbol>
where
    I: Iterator<Item = Token>
{
    let mut args = Vec::new();

    while let Some(token) = tokens.peek() {
        if token.token_type != TokenType::RightParen {
            if token.token_type == TokenType::Comma {
                consume(tokens);
            }

            let arg_name = expect(tokens, TokenType::Literal).content;
            expect(tokens, TokenType::Colon);
            let arg_type = expect(tokens, TokenType::Literal).content;
            args.push(Symbol::new_var(&arg_name, parse_type(arg_type)));

        } else {
            break;
        }
    }

    args
}

fn parse_type(type_name: String) -> T {
    match type_name.as_ref() {
        "Int" => T::Int,
        "Bool" => T::Bool,
        "Fn" => T::Fn,
        "None" => T::None,
        _ => T::Generic(type_name)
    }
}

fn consume_body<'a, I>(tokens: &mut Peekable<I>, symbols: &mut SymbolTable) -> Vec<ASTNode>
where
    I: Iterator<Item = Token>
{
    expect(tokens, TokenType::LeftBrace);

    let mut calls = Vec::new();

    while let Some(token) = tokens.peek() {
        if token.token_type == TokenType::RightBrace {
            consume(tokens);
            break;
        } else {
            calls.push(get_arg(tokens, symbols));
        }
    }

    calls
}

fn consume_fn_call<'a, I>(name: String, tokens: &mut Peekable<I>, symbols: &mut SymbolTable) -> ASTNode
where
    I: Iterator<Item = Token>
{
    let left_paren = expect(tokens, TokenType::LeftParen);

    if let Some(token) = tokens.peek() {
        if token.token_type == TokenType::RightParen {
            consume(tokens);
            ASTNode::Call(name, Vec::new())
        } else {
            let mut args = vec![get_arg(tokens, symbols)];

            while let Some(token) = tokens.peek() {
                if token.token_type == TokenType::RightParen {
                    tokens.next();
                    break;
                } else {
                    expect(tokens, TokenType::Comma);
                    args.push(get_arg(tokens, symbols));
                }
            }

            ASTNode::Call(name, args)
        }
    } else {
        throw_at("Expected closing paren", left_paren.line)
    }
}

fn get_arg<I>(tokens: &mut Peekable<I>, symbols: &mut SymbolTable) -> ASTNode
where
    I: Iterator<Item = Token>
{
    let token = consume(tokens);

    match token.token_type {
        TokenType::Integer => ASTNode::Int(token.content.parse::<i32>().unwrap()),
        TokenType::True => ASTNode::Bool(true),
        TokenType::False => ASTNode::Bool(false),
        TokenType::Literal => {
            let mut symbol = None;

            for s in symbols.table.iter() {
                if s.name == token.content {
                    symbol = Some(s.clone());
                    break;
                }
            }

            if let Some(s) = symbol {
                if let Some(_) = s.arg_types {
                    consume_fn_call(token.content, tokens, symbols)
                } else {
                    ASTNode::Var(Symbol::new_var(&token.content, s.symbol_type))
                }
            }
            else {
                throw_at(&format!("Unknown symbol {}", token.content), token.line);
            }
        }
        _ => throw_at(&format!("Invalid argument: {}", token.content), token.line)
    }
}

fn consume<'a, I>(tokens: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = Token>
{
    if let Some(token) = tokens.next() {
        return token;
    }

    throw("Expected another token, but reached end");
}

fn expect<'a, I>(tokens: &mut Peekable<I>, expected: TokenType) -> Token
where
    I: Iterator<Item = Token>,
{
    let token = consume(tokens);

    if token.token_type == expected {
        return token;
    } else {
        throw_at(&format!("Unexpected token {}, expected {expected:?}", token.content), token.line);
    }
}
