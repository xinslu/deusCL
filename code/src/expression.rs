#[allow(unused_imports)]
use crate::token::{
    Token
};



#[derive(Clone, Debug)]
pub enum Expression {
        Assignment {
            name: Box<Expression>,
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
        }, Local {
            declarations: Vec<Expression>,
            body: Vec<Expression>
        }, Set {
            declarations: Vec<Expression>
        }, Print {
            print: Box<Expression>
        }
}



