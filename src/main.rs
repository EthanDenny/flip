mod ast;
mod compiler;
mod error;
mod parser;
mod scanner;
mod symbols;
mod tokens;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let path = PathBuf::from(&args[1]);
        let code = fs::read_to_string(&path).expect("Could not read file");
        
        if let Err(e) = compile(code) {
            eprintln!("{e}");
        };
    } else {
        println!("Usage: [path]");
    }
}

fn compile(code: String) -> std::io::Result<()>  {
    let mut symbols = compiler::table_from_inlines();
    let tokens = scanner::get_tokens(&code);

    let ast = parser::build_ast(tokens, &mut symbols);

    let mut file = File::create("build/out.c")?;

    file.write_all(b"#include \"lambda.h\"\n\n")?;

    for branch in ast {
        let buf = compiler::compile_expr(&branch, &mut symbols);
        file.write_all(buf.get().as_bytes())?;
    }

    Ok(())
}
