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
            buf.emit_instr(&format!("#undef {}", s.name));
            buf.emit_instr(&format!("#define {} {}", s.name, compile_expr(v, symbols)));
        }
        ASTNode::Fn(name, args, return_type, body) => {
            buf.emit(&format!("// {name}\n\n"));

            if name == "main" {
                buf.emit(&format!("long fn_main("));
                emit_fn_args(&mut buf, args);
                buf.emit(") {\n");
                emit_fn_body(&mut buf, symbols, body);
            } else {
                // Forward declaration
                buf.emit(&format!("fn fn_{name}("));
                emit_fn_args(&mut buf, args);
                buf.emit(");\n\n");

                // "Real" function (called when evaluating)
                if let NodeType::List(_) = return_type.unwrap_fn() {
                    buf.emit(&format!("list eval_{name}(char* args) {{\n"));
                } else {
                    buf.emit(&format!("long eval_{name}(char* args) {{\n"));
                }

                for arg in args {
                    if let NodeType::List(_) = arg.symbol_type.unwrap_fn() {
                        buf.emit_instr(&format!("list {} = get_arg(args, list);", arg.name));
                    } else {
                        buf.emit_instr(&format!("long {} = get_arg(args, long);", arg.name));
                    }
                }

                emit_fn_body(&mut buf, symbols, body);

                // Lambda factory
                buf.emit(&format!("fn fn_{name}("));
                emit_fn_args(&mut buf, args);
                buf.emit(") {\n");

                if args.len() > 0 {
                    buf.emit_instr("int size = 0;");
                    for arg in args {
                        buf.emit_instr(&format!("size += sizeof({});", arg.name));
                    }

                    buf.emit_instr("char* args = malloc(size);");
                    for arg in args {
                        buf.emit_instr(&format!("add_arg(args, {});", arg.name));
                    }
    
                    buf.emit_instr(&format!("return lambda(eval_{name}, args - size);\n}}\n"));
                } else {
                    buf.emit_instr(&format!("return lambda(eval_{name}, NULL);\n}}\n"));
                }
            }
        }
        ASTNode::Call(name, args) => {
            if let Some(body) = get_inline_fn_body(name, args, symbols) {
                buf.emit(&body(args.to_vec(), symbols));
            } else if symbols.check_types(name, args) {
                let mut fn_call = format!("eval(fn_{name}(");

                let max_index = args.len() - 1;
                
                for i in 0..max_index {
                    fn_call.push_str(&format!("{}, ", compile_expr(&args[i], symbols)));
                }

                if let NodeType::List(_) = symbols.get_return_type(name, args).unwrap_fn() {
                    fn_call.push_str(&format!("{})).as_l", compile_expr(&args[max_index], symbols)));
                } else {
                    fn_call.push_str(&format!("{})).as_i", compile_expr(&args[max_index], symbols)));
                }

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

fn emit_fn_args(buf: &mut Buffer, args: &Vec<Symbol>) {
    if args.len() > 0 {
        let max_index = args.len() - 1;
        
        for i in 0..max_index {
            if let NodeType::List(_) = args[i].symbol_type.unwrap_fn() {
                buf.emit(&format!("list {}, ", args[i].name));
            } else {
                buf.emit(&format!("long {}, ", args[i].name));
            }
        }

        if let NodeType::List(_) = args[max_index].symbol_type.unwrap_fn() {
            buf.emit(&format!("list {}", args[max_index].name));
        } else {
            buf.emit(&format!("long {}", args[max_index].name));
        }
    }
}

fn emit_fn_body(buf: &mut Buffer, symbols: &mut SymbolTable, body: &Vec<ASTNode>) {
    let max_index = body.len() - 1;
    for i in 0..max_index {
        buf.emit(&compile_expr(&body[i], symbols).get());
    }
    buf.emit_instr(&format!("return {};\n}}\n", compile_expr(&body[max_index], symbols).get()));
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
            format!("({} ? {} : {})",
                compile_expr(&args[0], symbols),
                compile_expr(&args[1], symbols),
                compile_expr(&args[2], symbols))
        }),

        ("[Int]", vec![], NodeType::List(Box::new(NodeType::Int)), &|_, _| {
            format!("((list) NULL)")
        }),
        ("len", vec![NodeType::List(Box::new(NodeType::gen("T")))], NodeType::Int, &|args, symbols| {
            format!("(len({}))",
                compile_expr(&args[0], symbols))
        }),
        ("head", vec![NodeType::List(Box::new(NodeType::gen("T")))], NodeType::gen("T"), &|args, symbols| {
            format!("(({})->head)",
                compile_expr(&args[0], symbols))
        }),
        ("tail", vec![NodeType::List(Box::new(NodeType::gen("T")))], NodeType::List(Box::new(NodeType::gen("T"))), &|args, symbols| {
            format!("(({})->tail)",
                compile_expr(&args[0], symbols))
        }),
        // List concatenation
        ("++", vec![NodeType::List(Box::new(NodeType::gen("T"))), NodeType::gen("T")], NodeType::List(Box::new(NodeType::gen("T"))), &|args, symbols| {
            format!("push({}, {})",
                compile_expr(&args[0], symbols),
                compile_expr(&args[1], symbols))
        }),
        ("is_null", vec![NodeType::List(Box::new(NodeType::gen("T")))], NodeType::Bool, &|args, symbols| {
            format!("({} == NULL)",
                compile_expr(&args[0], symbols))
        })
    ]
}

fn binary_op<'a>(op: &'a str, args: Vec<ASTNode>, symbols: &mut SymbolTable) -> String {
    format!("({} {op} {})",
        compile_expr(&args[0], symbols),
        compile_expr(&args[1], symbols))
}
