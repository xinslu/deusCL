#[allow(unused_imports)]
use crate::token::Token;



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
        }, If {
            condition: Box<Expression>,
            body: Box<Expression>,
            then: Option<Box<Expression>>
        }, Loop {
            variable: Box<Expression>,
            start: Box<Expression>,
            end: Box<Expression>,
            body: Vec<Expression>
        }, Global {
            name: Box<Expression>,
            expr: Box<Expression>
        }, Block {
            expressions: Vec<Expression>
        }, StringMan {
            operator: Token,
            expr: Vec<Expression>
        }, Return {
            result: Box<Expression,>
        }, Function {
            name: Token,
            parameters: Vec<Expression>,
            body: Box<Expression>
        }, Call {
            name: Token,
            parameters: Vec<Expression>
        }
}



