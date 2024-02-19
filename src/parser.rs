use crate::error::{throw, throw_at};
use crate::symbols::{Symbol, SymbolTable};
use crate::ast::{ASTNode, NodeType};
use crate::tokens::{Token, TokenType, TokensList};

pub fn build_ast(token_vec: Vec<Token>, symbols: &mut SymbolTable) -> Vec<ASTNode> {
    let mut tokens = TokensList::from(token_vec);
    let mut tree: Vec<ASTNode> = vec![];

    while tokens.peek().is_some() {
        tree.push(consume_fn(&mut tokens, symbols));
    }

    tree
}

fn consume_fn(tokens: &mut TokensList, symbols: &mut SymbolTable) -> ASTNode {
    let mut name = String::from("fn_");
    let name_token = tokens.expect(TokenType::Literal);
    name.push_str(&name_token.content);

    tokens.expect(TokenType::LeftParen);

    let args = consume_fn_args(tokens);
    let arg_types = args.iter()
        .map(|arg| arg.symbol_type.clone())
        .collect();

    tokens.expect(TokenType::RightParen);

    // No guaruntee this is actually what is returned by the function,
    // currently, the programmer must be trusted
    let return_type = consume_fn_return(tokens);

    symbols.insert(Symbol::new_fn(&name_token.content, arg_types, return_type.clone()));

    let mut scoped_symbols = symbols.clone();
    scoped_symbols.insert_vec(args.clone());

    let body = consume_block(tokens, &mut scoped_symbols);

    ASTNode::Fn(name, args, return_type, body)
}

fn consume_fn_args(tokens: &mut TokensList) -> Vec<Symbol> {
    let mut args = Vec::new();

    while let Some(token) = tokens.peek() {
        if token.token_type != TokenType::RightParen {
            if token.token_type == TokenType::Comma {
                tokens.consume();
            }

            let arg_name = tokens.expect(TokenType::Literal).content;
            tokens.expect(TokenType::Colon);
            let arg_type = tokens.expect(TokenType::Literal).content;
            args.push(Symbol::new_var(&arg_name, parse_type(arg_type)));

        } else {
            break;
        }
    }

    args
}

fn consume_fn_return(tokens: &mut TokensList) -> NodeType {
    if let Some(token) = tokens.peek() {
        if token.token_type == TokenType::Colon {
            tokens.consume();
            let type_name = tokens.expect(TokenType::Literal).content;
            parse_type(type_name)
        } else {
            NodeType::None
        }
    } else {
        throw("Expected return type or block, got end")
    }
}

// Statements surrounded by braces: { ... }
// "symbols" should be a SymbolTable created for ONLY this block
fn consume_block(tokens: &mut TokensList, symbols: &mut SymbolTable) -> Vec<ASTNode> {
    tokens.expect(TokenType::LeftBrace);

    let mut calls = Vec::new();

    while let Some(token) = tokens.peek() {
        match token.token_type {
            TokenType::RightBrace => {
                tokens.consume();
                break;
            }
            TokenType::Let => {
                tokens.consume();
                calls.push(consume_let(tokens, symbols));
            }
            _ => {
                calls.push(parse_node(tokens, symbols));
            }
        }
    }

    calls
}

fn consume_let(tokens: &mut TokensList, symbols: &mut SymbolTable) -> ASTNode {
    tokens.expect(TokenType::LeftParen);
    let name = tokens.expect(TokenType::Literal).content;    
    tokens.expect(TokenType::Colon);
    let symbol_type = parse_type(tokens.expect(TokenType::Literal).content);
    tokens.expect(TokenType::Comma);
    let value = parse_node(tokens, symbols);
    tokens.expect(TokenType::RightParen);
    
    let symbol = Symbol::new_var(&name, symbol_type);
    symbols.insert(symbol.clone());
    
    ASTNode::Let(symbol, Box::new(value))
}

fn consume_call(name: String, tokens: &mut TokensList, symbols: &mut SymbolTable) -> ASTNode {
    let left_paren = tokens.expect(TokenType::LeftParen);

    if let Some(token) = tokens.peek() {
        if token.token_type == TokenType::RightParen {
            tokens.consume();
            ASTNode::Call(name, Vec::new())
        } else {
            let mut args = vec![parse_node(tokens, symbols)];

            while let Some(token) = tokens.peek() {
                if token.token_type == TokenType::RightParen {
                    tokens.next();
                    break;
                } else {
                    tokens.expect(TokenType::Comma);
                    args.push(parse_node(tokens, symbols));
                }
            }

            ASTNode::Call(name, args)
        }
    } else {
        throw_at("Expected closing paren", left_paren.line)
    }
}

fn parse_node(tokens: &mut TokensList, symbols: &mut SymbolTable) -> ASTNode {
    let token = tokens.consume();

    match token.token_type {
        TokenType::Integer => ASTNode::Int(token.content.parse::<i32>().unwrap()),
        TokenType::True => ASTNode::Bool(true),
        TokenType::False => ASTNode::Bool(false),
        TokenType::Literal => {
            let mut symbol = None;

            for s in symbols.iter() {
                if s.name == token.content {
                    symbol = Some(s.clone());
                    break;
                }
            }

            if let Some(s) = symbol {
                if let Some(_) = s.arg_types {
                    consume_call(token.content, tokens, symbols)
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

fn parse_type(type_name: String) -> NodeType {
    match type_name.as_ref() {
        "Int" => NodeType::Int,
        "Bool" => NodeType::Bool,
        "Fn" => NodeType::Fn,
        "None" => NodeType::None,
        _ => NodeType::Generic(type_name)
    }
}
