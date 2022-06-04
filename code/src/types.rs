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
    IDENTIFIER,
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
    STRINGLITERAL,
    List,
    AND,
    OR,
    NOT,
    NIL,
    MOD,
    LET,
    SET,
    PRINT,
    IF,
    LOOP
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

