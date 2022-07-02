
#[derive(Debug)]
pub enum Type{
    PROGRAM
}

#[derive(Debug)]
pub struct Ast{
    typ: Type
}

pub trait Parser{
    fn parse(&self) -> Ast;
}

pub struct ImplParser{

}

impl Ast {
    fn new(ast: Box<Ast>) -> Ast {
        return Ast{typ: ast.typ};
    }
}


impl Parser for ImplParser{
    fn parse(&self) -> Ast{
        let ast = Box::new(Ast{typ: Type::PROGRAM});
        return Ast::new(ast);
    }
}