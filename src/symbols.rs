use std::collections::HashMap;

use crate::error::throw;
use crate::ast::{ASTNode, NodeType};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: NodeType,
    pub arg_types: Option<Vec<NodeType>>
}

impl<'a> Symbol {
    pub fn new_var(name: &'a str, symbol_type: NodeType) -> Symbol {
        Symbol { name: name.to_string(), symbol_type, arg_types: None }
    }

    pub fn new_fn(name: &'a str, arg_types: Vec<NodeType>, return_type: NodeType) -> Symbol {
        Symbol { name: name.to_string(), symbol_type: return_type, arg_types: Some(arg_types) }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    pub table: Vec<Symbol>
}

impl SymbolTable {
    pub fn from(table: Vec<Symbol>) -> SymbolTable {
        SymbolTable { table }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Symbol> {
        self.table.iter()
    }

    pub fn insert(&mut self, s: Symbol) {
        self.table.push(s);
    }

    pub fn insert_vec(&mut self, v: &Vec<Symbol>) {
        self.table.extend_from_slice(v);
    }

    pub fn check_types(&self, name: &String, args: &Vec<ASTNode>) -> bool {
        self.find_fn(name, args).is_some()
    }

    pub fn get_arg_types(&self, name: &String, args: &Vec<ASTNode>) -> Vec<NodeType> {
        if let Some(s) = self.find_fn(name, args) {
            return s.arg_types.clone().unwrap();
        } else {
            panic!("Could not find symbol {name}");
        }
    }


    pub fn get_return_type(&self, name: &String, args: &Vec<ASTNode>) -> NodeType {
        if let Some(s) = self.find_fn(name, args) {
            s.symbol_type.clone()
        } else {
            panic!("Could not find symbol {name}");
        }
    }

    fn find_fn(&self, name: &String, args: &Vec<ASTNode>) -> Option<&Symbol> {
        for s in self.table.iter() {
            if let Some(arg_types) = &s.arg_types {
                if *name == s.name && self.compare_types(&args, arg_types) {
                    return Some(s);
                }
            }
        }

        None
    }

    pub fn compare_types(&self, args: &Vec<ASTNode>, goal_types: &Vec<NodeType>) -> bool {
        if  args.len() != goal_types.len() {
            return false;
        }
    
        let mut generics = HashMap::new();
    
        for (arg, goal_type) in args.into_iter().zip(goal_types.into_iter()) {
            let arg_type = self.get_node_type(arg);
            if !Self::compare(&arg_type, goal_type, &mut generics) {
                return false;
            }
        }
    
        true
    }

    fn compare<'a>(a: &'a NodeType, b: &'a NodeType, generics: &mut HashMap<String, NodeType>) -> bool {
        match (a, b) {
            (NodeType::Int, NodeType::Int) => true,
            (NodeType::Bool, NodeType::Bool) => true,
            (NodeType::None, NodeType::None) => true,
            (NodeType::Generic(a), NodeType::Generic(b)) => a == b,
            (NodeType::Generic(g), t) |
            (t, NodeType::Generic(g)) => {
                if let Some(type_from_generic) = generics.get(g) {
                    let type_from_generic = type_from_generic.unwrap_fn();
                    if !Self::compare(t, &type_from_generic, generics) {
                        return false;
                    }
                } else {
                    generics.insert(g.to_string(), t.clone());
                }

                true
            }
            (NodeType::List(a), NodeType::List(b)) => Self::compare(a, b, generics),
            (NodeType::Fn(_), _) |
            (_, NodeType::Fn(_)) => {
                Self::compare(&a.unwrap_fn(), &b.unwrap_fn(), generics)
            }
            _ => false
        }
    }

    pub fn get_node_type<'a>(&self, node: &ASTNode) -> NodeType {
        match node {
            ASTNode::Fn(_, _, return_type, _) => NodeType::Fn(Box::new(return_type.clone())),
            ASTNode::Let(_, _) => throw("Cannot pass a let-binding as an argument"),
            ASTNode::Call(name, args) => {
                let return_type = self.get_return_type(name, args);
                let arg_types = self.get_arg_types(name, args);
                let mut generics = HashMap::new();

                for (arg_type, arg) in arg_types.iter().zip(args.iter()) {
                    let mut left_type = arg_type.unwrap_fn();
                    let mut right_type = self.get_node_type(arg).unwrap_fn();

                    loop {
                        match (left_type, &right_type) {
                            (NodeType::List(a), NodeType::List(b)) => {
                                left_type = *a.clone();
                                right_type = *b.clone();
                            }
                            (NodeType::Generic(g), _) => {
                                if !generics.contains_key(&g) {
                                    generics.insert(g, right_type);
                                }
                                break;
                            }
                            _ => break
                        }
                    }
                }

                fn wrap(return_type: NodeType, generics: HashMap<String, NodeType>) -> NodeType {
                    match return_type {
                        NodeType::List(inner) => {
                            NodeType::List(Box::new(wrap(*inner, generics)))
                        }
                        NodeType::Generic(g) => {
                            generics.get(&g).unwrap().clone()
                        }
                        _ => return_type
                    }
                }

                wrap(return_type, generics)
            },
            ASTNode::Var(s) => s.symbol_type.clone(),
            ASTNode::Int(_) => NodeType::Int,
            ASTNode::Bool(_) => NodeType::Bool,
        }
    }
}
