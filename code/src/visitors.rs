use crate::expression::{
    Expression
};


pub trait Visitor {
    fn visit_logical(&mut self, log: Expression) -> bool;
}
