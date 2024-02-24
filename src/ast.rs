use std::fmt;

use crate::symbols::Symbol;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Fn(String, Vec<Symbol>, NodeType, Vec<ASTNode>),
    Call(String, Vec<ASTNode>),
    Let(Symbol, Box<ASTNode>),
    Var(Symbol),
    Int(i32),
    Bool(bool),
}

impl ASTNode {
    pub fn imm_repr(&self) -> String {
        match self {
            ASTNode::Int(v) => format!("{v}"),
            ASTNode::Bool(v) => if *v { String::from("1") } else { String::from("0") }
            ASTNode::Var(s) => s.name.clone(),
            _ => String::new()
        }
    }
}

// Types of function arguments and returns
#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Int,
    Bool,
    Fn(Box<NodeType>),
    None,
    Generic(String)
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Int => write!(f, "Int"),
            NodeType::Bool => write!(f, "Bool"),
            NodeType::Fn(return_type) => write!(f, "Fn({return_type})"),
            NodeType::None => write!(f, "None"),
            NodeType::Generic(generic_name) => write!(f, "Generic({generic_name})"),
        }
    }
}

impl<'a> NodeType {
    pub fn gen(name: &'a str) -> NodeType {
        NodeType::Generic(name.to_string())
    }

    pub fn unwrap_fn(&self) -> NodeType {
        match &self {
            NodeType::Fn(arg_type) => *arg_type.clone(),
            _ => self.clone()
        }
    }
}
