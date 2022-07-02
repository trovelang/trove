
#[derive(Debug)]
pub struct Position {
    index: u32, // 0-based
    line: u32 // 0-based
}

#[derive(Debug)]
pub enum Type {
    NUMBER(String),
    VAR(String)
}

#[derive(Debug)]
pub struct Token {
    typ: Type
}

pub trait Lexer {
    fn lex(&self) -> Vec<Token>;
}


pub struct ImplLexer {
}


impl Lexer for ImplLexer {
    fn lex(&self) -> Vec<Token> {
        let mut v = Vec::new();
        v.push(Token{typ: Type::NUMBER("123".to_string())});
        return v;
    }
}