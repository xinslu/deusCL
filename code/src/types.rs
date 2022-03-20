use std::collections::HashMap;



pub use crate::token:: {
    Token
};

#[derive(Debug)]
pub enum TokenFunction {
    Builtin(),
    Lambda(),
}

#[derive(Debug)]
pub enum TokenTypes {
    Fun(TokenFunction),
    Number,
    Symbol,
    LeftParen,
    RightParen,
    MINUS,
    PLUS,
    SLASH,
    SLASHEQUAL,
    STAR,
    EQUAL,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
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

