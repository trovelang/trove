

mod lexer;
mod parser;

use crate::lexer::*;
use crate::parser::*;

fn main() {
    println!("Hello, world!");


    let lexer = ImplLexer{};
    let tokens = lexer.lex();
    println!("tokens = {:?}.", tokens);

    let parser = ImplParser{};
    let ast = parser.parse();
    println!("ast = {:?}.", ast);

}
