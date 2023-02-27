use std::{fs, process::exit};
mod lexer;

fn main() {
    // File to parse
    let content: Vec<_> = fs::read_to_string("./test.tkn")
        .unwrap_or_else(|err| {
            eprintln!("[Error] [FileRead]: {err} ");
            exit(1)
        })
        .chars()
        .collect();
    let mut lexer = lexer::Lexer::new(&content);
    lexer.parse();
}
