mod compiler;
mod parser;
mod scanner;
mod types;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::types::Buffer;

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

    buf.emit("    .globl _main\n_main:\n");

    let tokens = scanner::get_tokens(&code);
    let ast = parser::build_ast(tokens);

    for branch in ast.iter() {
        compiler::compile_expr(&mut buf, &branch);
    }

    buf.emit_instr(String::from("ret"));

    let mut file = File::create("build/out.s")?;
    file.write_all(buf.get().as_bytes())?;
    Ok(())
}
