use crate::{expression::Expression, environment::Values};


pub trait Visitor {
    fn visit_logical(&mut self, log: Expression) -> bool;
    fn visit_literal(&self, lit: &Expression) -> Values;
    fn visit_arithmetic(&self, log: Expression) -> i64;
    fn visit_local(&mut self, loc: Expression);
    fn visit_set(&mut self, set: Expression);
    fn visit_print(&mut self, print: Expression);
    fn visit_if(&mut self, ifBlock: Expression);
    fn visit_string(&mut self, string: Expression) -> String;
    fn visit_for(&mut self, loopExpr: Expression);
    fn visit_global(&mut self, global: Expression);
}
