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
use std::process::Command;
use std::str::from_utf8;

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
    let mut symbols = compiler::table_from_inlines();
    let tokens = scanner::get_tokens(&code);

    let ast = parser::build_ast(tokens, &mut symbols);

    let mut file = File::create("build/out.c")?;

    for branch in ast {
        let buf = compiler::compile_expr(&branch, &mut symbols);
        file.write_all(buf.get().as_bytes())?;
    }

    Command::new("gcc")
        .arg("print.c")
        .arg("build/out.c")
        .arg("-o")
        .arg("build/out")
        .output()
        .expect("Failed to compile C");

    let output = Command::new("./build/out")
        .output()
        .expect("Failed to execute");

    print!("{}", from_utf8(&output.stdout).unwrap());

    Ok(())
}
