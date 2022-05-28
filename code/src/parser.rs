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
        self.current+=1;
        let expr = self.equality();
        return Ok(vec![expr]);
    }

    pub fn equality(&mut self) -> Expression {
        // print!("in equality");
        match self.token_list[self.current as usize]._type {
            TokenTypes::EQUAL => {
                return self.return_logical();
            },
            TokenTypes::SLASHEQUAL => {
                return self.return_logical();
            },
            TokenTypes::GREATER => {
                return self.return_logical();
            },
            TokenTypes::LESS => {
                return self.return_logical();
            },
            TokenTypes::GreaterEqual => {
                return self.return_logical();
            },
            TokenTypes::LessEqual => {
                return self.return_logical();
            },
            TokenTypes::MAX => {
                return self.return_arithmetic();
            },
            TokenTypes::MIN => {
                return self.return_arithmetic();
            },
            TokenTypes::PLUS => {
                return self.return_arithmetic();
            },
            TokenTypes::MINUS => {
                return self.return_arithmetic();
            },
            TokenTypes::STAR => {
                return self.return_arithmetic();
            },
            TokenTypes::SLASH => {
                return self.return_arithmetic();
            },
            TokenTypes::AND => {
                return self.return_logical();
            },
            TokenTypes::OR => {
                return self.return_logical();
            },
            TokenTypes::NOT => {
                return self.return_logical();
            },
            TokenTypes::MOD => {
                return self.return_arithmetic();
            },
            TokenTypes::LET => {
                return self.variable_declaration();
            },
            _ => {
                panic!("Not a Valid Operator {:?}", self.token_list[self.current as usize]._type);
            }
        }
    }

    pub fn return_logical(&mut self) -> Expression {
        let mut literals: Vec<Expression> = Vec::new();
        let operator = self.token_list[self.current as usize].clone();
        self.current+=1;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.is_at_end() {
                break;
            }
            if self.r#match(TokenTypes::LeftParen) {
                literals.push(self.equality());
            } else if self.r#match(TokenTypes::Number)  || self.r#match(TokenTypes::NIL) {
                literals.push(Expression::Literal {token: self.peek_before().clone()});
            }
        }
        Expression::Logical {operator: operator, expr: literals}
    }


    pub fn return_arithmetic(&mut self) -> Expression {
        let mut arith: Vec<Expression> = Vec::new();
        let operator = self.token_list[self.current as usize].clone();
        self.current+=1;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.is_at_end() {
                break;
            }
            if self.r#match(TokenTypes::LeftParen) {
                arith.push(self.equality());
            } else if self.r#match(TokenTypes::Number){
                arith.push(Expression::Literal {token: self.peek_before().clone()});
            }
        }
        Expression::Arithmetic {operator: operator, expr: arith}
    }


    pub fn variable_declaration(&mut self) -> Expression {
        self.current+=1;
        self.r#match(TokenTypes::LeftParen);
        self.r#match(TokenTypes::LeftParen);
        loop {
            if self.is_at_end() {
                break;
            }
            if self.r#match(TokenTypes::RightParen) {
                self.current+=1;
            }
            if self.r#match(TokenTypes::IDENTIFIER) {
                let name = Box::new(Expression::Variable {name: self.peek_before().clone()});
                let expr;
                if self.r#match(TokenTypes::LeftParen) {
                    expr = Box::new(self.equality());
                } else {
                    expr = Box::new(Expression::Literal {token: self.peek().clone()});
                }
                return Expression::Assignment {name , expr}
            }
        }
        panic!("Illegal Assignment {:?}", self.peek());
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
        } else if self.check(expected) == false {
            return false;
        }
        self.current += 1;
        return true;
    }

    pub fn peek(&mut self) -> &Token {
        return &self.token_list[self.current as usize];
    }

    pub fn peek_before(&mut self) -> &Token {
        return &self.token_list[(self.current - 1) as usize];
    }


    pub fn peek_next(&mut self) -> &Token {
        return &self.token_list[(self.current+1) as usize];
    }


}

