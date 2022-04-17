use std::collections::HashMap;



pub use crate::token:: {
    Token
};

#[derive(Clone, Debug, Copy)]
pub enum TokenFunction {
    Builtin(),
    Lambda(),
}

#[derive(Clone, Debug, Copy)]
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
    MAX,
    MIN,
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

