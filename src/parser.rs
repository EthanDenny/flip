use std::iter::Peekable;

use crate::types::{ASTNode, Token, TokenType};

pub fn build_ast(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut tokens = tokens.into_iter().peekable();
    let mut tree: Vec<ASTNode> = vec![];

    while let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::Literal => {
                tree.push(build_fn_call(token.content, &mut tokens));
            }
            _ => {}
        }
    }

    tree
}

fn build_fn_call<'a, I>(name: String, tokens: &mut Peekable<I>) -> ASTNode
where
    I: Iterator<Item = Token>
{
    expect(tokens, TokenType::LeftParen);

    if let Some(arg_or_paren) = tokens.peek() {
        if arg_or_paren.token_type == TokenType::RightParen {
            consume(tokens);
            ASTNode::Call(name, Vec::new())
        } else {
            let mut args = vec![get_arg(tokens)];

            while let Some(token) = tokens.peek() {
                if token.token_type == TokenType::RightParen {
                    tokens.next();
                    break;
                } else {
                    expect(tokens, TokenType::Comma);
                    args.push(get_arg(tokens));
                }
            }

            ASTNode::Call(name, args)
        }
    } else {
        panic!("Expected closing paren");
    }
}

fn get_arg<I>(tokens: &mut Peekable<I>) -> ASTNode
where
    I: Iterator<Item = Token>
{
    let token = consume(tokens);

    match token.token_type {
        TokenType::Integer => ASTNode::Int(token.content.parse::<i32>().unwrap()),
        TokenType::Literal => {
            match token.content {
                x if x == "true" || x == "false" => ASTNode::Bool(x == "true"),
                _  => build_fn_call(token.content, tokens)
            }
        }
        _ => { panic!("Invalid argument: {}", token.content) }
    }
}

fn consume<'a, I>(tokens: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = Token>
{
    if let Some(token) = tokens.next() {
        return token;
    }

    panic!("Expected another token, but reached end");
}

fn expect<'a, I>(tokens: &mut Peekable<I>, expected: TokenType) -> Token
where
    I: Iterator<Item = Token>,
{
    if let Some(token) = tokens.peek() {
        if token.token_type == expected {
            return tokens.next().unwrap();
        }
    }

    panic!("Unexpected token, expected {expected:?}");
}
