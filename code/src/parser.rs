use crate::token::Token;
use crate::types::{
    Errors, TokenTypes
};
use crate::expression::{
    Expression
};
use std::mem;
pub struct Parser {
    current: i32,
    token_list: Vec<Token>
}

impl Parser{
    pub fn create(tokens: Vec<Token>) -> Parser{
        Parser {
            current: 0,
            token_list: tokens
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Expression>, Errors> {
        let expr = self.equality();
        return Ok(vec![expr]);
    }

    pub fn equality(&mut self) -> Expression{
        self.current+=1;
        match self.token_list[self.current as usize]._type {
            TokenTypes::EQUAL => {
                println!("Here in Parser Equal");
                Expression::Logical {operator: self.token_list[self.current as usize].clone(), Expr: vec![Expression::Literal {token: self.token_list[(self.current+1) as usize].clone()}, Expression::Literal {token: self.token_list[(self.current+2) as usize].clone()}]}
            },
            _ => {
                panic!("Brung Error");
            }
        }
    }

    pub fn return_slice(&mut self) {

    }

    pub fn check(&mut self, toktype: TokenTypes) -> bool{
        if self.is_at_end() {
            return false;
        } else {
            return mem::discriminant(&self.token_list[self.current as usize]._type) == { mem::discriminant(&toktype) }
        }

    }

    pub fn is_at_end(&mut self) -> bool {
        return self.current >= self.token_list.len() as i32;
    }

    pub fn r#match(&mut self, expected: TokenTypes) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.check(expected) {
            return false;
        }
        self.current += 1;
        return true;
    }

    pub fn peek(&mut self) -> &Token {
        return &self.token_list[self.current as usize];
    }


    pub fn peek_next(&mut self) -> &Token {
        return &self.token_list[(self.current+1) as usize];
    }


}

