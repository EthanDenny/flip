use std::collections::HashMap;

use crate::types::{ASTNode, Buffer, T};

pub fn compile_expr(buf: &mut Buffer, node: &ASTNode) {
    match node {
        ASTNode::Call(name, args) => {
            let mut found_fn = false;
            for (fn_name, arg_types, _, body) in FUNC_TABLE {
                if &name == fn_name && compare_types(args, arg_types) {
                    found_fn = true;
                    body(buf, args);
                    break;
                }
            }
            if !found_fn { panic!("Unrecognized function: {name}") };
        }
        ASTNode::Int(_) => {
            buf.emit_instr(format!("movl ${}, %eax", immediate_repr(node)));
        },
        ASTNode::Bool(_) => {
            buf.emit_instr(format!("movb ${}, %al", immediate_repr(node)));
        }
    }
}

fn compare_types<'a>(args: &Vec<ASTNode>, goal_types: &[T]) -> bool {
    if  args.len() != goal_types.len() {
        return false;
    }

    let mut generics_map = HashMap::new();

    for (arg, goal_type) in args.into_iter().zip(goal_types.into_iter()) {
        let arg_type = match arg {
            ASTNode::Call(name, fn_args) => {
                let mut fn_type = None;
                for (fn_name, arg_types, return_type, _) in FUNC_TABLE {
                    if name == fn_name && compare_types(fn_args, arg_types) {
                        fn_type = Some(return_type);
                    }
                }
                if let Some(fn_type) = fn_type {
                    fn_type
                } else {
                    panic!("Unrecognized function: {name}");
                }
            },
            ASTNode::Int(_) => &T::Int,
            ASTNode::Bool(_) => &T::Bool,
        };

        if let T::Generic(generic_name) = goal_type {
            if let Some(&type_from_generic) = generics_map.get(&generic_name) {
                if arg_type != type_from_generic {
                    return false;
                }
            } else {
                generics_map.insert(generic_name, arg_type);
            }
        } else if arg_type != goal_type {
            return false;
        }
    }

    true
}

fn immediate_repr<'a>(node: &ASTNode) -> String {
    match node {
        ASTNode::Int(v) => format!("{v}"),
        ASTNode::Bool(v) => if *v { String::from("1") } else { String::from("0") }
        _ => String::new()
    }
}

const FUNC_TABLE: &[(&str, &[T], T, &(dyn Fn(&mut Buffer, &Vec<ASTNode>) -> ()))] = &[
    ("+", &[T::Int, T::Int], T::Int, &|buf, args| {
        match &args[..] {
            [ASTNode::Int(v1), ASTNode::Int(v2)] => {
                buf.emit_instr(format!("movl ${}, %eax", v1 + v2));
            }
            [ASTNode::Call(_, _), ASTNode::Int(v)] => {
                compile_expr(buf, &args[0]);
                buf.emit_instr(format!("addl ${v}, %eax"));
            }
            [ASTNode::Int(v), ASTNode::Call(_, _)] => {
                compile_expr(buf, &args[1]);
                buf.emit_instr(format!("addl ${v}, %eax"));
            }
            [ASTNode::Call(_, _), ASTNode::Call(_, _)] => {
                compile_expr(buf, &args[1]);
                buf.emit_instr(format!("movl %eax, -4(%rsp)"));
                compile_expr(buf, &args[0]);
                buf.emit_instr(format!("addl -4(%rsp), %eax"));
            }
            _ => panic!("Incorrect types")
        }
    }),
    ("-", &[T::Int, T::Int], T::Int, &|buf, args| {
        match &args[..] {
            [ASTNode::Int(v1), ASTNode::Int(v2)] => {
                buf.emit_instr(format!("movl ${}, %eax", v1 - v2));
            }
            [ASTNode::Call(_, _), ASTNode::Int(v)] => {
                compile_expr(buf, &args[0]);
                buf.emit_instr(format!("subl ${v}, %eax"));
            }
            [ASTNode::Int(v), ASTNode::Call(_, _)] => {
                compile_expr(buf, &args[1]);
                buf.emit_instr(format!("subl ${v}, %eax"));
            }
            [ASTNode::Call(_, _), ASTNode::Call(_, _)] => {
                compile_expr(buf, &args[1]);
                buf.emit_instr(format!("movl %eax, -4(%rsp)"));
                compile_expr(buf, &args[0]);
                buf.emit_instr(format!("subl -4(%rsp), %eax"));
            }
            _ => panic!("Incorrect types")
        }
    }),
    ("if", &[T::Bool, T::Generic("T"), T::Generic("T")], T::Generic("T"), &|buf, args| {
        match &args[0] {
            ASTNode::Bool(true) => {
                compile_expr(buf, &args[1]);
            }
            ASTNode::Bool(false) => {
                compile_expr(buf, &args[2]);
            }
            ASTNode::Call(_, _) => {
                compile_expr(buf, &args[0]);
                buf.emit_instr(format!("cmpl $0, %eax"));
                buf.emit_instr(format!("jne if_branch"));
                compile_expr(buf, &args[2]);
                buf.emit_instr(format!("jmp resume"));
                buf.emit("if_branch:\n");
                compile_expr(buf, &args[1]);
                buf.emit("resume:\n");
            }
            _ => panic!("Incorrect types")
        }
    })
];
