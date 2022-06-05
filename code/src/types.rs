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
    LOOP,
    VAR
}

#[derive(Debug)]
pub enum Errors {
    Reason(String)
}

