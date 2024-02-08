mod compiler;
mod error;
mod parser;
mod scanner;
mod symbols;
mod types;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::symbols::{Symbol, SymbolTable};
use crate::types::{Buffer, T};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let path = PathBuf::from(&args[1]);
        let code = fs::read_to_string(&path).expect("Could not read file");
        
        if let Err(e) = compile(code) {
            println!("{e}");
        };
    } else {
        eprintln!("Usage: [path]");
    }
}

fn compile(code: String) -> std::io::Result<()>  {
    let mut buf = Buffer::new();
    let mut symbols = SymbolTable::new(vec![
        Symbol::new_fn("if", vec![T::Bool, T::gen("T"), T::gen("T")], T::gen("T")),
        Symbol::new_fn("if_else", vec![T::Bool, T::gen("T")], T::gen("T")),
        Symbol::new_fn("+", vec![T::Int, T::Int], T::Int),
        Symbol::new_fn("-", vec![T::Int, T::Int], T::Int),
    ]);

    buf.emit("    .globl fn_main\n");

    let tokens = scanner::get_tokens(&code);
    let ast = parser::build_ast(tokens, &mut symbols);

    for branch in ast {
        compiler::compile_expr(&mut buf, &branch, &mut symbols);
    }

    let mut file = File::create("build/out.s")?;
    file.write_all(buf.get().as_bytes())?;
    Ok(())
}
