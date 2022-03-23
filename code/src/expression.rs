use crate::token::{
    Token
};
#[derive(Clone, Debug)]
pub enum Expression {
        Assignment {
                name: Token,
                Expr: Box<Expression>
        }, Literal {
                token: Token,
        }, Logical {
                operator: Token,
                Expr: Vec<Expression>
        }, Grouping {
                Expr: Box<Expression>
        }, Variable {
                name: Token
        }, Arithmetic {
                operator: Token,
                Expr: Vec<Expression>
        }
}


