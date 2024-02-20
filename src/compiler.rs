use std::fmt;

use crate::symbols::{Symbol, SymbolTable};
use crate::ast::{ASTNode, NodeType};

type InlineFnBody<'a> = &'a (dyn Fn(Vec<ASTNode>, &'a mut SymbolTable) -> String);
type InlineFn<'a> = (&'a str, Vec<NodeType>, NodeType, InlineFnBody<'a>);

pub struct Buffer {
    content: String
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { content: String::new() }
    }

    pub fn emit(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn emit_instr(&mut self, text: &str) {
        self.content.push_str(&format!("    {text}\n"));
    }

    pub fn get<'a>(self) -> String {
        self.content
    }
}

pub fn compile_expr<'a>(node: &ASTNode, symbols: &mut SymbolTable) -> Buffer {
    let mut buf = Buffer::new();

    match node {
        ASTNode::Let(s, v) => {
            buf.emit_instr(&format!("int {} = {};", s.name, compile_expr(v, symbols)));
        }
        ASTNode::Fn(name, args, _, body) => {
            buf.emit(&format!("int {name}("));

            if args.len() > 0 {
                let max_index = args.len() - 1;
                
                for i in 0..max_index {
                    buf.emit(&format!("int {}, ", args[i].name));
                }

                buf.emit(&format!("int {}", args[max_index].name));
            }
            
            buf.emit(") {\n");

            let max_index = body.len() - 1;

            for i in 0..max_index {
                buf.emit(&compile_expr(&body[i], symbols).get());
            }

            buf.emit_instr(&format!("return {};\n}}", compile_expr(&body[max_index], symbols).get()));
        }
        ASTNode::Call(name, args) => {
            if let Some(body) = get_inline_fn_body(name, args, symbols) {
                buf.emit(&body(args.to_vec(), symbols));
            } else if symbols.check_types(name, args) {
                let mut fn_call = format!("fn_{name}(");

                let max_index = args.len() - 1;
                
                for i in 0..max_index {
                    fn_call.push_str(&format!("{}, ", compile_expr(&args[i], symbols)));
                }
                
                fn_call.push_str(&format!("{})", compile_expr(&args[max_index], symbols)));

                buf.emit(&fn_call);
            } else {
                panic!("Could not find function \"{name}\" with args {args:?}");
            }
        }
        ASTNode::Var(_) |
        ASTNode::Int(_) |
        ASTNode::Bool(_) => {
            buf.emit(&format!("{}", node.imm_repr()));
        }
    }

    buf
}

fn get_inline_fn_body<'a>(name: &String, args: &Vec<ASTNode>, symbols: &mut SymbolTable) -> Option<InlineFnBody<'a>> {
    for f in get_inlines() {
        let (fn_name, arg_types, _, body) = f;
        if name == fn_name && symbols.compare_types(&args, &arg_types) {
            return Some(body);
        }
    }

    None
}

pub fn table_from_inlines() -> SymbolTable {
    let mut table = Vec::new();

    for (name, arg_types, return_type, _) in get_inlines() {
        table.push(Symbol::new_fn(name, arg_types, return_type))
    }

    SymbolTable::from(table)
}

fn get_inlines<'a>() -> Vec<InlineFn<'a>> {
    vec![
        ("+", vec![NodeType::Int, NodeType::Int], NodeType::Int, &|args, symbols| binary_op("+", args, symbols)),
        ("-", vec![NodeType::Int, NodeType::Int], NodeType::Int, &|args, symbols| binary_op("-", args, symbols)),
        ("*", vec![NodeType::Int, NodeType::Int], NodeType::Int, &|args, symbols| binary_op("*", args, symbols)),
        ("/", vec![NodeType::Int, NodeType::Int], NodeType::Int, &|args, symbols| binary_op("/", args, symbols)),
        ("mod", vec![NodeType::Int, NodeType::Int], NodeType::Int, &|args, symbols| binary_op("%", args, symbols)),

        ("==", vec![NodeType::gen("T"), NodeType::gen("T")], NodeType::Bool, &|args, symbols| binary_op("==", args, symbols)),
        ("!=", vec![NodeType::gen("T"), NodeType::gen("T")], NodeType::Bool, &|args, symbols| binary_op("!=", args, symbols)),
        (">", vec![NodeType::gen("T"), NodeType::gen("T")], NodeType::Bool, &|args, symbols| binary_op(">", args, symbols)),
        ("<", vec![NodeType::gen("T"), NodeType::gen("T")], NodeType::Bool, &|args, symbols| binary_op("<", args, symbols)),
        (">=", vec![NodeType::gen("T"), NodeType::gen("T")], NodeType::Bool, &|args, symbols| binary_op(">=", args, symbols)),
        ("<=", vec![NodeType::gen("T"), NodeType::gen("T")], NodeType::Bool, &|args, symbols| binary_op("<=", args, symbols)),

        ("and", vec![NodeType::Bool, NodeType::Bool], NodeType::Bool, &|args, symbols| binary_op("&&", args, symbols)),
        ("or", vec![NodeType::Bool, NodeType::Bool], NodeType::Bool, &|args, symbols| binary_op("||", args, symbols)),
        ("not", vec![NodeType::Bool], NodeType::Bool, &|args, symbols| {
            format!("(!{})",
                compile_expr(&args[0], symbols))
        }),

        ("if", vec![NodeType::Bool, NodeType::gen("T"), NodeType::gen("T")], NodeType::gen("T"), &|args, symbols| {
            format!("({} != 0 ? {} : {})",
                compile_expr(&args[0], symbols),
                compile_expr(&args[1], symbols),
                compile_expr(&args[2], symbols))
        })
    ]
}

fn binary_op<'a>(op: &'a str, args: Vec<ASTNode>, symbols: &mut SymbolTable) -> String {
    format!("({} {op} {})",
        compile_expr(&args[0], symbols),
        compile_expr(&args[1], symbols))
}
