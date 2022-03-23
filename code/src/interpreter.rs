use crate::token::Token;
use crate::types::{
    Errors, TokenTypes
};
use crate::expression::{
    Expression
};
use crate::visitors::{
    Visitor
};
use std::mem;



pub struct Interpreter;
impl Interpreter {
    pub fn bruh(&mut self) {}
}
impl Visitor for Interpreter {
    fn visit_logical(&mut self, log: Expression) -> bool {
        match log {
            Expression::Logical { operator: _, Expr: _ } => {
                println!("it is logical in interperter");
            }
            _=> {
                println!("bruh error");
            }
        }
        return true;
    }
}
