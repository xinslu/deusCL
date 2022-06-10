use std::collections::HashMap;
use std::fmt;
pub use crate::token:: Token;

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
    VAR,
    CONCAT,
    RETURN,
    DEFUN
}

pub enum Error {
    Reason(String)
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reason(string) => {
                write!(f, "ERROR: {}", string)
            }
        }

    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reason(string) => {
                write!(f, "ERROR: {}", string)
            }
        }

    }
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



