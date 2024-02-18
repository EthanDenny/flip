use crate::error::throw;
use crate::symbols::{Symbol, SymbolTable};
use crate::types::{ASTNode, Buffer, T};

type InlineFnBody<'a> = &'a (dyn Fn(Vec<ASTNode>, &'a mut SymbolTable) -> Buffer);
type InlineFn<'a> = (&'a str, Vec<T>, T, InlineFnBody<'a>);

pub fn compile_expr<'a>(node: &ASTNode, symbols: &mut SymbolTable) -> Buffer {
    let mut buf = Buffer::new();

    match node {
        ASTNode::Let(s, v) => {
            buf.emit_instr(&format!("int {} = {};", s.name, compile_expr(v, symbols)));
        }
        ASTNode::Fn(name, args, _, body) => {
            buf.emit(&format!("int {name}("));

            if args.len() > 0 {
                let max_args_index = args.len() - 1;
                
                for i in 0..max_args_index {
                    buf.emit(&format!("int {}, ", args[i].name));
                }

                buf.emit(&format!("int {}", args[max_args_index].name));
            }

            buf.emit(") {\n");


            let max_branch_index = body.len() - 1;

            for i in 0..max_branch_index {
                buf.emit(&compile_expr(&body[i], symbols).get());
            }

            buf.emit_instr(&format!("return {};\n}}", compile_expr(&body[max_branch_index], symbols).get()));
        }
        ASTNode::Call(name, args) => {
            if let Some(body) = get_inline_fn_body(name, args, symbols) {
                buf.emit(&body(args.to_vec(), symbols).get());
            } else if symbols.check_types(name, args) {
                let mut fn_call = format!("fn_{name}(");

                let max_args_index = args.len() - 1;
                
                for i in 0..max_args_index {
                    fn_call.push_str(&format!("{}, ", compile_expr(&args[i], symbols)));
                }
                
                fn_call.push_str(&format!("{})", compile_expr(&args[max_args_index], symbols)));

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

    SymbolTable::new(table)
}

fn get_inlines<'a>() -> Vec<InlineFn<'a>> {
    vec![
        ("+", vec![T::Int, T::Int], T::Int, &|args, symbols| {
            let mut buf = Buffer::new();

            buf.emit(&format!(
                "({} + {})",
                compile_expr(&args[0], symbols),
                compile_expr(&args[1], symbols)
            ));

            buf
        }),
        (">", vec![T::Int, T::Int], T::Bool, &|args, symbols| {
            let mut buf = Buffer::new();

            buf.emit(&format!(
                "({} > {})",
                compile_expr(&args[0], symbols),
                compile_expr(&args[1], symbols)
            ));

            buf
        }),
        ("-", vec![T::Int, T::Int], T::Int, &|args, symbols| {
            let mut buf = Buffer::new();

            buf.emit(&format!(
                "({} - {})",
                compile_expr(&args[0], symbols),
                compile_expr(&args[1], symbols)
            ));

            buf
        }),
        ("if", vec![T::Bool, T::gen("T"), T::gen("T")], T::gen("T"), &|args, symbols| {
            let mut buf = Buffer::new();

            match &args[0] {
                ASTNode::Bool(true) => {
                    compile_expr(&args[1], symbols);
                }
                ASTNode::Bool(false) => {
                    compile_expr(&args[2], symbols);
                }
                ASTNode::Call(_, _) |
                ASTNode::Var(_) => {
                    buf.emit(&format!(
                        "({} != 0 ? {} : {})",
                        compile_expr(&args[0], symbols),
                        compile_expr(&args[1], symbols),
                        compile_expr(&args[2], symbols),
                    ));
                }
                _ => throw("Incorrect types")
            }

            buf
        })
    ]
}
