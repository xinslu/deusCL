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
                return self.local_declaration();
            },
            TokenTypes::SET => {
                return self.set_declaration();
            },
            TokenTypes::PRINT => {
                return self.print_declration();
            },
            TokenTypes::IDENTIFIER => {
                return self.variable()
            }
            TokenTypes::IF => {
                return self.if_declaration()
            },
            TokenTypes::LOOP => {
                return self.loop_declaration()
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
            } else if self.r#match(TokenTypes::IDENTIFIER){
                literals.push(Expression::Variable {name: self.peek_before().clone()});
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
            } else if self.r#match(TokenTypes::IDENTIFIER){
                arith.push(Expression::Variable {name: self.peek_before().clone()});
            }
        }
        Expression::Arithmetic {operator: operator, expr: arith}
    }


    pub fn local_declaration(&mut self) -> Expression {
        let mut local: Vec<Expression> = Vec::new();
        let mut bodyVec: Vec<Expression> = Vec::new();
        let mut body = false;
        self.current+=1;
        self.r#match(TokenTypes::LeftParen);
        self.r#match(TokenTypes::LeftParen);
        let mut closeCount = 0;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.r#match(TokenTypes::LeftParen) {
                self.current+=1;
                closeCount += 1
            }
            if self.is_at_end() {
                break;
            }

            if closeCount >= 2 {
                body = true
            }

            if self.r#match(TokenTypes::IDENTIFIER)  && !body{
                let name = Box::new(Expression::Variable {name: self.peek_before().clone()});
                let expr;
                if self.r#match(TokenTypes::LeftParen) {
                    expr = Box::new(self.equality());
                } else {
                    expr = Box::new(Expression::Literal {token: self.peek().clone()});
                    self.current+=1;
                }
                local.push(Expression::Assignment {name , expr});
            }

            if body && !self.r#match(TokenTypes::RightParen){
                self.r#match(TokenTypes::LeftParen);
                bodyVec.push(self.equality());
                println!("{:?}", self.peek());
                self.current += 1;
            }
        }
        Expression::Local {
            declarations: local,
            body: bodyVec
        }
    }


    pub fn set_declaration(&mut self) -> Expression {
        let mut local: Vec<Expression> = Vec::new();
        self.current+=1;
        let mut open = 1;
        loop {
            if self.r#match(TokenTypes::LeftParen) {
                self.current+=1;
                open += 1
            }

            if self.r#match(TokenTypes::RightParen) {
                self.current+=1;
                open -= 1;
            }

            if open <= 0 {
                self.current-=1;
                break;
            }

            if self.is_at_end() {
                break;
            }

            if self.r#match(TokenTypes::IDENTIFIER){
                let name = Box::new(Expression::Variable {name: self.peek_before().clone()});
                let expr;
                if self.r#match(TokenTypes::LeftParen) {
                    expr = Box::new(self.equality());
                } else {
                    expr = Box::new(Expression::Literal {token: self.peek().clone()});
                    self.current+=1;
                }
                local.push(Expression::Assignment {name , expr});
            }
        }
        Expression::Set {
            declarations: local
        }
    }

    pub fn print_declration(&mut self) -> Expression {
        self.current+=1;
        let expr;
        if self.r#match(TokenTypes::LeftParen) {
            expr = Box::new(self.equality());
        } else if self.r#match(TokenTypes::IDENTIFIER) {
            self.current-=1;
            expr = Box::new(self.equality());
        } else {
            expr = Box::new(Expression::Literal {token: self.peek().clone()});
            self.current+=1;
        }
        Expression::Print {
            print: expr
        }
    }

    pub fn if_declaration(&mut self) -> Expression {
        self.current+=2;
        let condition = self.equality();
        self.current+=1;
        let Body : Expression =  self.equality();
        let mut then : Option<Box<Expression>> = None;
        self.current -=1;
        if self.r#match(TokenTypes::LeftParen) {
            then = Some(Box::new(self.equality()))
        }
        self.current+=1;
        Expression::If {
            condition: Box::new(condition),
            body: Box::new(Body),
            then: then
        }
    }

    pub fn variable(&mut self) -> Expression {
        let ret = Expression::Variable {name: self.peek().clone()};
        self.current+=2;
        return ret;
    }


    pub fn loop_declaration(&mut self) -> Expression {
        let mut loopVec: Vec<Expression> = Vec::new();
        self.current+=1;
        self.matchPanic(TokenTypes::LeftParen);
        let name : Box<Expression> = Box::new(Expression::Variable {name: self.matchPanic(TokenTypes::IDENTIFIER)});
        let start : Box<Expression> =  Box::new(Expression::Literal {token: self.matchPanic(TokenTypes::Number)});
        let end : Box<Expression> =  Box::new(Expression::Literal {token: self.matchPanic(TokenTypes::Number)});
        self.matchPanic(TokenTypes::RightParen);
        loop {
            if self.r#match(TokenTypes::RightParen) || self.r#match(TokenTypes::LeftParen) { }
            if self.is_at_end() {
                break;
            } else {
                // println!("{:?}", self.peek());
                loopVec.push(self.equality());
                // println!("{:?}", loopVec);
                self.current += 1;
            }

        }
        Expression::Loop {
            variable: name,
            start: start,
            end: end,
            body: loopVec
        }

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

    pub fn matchPanic(&mut self, expected: TokenTypes) -> Token {
        if self.is_at_end() || self.check(expected) == false {
            panic!("Excepted {:?}", expected);
        }
        self.current += 1;
        return self.peek_before().clone();
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

