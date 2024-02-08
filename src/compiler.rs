use crate::error::throw;
use crate::symbols::SymbolTable;
use crate::types::{ASTNode, Buffer, T};

type InlineFnBody<'a> = &'a (dyn Fn(&'a mut Buffer, Vec<ASTNode>, &'a mut SymbolTable) -> ());
type InlineFn<'a> = (&'a str, Vec<T>, T, InlineFnBody<'a>);

pub fn compile_expr<'a>(buf: &mut Buffer, node: &ASTNode, symbols: &mut SymbolTable) {
    match node {
        ASTNode::Fn(name, _, _, body) => {
            buf.emit(&format!("\n{}:\n", name));

            buf.emit_instr("pushq %rbp");
            buf.emit_instr("pushq %rsi");
            buf.emit_instr("pushq %rdi");

            for branch in body {
                compile_expr(buf, &branch, symbols);
            }

            buf.emit_instr("popq %rdi");
            buf.emit_instr("popq %rsi");
            buf.emit_instr("popq %rbp");

            buf.emit_instr("ret");
        }
        ASTNode::Call(name, args) => {
            if let Some(body) = get_inline_fn_body(name, args, symbols) {
                body(buf, args.to_vec(), symbols);
            } else if symbols.check_types(name, args) {
                buf.emit_instr("pushq %rax");
                buf.emit_instr("pushq %rcx");
                buf.emit_instr("pushq %rdx");

                compile_expr(buf, &args[0], symbols);
                buf.emit_instr(&format!("call fn_{name}"));
                buf.emit_instr("movl %eax, %ebp");

                buf.emit_instr("popq %rdx");
                buf.emit_instr("popq %rcx");
                buf.emit_instr("popq %rax");

                buf.emit_instr("movl %ebp, %eax");
            } else {
                panic!("Could not find function \"{name}\"");
            }
        }
        ASTNode::Var(_) => {}
        ASTNode::Int(_) => {
            buf.emit_instr(&format!("movl ${}, %eax", node.imm_repr()));
        },
        ASTNode::Bool(_) => {
            buf.emit_instr(&format!("movb ${}, %al", node.imm_repr()));
        }
    }
}

fn get_inline_fn_body<'a>(name: &String, args: &Vec<ASTNode>, symbols: &mut SymbolTable) -> Option<InlineFnBody<'a>> {
    let inlines: Vec<InlineFn> = vec![
        ("+", vec![T::Int, T::Int], T::Int, &|buf, args, symbols| {
            match &args[..] {
                [ASTNode::Int(v1), ASTNode::Int(v2)] => {
                    buf.emit_instr(&format!("movl ${}, %eax", v1 + v2));
                }
                [ASTNode::Call(_, _), ASTNode::Int(v)] => {
                    compile_expr(buf, &args[0], symbols);
                    buf.emit_instr(&format!("addl ${v}, %eax"));
                }
                [ASTNode::Int(v), ASTNode::Call(_, _)] => {
                    compile_expr(buf, &args[1], symbols);
                    buf.emit_instr(&format!("addl ${v}, %eax"));
                }
                [ASTNode::Call(_, _), ASTNode::Call(_, _)] => {
                    compile_expr(buf, &args[1], symbols);
                    buf.emit_instr(&format!("movl %eax, -4(%rsp)"));
                    compile_expr(buf, &args[0], symbols);
                    buf.emit_instr(&format!("addl -4(%rsp), %eax"));
                },
                [ASTNode::Var(_), ASTNode::Int(v)] | 
                [ASTNode::Int(v), ASTNode::Var(_)] => {
                    buf.emit_instr(&format!("addl ${v}, %eax"));
                }
                _ => throw("Incorrect types")
            }
        }),
        ("-", vec![T::Int, T::Int], T::Int, &|buf, args, symbols| {
            match &args[..] {
                [ASTNode::Int(v1), ASTNode::Int(v2)] => {
                    buf.emit_instr(&format!("movl ${}, %eax", v1 - v2));
                }
                [ASTNode::Call(_, _), ASTNode::Int(v)] => {
                    compile_expr(buf, &args[0], symbols);
                    buf.emit_instr(&format!("subl ${v}, %eax"));
                }
                [ASTNode::Int(v), ASTNode::Call(_, _)] => {
                    compile_expr(buf, &args[1], symbols);
                    buf.emit_instr(&format!("subl ${v}, %eax"));
                }
                [ASTNode::Call(_, _), ASTNode::Call(_, _)] => {
                    compile_expr(buf, &args[1], symbols);
                    buf.emit_instr(&format!("movl %eax, -4(%rsp)"));
                    compile_expr(buf, &args[0], symbols);
                    buf.emit_instr(&format!("subl -4(%rsp), %eax"));
                },
                [ASTNode::Var(_), ASTNode::Int(v)] | 
                [ASTNode::Int(v), ASTNode::Var(_)] => {
                    buf.emit_instr(&format!("subl ${v}, %eax"));
                }
                _ => throw("Incorrect types")
            }
        }),
        ("if", vec![T::Bool, T::gen("T"), T::gen("T")], T::gen("T"), &|buf, args, symbols| {
            match &args[0] {
                ASTNode::Bool(true) => {
                    compile_expr(buf, &args[1], symbols);
                }
                ASTNode::Bool(false) => {
                    compile_expr(buf, &args[2], symbols);
                }
                ASTNode::Call(_, _) => {
                    let if_branch = buf.get_label();
                    let resume = buf.get_label();

                    compile_expr(buf, &args[0], symbols);
                    buf.emit_instr(&format!("cmpl $0, %eax"));
                    buf.emit_instr(&format!("jne {if_branch}"));
                    compile_expr(buf, &args[2], symbols);
                    buf.emit_instr(&format!("jmp {resume}"));
                    buf.emit(&format!("{if_branch}:\n"));
                    compile_expr(buf, &args[1], symbols);
                    buf.emit(&format!("{resume}:\n"));
                }
                ASTNode::Var(_) => {
                    let if_branch = buf.get_label();
                    let resume = buf.get_label();

                    buf.emit_instr(&format!("cmpl $0, %eax"));
                    buf.emit_instr(&format!("jne {if_branch}"));
                    compile_expr(buf, &args[2], symbols);
                    buf.emit_instr(&format!("jmp {resume}"));
                    buf.emit(&format!("{if_branch}:\n"));
                    compile_expr(buf, &args[1], symbols);
                    buf.emit(&format!("{resume}:\n"));
                }
                _ => throw("Incorrect types")
            }
        })
    ];

    for f in inlines {
        let (fn_name, arg_types, _, body) = f;
        if name == fn_name && symbols.compare_types(&args, &arg_types) {
            return Some(body);
        }
    }

    None
}
