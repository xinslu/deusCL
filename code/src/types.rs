use std::collections::HashMap;
pub type Result<T> = std::result::Result<T, Errors>;
pub type DeusResult = Result<Box<Token>>;

pub enum TokenFunction {
    Builtin(),
    Lambda(),
}


pub enum Token {
    Fun(TokenFunction),
    Number(i64),
    Symbol(String),
    StringLiteral(String)
}


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


