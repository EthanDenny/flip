mod token;
mod parser;

use std::env;
use std::fs;
use std::io::{self, Write};

use crate::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: [path]");
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Could not read line");
        interpret(&line);
    }
}


fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Could not read file");
    interpret(&source);
}

fn interpret(code: &str) {
    let tokens: Vec<Token> = parser::parse(code);
    token::debug_tokens(&tokens);
    println!();
}
