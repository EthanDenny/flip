mod interpreter;
mod scanner;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};

use crate::interpreter::interpret;

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
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("Could not read line");
        interpret(line);
    }
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("Could not read file");
    interpret(source);
}
