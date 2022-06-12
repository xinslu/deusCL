use crate::{expression::Expression, environment::Values, types::Error};


pub trait Visitor {
    fn visit_logical(&mut self, log: Expression) -> Result<bool, Error >;
    fn visit_literal(&self, lit: &Expression) -> Result<Values, Error>;
    fn visit_arithmetic(&self, log: Expression) -> Result<i64, Error>;
    fn visit_local(&mut self, loc: Expression) -> Result<(), Error>;
    fn visit_set(&mut self, set: Expression) -> Result<(), Error >;
    fn visit_print(&mut self, print: Expression) -> Result<(), Error >;
    fn visit_if(&mut self, ifBlock: Expression) -> Result<(), Error >;
    fn visit_string(&mut self, string: Expression) -> Result<String, Error>;
    fn visit_for(&mut self, loopExpr: Expression) -> Result<(), Error >;
    fn visit_global(&mut self, global: Expression)-> Result<(), Error >;
    fn visit_block(&mut self, global: Expression) -> Result<(), Error>;
    fn visit_string_man(&mut self, strings: Expression) -> Result<String, Error>;
    fn visit_return(&mut self, ret: Expression) -> Result<(),Error>;
    fn visit_function_dec(&mut self, func: Expression) -> Result<(), Error>;
}
