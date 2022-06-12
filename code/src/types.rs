use std::collections::HashMap;
use std::fmt;
pub use crate::token:: Token;
use crate::environment::Values;

#[derive(Clone, Debug, Copy)]
pub enum TokenFunction {
    Builtin(),
    Lambda(),
}

#[derive(Clone, Debug, Copy)]
pub enum TokenTypes {
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
    DEFUN,
    CALL
}

pub enum Error {
    Reason(String),
    Return(Values)
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reason(string) => {
                write!(f, "ERROR: {}", string)
            }
            Error::Return(values) => {
                write!(f, "{}", values)
            }
        }

    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reason(string) => {
                write!(f, "ERROR: {}", string)
            },
            Error::Return(values)=> {
                write!(f, "{}", values)
            }
        }

    }
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



