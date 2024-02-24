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
    
        let mut generics_map = HashMap::new();
    
        for (arg, goal_type) in args.into_iter().zip(goal_types.into_iter()) {
            let arg_type = self.get_node_type(arg).unwrap_fn();

            match goal_type {
                NodeType::Generic(generic_name) => {
                    if let Some(type_from_generic) = generics_map.get(&generic_name) {
                        if arg_type != *type_from_generic {
                            return false;
                        }
                    } else {
                        generics_map.insert(generic_name, arg_type);
                    }
                }
                _ => {
                    if arg_type != *goal_type {
                        return false;
                    }
                }
            }
        }
    
        true
    }

    pub fn get_node_type<'a>(&self, node: &ASTNode) -> NodeType {
        match node {
            ASTNode::Fn(_, _, return_type, _) => NodeType::Fn(Box::new(return_type.clone())),
            ASTNode::Let(_, _) => throw("Cannot pass a let-binding as an argument"),
            ASTNode::Call(name, args) => {
                let return_type = self.get_return_type(name, args);

                if let NodeType::Generic(generic_name) = &return_type {
                    let arg_types = self.get_arg_types(name, args);
            
                    for (arg_type, arg) in arg_types.iter().zip(args.iter()) {
                        if &return_type == arg_type {
                            return self.get_node_type(arg);
                        }
                    }
            
                    throw(&format!("Could not resolve generic {generic_name}"));
                } else {
                    return_type
                }
            },
            ASTNode::Var(s) => s.symbol_type.clone(),
            ASTNode::Int(_) => NodeType::Int,
            ASTNode::Bool(_) => NodeType::Bool,
        }
    }
}
