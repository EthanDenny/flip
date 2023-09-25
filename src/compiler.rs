use std::fs::{self, File};
use std::io::Write;
use std::slice::Iter;
use std::path::PathBuf;

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

#[derive(Debug, Clone, Copy)]
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
    Temp(usize),
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

fn deconstruct_node<'a>(f: &mut File, node: NodeRef<'a, ParseNode>, i: &mut usize) -> ParseNode<'a> {
    match node.data() {
        ParseNode::Function(name) => {
            let mut args: Vec<ParseNode> = Vec::new();

            for c in node.children() {
                let n = deconstruct_node(f, c, i);

                match n {
                    ParseNode::Function(_) => {
                        args.push(ParseNode::Temp(*i));
                    }
                    _ => {
                        args.push(n);
                    }
                }
            }

            *i += 1;

            write(f, &format!("    let t{i} = flip.call_fn(\"{name}\", vec!["));

            let mut args_iter = args.iter().peekable();

            while let Some(a) = args_iter.next() {
                match a {
                    ParseNode::Atom(name) => write(f, &format!("FlipType::Atom(\"{name}\")")),
                    ParseNode::Int(v) => write(f, &format!("FlipType::Int({v})")),
                    ParseNode::Temp(n) => write(f, &format!("t{n}")),
                    ParseNode::String(v) => write(f, &format!("FlipType::String(\"{v}\")")),
                    _ => write(f, &format!("{:?}", a)),
                }
                if args_iter.peek().is_some() {
                    write(f, ", ");
                }
            }

            write(f, "]);\n");

            *node.data()
        }
        _ => {
            *node.data()
        }
    }
}

fn write(f: &mut File, data: &str) {
    f.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn compile(mut path: PathBuf, code: String) {
    // Create file buffer

    path.set_extension("rs");
    
    let f = &mut File::create(path).expect("Unable to create file");

    // Write the core Flip functions

    let flip = fs::read_to_string("src/flip.rs").expect("Could not read flip.rs");

    write(f, &flip);

    // Main declaration before the actual code

    write(f, "\n#[allow(unused_variables)]\n");
    write(f, "fn main() {\n");
    write(f, "    let mut flip = Flip::new();\n\n");

    // Convert AST into Rust

    let tokens: Vec<Token> = scan(&code);
    let ast: Tree<ASTNode> = build_ast(&tokens);
    let parse_tree: Tree<ParseNode> = build_parse_tree(ast);

    let mut i = 0;

    if let Some(root) = parse_tree.root() {
        for c in root.children() {
            deconstruct_node(f, c, &mut i);
        }
    }

    // Closing brace

    write(f, "}\n");
}
