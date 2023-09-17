use std::slice::Iter;

use slab_tree::*;

use crate::scanner::scan;
use crate::token::{Token, TokenType};

#[derive(Debug)]
enum ASTNode<'a> {
    Root,
    List,
    Literal(&'a str),
    Atom(&'a str),
    Int(i32),
    Float(f64),
    String(&'a str),
}

#[derive(Debug)]
enum ParseNode<'a> {
    Root,
    List,
    Lambda,
    Function(&'a str),
    Literal(&'a str),
    Atom(&'a str),
    Int(i32),
    Float(f64),
    String(&'a str),
}

fn add_ast_children<'a>(
    token_iter: &mut Iter<'a, Token<'a>>,
    parent: &mut NodeMut<ASTNode<'a>>,
)
{
    while let Some(token) = token_iter.next() {
        match token.token_type {
            TokenType::LeftParen => {
                parent.append(ASTNode::List);
                add_ast_children(token_iter, &mut parent.last_child().unwrap());
            }
            TokenType::RightParen => break,
            TokenType::Literal => {
                parent.append(ASTNode::Literal(token.content));
            },
            TokenType::Atom => {
                parent.append(ASTNode::Atom(token.content));
            },
            TokenType::Int => {
                parent.append(ASTNode::Int(token.content.parse::<i32>().unwrap()));
            },
            TokenType::Float => {
                parent.append(ASTNode::Float(token.content.parse::<f64>().unwrap()));
            },
            TokenType::String => {
                parent.append(ASTNode::String(token.content));
            },
        }
    }
}

fn build_ast<'a>(tokens: &'a [Token<'a>]) -> Tree<ASTNode<'a>> {
    let mut token_iter = tokens.iter();

    let mut tree: Tree<ASTNode> = TreeBuilder::new().with_root(ASTNode::Root).build();
    let mut root = tree.root_mut().expect("AST root doesn't exist");

    add_ast_children(&mut token_iter, &mut root);

    tree
}

fn add_parse_children<'a>(
    ast_parent: NodeRef<ASTNode<'a>>,
    parse_parent: &mut NodeMut<ParseNode<'a>>,
    skip_first: bool
)
{
    let mut children = ast_parent.children();

    if skip_first {
        children.next();
    }

    for ast_child in children {
        let parse_child = match ast_child.data() {
            ASTNode::Root => ParseNode::Root,
            ASTNode::List => {
                if let Some(n) = ast_child.first_child() {
                    if let ASTNode::Atom(name) = n.data() {
                        if name.to_string() == *"lambda" {
                            ParseNode::Lambda
                        } else {
                            ParseNode::Function(name)
                        }
                    } else {
                        ParseNode::List
                    }
                } else {
                    ParseNode::List
                }
            },
            ASTNode::Literal(x) => ParseNode::Literal(x),
            ASTNode::Atom(x) => ParseNode::Atom(x),
            ASTNode::Int(x) => ParseNode::Int(*x),
            ASTNode::Float(x) => ParseNode::Float(*x),
            ASTNode::String(x) => ParseNode::String(x),
        };

        match parse_child {
            ParseNode::Lambda | ParseNode::Function(_) => {
                let mut new_parent = parse_parent.append(parse_child);
                add_parse_children(ast_child, &mut new_parent, true);
            },
            ParseNode::List => {
                let mut new_parent = parse_parent.append(parse_child);
                add_parse_children(ast_child, &mut new_parent, false);
            },
            _ => {
                parse_parent.append(parse_child);
            }
        }
    }
}

fn build_parse_tree(ast: Tree<ASTNode>) -> Tree<ParseNode> {
    let mut tree: Tree<ParseNode> = TreeBuilder::new().with_root(ParseNode::Root).build();
    let mut root = tree.root_mut().expect("AST root doesn't exist");

    add_parse_children(ast.root().unwrap(), &mut root, false);

    tree
}

pub fn interpret(code: String) {
    let tokens: Vec<Token> = scan(&code);
    let ast: Tree<ASTNode> = build_ast(&tokens);
    let parse_tree: Tree<ParseNode> = build_parse_tree(ast);
    
    let mut s = String::new();
    parse_tree.write_formatted(&mut s).unwrap();
    print!("{s}");
}
