use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use flip::{compiler, parser, scanner};

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

fn compile(code: String) -> std::io::Result<()> {
    let mut symbols = compiler::table_from_inlines();
    let tokens = scanner::get_tokens(&code);

    let ast = parser::build_ast(tokens, &mut symbols);

    let mut file = File::create("build/out.c")?;

    file.write_all(format!("{INCLUDES}\n\n").as_bytes())?;

    for branch in ast {
        let buf = flip::compiler::compile_expr(&branch, &mut symbols);
        file.write_all(buf.get().as_bytes())?;
    }

    file.write_all(C_MAIN.as_bytes())?;

    Ok(())
}

const INCLUDES: &str = "\
#include <stdio.h>
#include \"flip.h\"";

const C_MAIN: &str = "\
// C main

int main() {
    printf(\"%ld\\n\", fn_main());
    return 0;
}
";
