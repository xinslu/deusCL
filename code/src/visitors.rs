use crate::{expression::Expression, environment::Values};


pub trait Visitor {
    fn visit_logical(&mut self, log: Expression) -> Result<bool, &'static str >;
    fn visit_literal(&self, lit: &Expression) -> Result<Values, &'static str>;
    fn visit_arithmetic(&self, log: Expression) -> Result<i64, &'static str>;
    fn visit_local(&mut self, loc: Expression) -> Result<(), &'static str>;
    fn visit_set(&mut self, set: Expression) -> Result<(), &'static str >;
    fn visit_print(&mut self, print: Expression) -> Result<(), &'static str >;
    fn visit_if(&mut self, ifBlock: Expression) -> Result<(), &'static str >;
    fn visit_string(&mut self, string: Expression) -> Result<String, &'static str>;
    fn visit_for(&mut self, loopExpr: Expression) -> Result<(), &'static str >;
    fn visit_global(&mut self, global: Expression)-> Result<(), &'static str >;
}
