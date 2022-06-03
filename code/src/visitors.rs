use crate::expression::{
    Expression
};


pub trait Visitor {
    fn visit_logical(&mut self, log: Expression) -> bool;
    fn visit_literal(&self, lit: &Expression) -> i64;
    fn visit_arithmetic(&self, log: Expression) -> i64;
    fn visit_local(&mut self, loc: Expression);
    fn visit_set(&mut self, set: Expression);
    fn visit_print(&mut self, print: Expression);
}
