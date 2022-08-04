mod lexer;
use lexer::Lexer;

use std::io;

fn main() {
    println!("Input:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // let input = "123\n+456\n-789/123";

    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();

    Lexer::print_tokens(tokens);
}
