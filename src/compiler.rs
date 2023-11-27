use std::fs::{self, File};
use std::io::Write;
use std::slice::Iter;
use std::path::PathBuf;

use slab_tree::*;

use crate::scanner::scan;
use crate::token::{Token, TokenType};

#[derive(PartialEq, Debug)]
enum ASTNode<'a> {
    Root,
    List,
    Literal(&'a str),
    Atom(&'a str),
    Int(i32),
    Float(f64),
    String(&'a str),
}

fn add_ast_children<'a>(token_iter: &mut Iter<'a, Token<'a>>, parent: &mut NodeMut<ASTNode<'a>>) {
    while let Some(token) = token_iter.next() {
        match token.token_type {
            TokenType::LeftParen => {
                parent.append(ASTNode::List);
                add_ast_children(token_iter, &mut parent.last_child().unwrap());
            }
            TokenType::Literal => {
                parent.append(ASTNode::Literal(token.content));
            }
            TokenType::Atom => {
                parent.append(ASTNode::Atom(token.content));
            }
            TokenType::Int => {
                parent.append(ASTNode::Int(token.content.parse::<i32>().unwrap()));
            }
            TokenType::Float => {
                parent.append(ASTNode::Float(token.content.parse::<f64>().unwrap()));
            }
            TokenType::String => {
                parent.append(ASTNode::String(token.content));
            }
            TokenType::RightParen => break
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

/*
fn deconstruct_node<'a>(f: &mut File, node: NodeRef<'a, ASTNode>) -> ASTNode<'a> {
    match node.data() {
        ASTNode::List => {
            let mut args: Vec<ASTNode> = Vec::new();

            for c in node.children() {
                let n = deconstruct_node(f, c, i);

                match n {
                    ASTNode::List => {
                        args.push(ASTNode::Temp(*i));
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
                    ASTNode::Atom(name) => write(f, &format!("FlipType::Atom(\"{name}\")")),
                    ASTNode::Int(v) => write(f, &format!("FlipType::Int({v})")),
                    ASTNode::Temp(n) => write(f, &format!("t{n}")),
                    ASTNode::String(v) => write(f, &format!("FlipType::String(\"{v}\")")),
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
 */

fn deconstruct_node<'a>(node: NodeRef<'a, ASTNode>) -> String {
    let mut write_this = String::new();

    write_this.push_str("list!(");

    for c in node.children() {
        let ast_node = c.data();
        if ast_node == &ASTNode::List {
            write_this.push_str(&deconstruct_node(c));
        } else {
            write_this.push_str(&format!("FlipType::{ast_node:?}, "));
        }
    }

    write_this.push_str("FlipType::Ignore), ");

    write_this
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
    write(f, "    interpret(&[");

    // Convert AST into Rust

    let tokens: Vec<Token> = scan(&code);
    let ast: Tree<ASTNode> = build_ast(&tokens);

    if let Some(root) = ast.root() {
        for c in root.children() {
            write(f, "\n        ");
            write(f, deconstruct_node(c).as_str());
        }
    }

    // Closing brace

    write(f, "\n    ])\n}\n");
}
