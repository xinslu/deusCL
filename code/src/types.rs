use std::collections::HashMap;



pub use crate::token:: {
    Token
};
pub enum TokenFunction {
    Builtin(),
    Lambda(),
}


pub enum TokenTypes {
    Fun(TokenFunction),
    Number,
    Symbol,
    StringLiteral,
    List,
}

#[derive(Debug)]
pub enum Errors {
    FunctionFormat,
    NotANumber,
    NumArguments(usize, usize),
    ParseError(String),
    ReadLineError(String),
    WrongType(String, String),
    FunctionNoDefined(String),
    DivisionByZero,
}



struct Environment {
  data: HashMap<String, Token>,
}

