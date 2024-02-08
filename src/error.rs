use std::process;

pub fn throw(error: &str) -> ! {
    eprintln!("Error: {error}");
    process::exit(1)
}

pub fn throw_at(error: &str, line: usize) -> ! {
    eprintln!("Error, line {line}: {error}");
    process::exit(1)
}
