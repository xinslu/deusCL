#[allow(unused_imports)]
use crate::token::{
    Token
};



#[derive(Clone, Debug)]
pub enum Expression {
        Assignment {
            name: Token,
            expr: Box<Expression>
        }, Literal {
            token: Token,
        }, Logical {
            operator: Token,
            expr: Vec<Expression>
        }, Grouping {
            expr: Box<Expression>
        }, Variable {
            name: Token
        }, Arithmetic {
            operator: Token,
            expr: Vec<Expression>
        }
}



