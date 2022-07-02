pub struct Position {
    index: u32, // 0-based
    line: u32 // 0-based
}

pub enum Type {
    NUMBER(String),
    VAR(String)
}

pub struct Token {
    typ: Type
}

pub trait Lexer {
}


pub struct ImplLexer {
}


impl Lexer for ImplLexer {

}