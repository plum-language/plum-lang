use std::fs;

use logos::Logos;
mod lexer;
mod parser;

fn main() {
    let src = fs::read_to_string("examples/example2.plum").expect("Shit happens");
    let mut lex = lexer::Token::lexer(src.as_str()).peekable();

    parser::parse(&mut lex);
}
