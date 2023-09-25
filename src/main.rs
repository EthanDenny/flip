mod compiler;
mod scanner;
mod token;

use std::env;
use std::fs;
use std::path::PathBuf;

use crate::compiler::compile;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
        let path = PathBuf::from(&args[1]);
        let code = fs::read_to_string(&path).expect("Could not read file");
        compile(path, code);
    } else {
        eprintln!("Usage: [path]");
    }
}
