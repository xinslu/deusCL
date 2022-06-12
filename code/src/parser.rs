use crate::token::Token;
use crate::{types::{
    Error, TokenTypes
}, expression::Expression};
use std::mem;
pub struct Parser {
    current: i32,
    token_list: Vec<Token>
}


impl Parser{
    pub fn new(tokens: Vec<Token>) -> Parser{
        Parser {
            current: 0,
            token_list: tokens
        }
    }
    pub fn parse(&mut self) -> Result<Expression, Error> {
        self.matchPanic(TokenTypes::LeftParen)?;
        let expr = self.equality()?;
        return Ok(expr);
    }

    pub fn equality(&mut self) -> Result<Expression,Error> {
        // print!("in equality");
        match self.token_list[self.current as usize]._type {
            TokenTypes::EQUAL => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::SLASHEQUAL => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::GREATER => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::LESS => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::GreaterEqual => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::LessEqual => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::MAX => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::MIN => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::PLUS => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::MINUS => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::STAR => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::SLASH => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::AND => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::OR => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::NOT => {
                return Ok(self.return_logical()?);
            },
            TokenTypes::MOD => {
                return Ok(self.return_arithmetic()?);
            },
            TokenTypes::LET => {
                return Ok(self.local_declaration()?);
            },
            TokenTypes::SET => {
                return Ok(self.set_declaration()?);
            },
            TokenTypes::PRINT => {
                return Ok(self.print_declration()?);
            },
            TokenTypes::IDENTIFIER => {
                return Ok(self.variable())
            }
            TokenTypes::IF => {
                return Ok(self.if_declaration()?)
            },
            TokenTypes::LOOP => {
                return Ok(self.loop_declaration()?)
            },
            TokenTypes::VAR => {
                return Ok(self.var_declaration()?)
            },
            TokenTypes::Number => {
                return Ok(self.return_num()?)
            },
            TokenTypes::LeftParen => {
                return Ok(self.block_declaration()?)
            }
            TokenTypes::CONCAT => {
                return Ok(self.strings()?)
            },
            TokenTypes::RETURN => {
                return Ok(self.return_statements()?)
            }, TokenTypes::DEFUN =>  {
                return Ok(self.functional_declaration()?)
            }
            _ => {
                Err(Error::Reason(format!("Invalid Operator {}", self.peek())))
            }
        }
    }

    pub fn functional_declaration(&mut self) -> Result<Expression, Error> {
        self.matchPanic(TokenTypes::DEFUN)?;
        let name = self.matchPanic(TokenTypes::IDENTIFIER)?;
        self.matchPanic(TokenTypes::LeftParen)?;
        let mut parameters: Vec<Expression> = Vec::new();
        loop {
            if self.r#match(TokenTypes::RightParen) {
                break;
            }
            parameters.push(self.equality()?)
        }
        let body = self.equality()?;
        Ok(Expression::Function {
            name,
            parameters,
            body: Box::new(body)
        })
    }

    pub fn return_statements(&mut self) -> Result<Expression, Error> {
        self.current+=1;
        self.matchPanic(TokenTypes::LeftParen)?;
        Ok(Expression::Return {
            result: Box::new(self.equality()?)
        })
    }

    pub fn strings(&mut self) ->Result<Expression, Error> {
        let mut string_concat: Vec<Expression> = Vec::new();
        let operator = self.token_list[self.current as usize].clone();
        self.current+=1;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.is_at_end() {
                break;
            }
            if self.r#match(TokenTypes::LeftParen) {
                string_concat.push(self.equality()?);
            } else if self.r#match(TokenTypes::Number) || self.r#match(TokenTypes::STRINGLITERAL) {
                string_concat.push(Expression::Literal {token: self.peek_before().clone()});
            } else if self.r#match(TokenTypes::IDENTIFIER){
                string_concat.push(Expression::Variable {name: self.peek_before().clone()});
            }
        }
        Ok(Expression::StringMan {operator, expr: string_concat})
    }

    pub fn block_declaration(&mut self) -> Result<Expression, Error> {
        let mut expressions = Vec::new();
        self.matchPanic(TokenTypes::LeftParen)?;
        loop {
            if self.r#match(TokenTypes::RightParen) {
                break;
            }
            expressions.push(self.equality()?);
        }
        Ok(Expression::Block {expressions: expressions})
    }

    pub fn return_num(&mut self) -> Result<Expression, Error> {
        self.current+=1;
        Ok(Expression::Literal {token: self.peek_before ().clone()})
    }

    pub fn return_logical(&mut self) -> Result<Expression,Error> {
        let mut literals: Vec<Expression> = Vec::new();
        let operator = self.token_list[self.current as usize].clone();
        self.current+=1;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.is_at_end() {
                break;
            }
            if self.r#match(TokenTypes::LeftParen) {
                literals.push(self.equality()?);
            } else if self.r#match(TokenTypes::Number)  || self.r#match(TokenTypes::NIL) {
                literals.push(Expression::Literal {token: self.peek_before().clone()});
            } else if self.r#match(TokenTypes::IDENTIFIER){
                literals.push(Expression::Variable {name: self.peek_before().clone()});
            }
        }
        Ok(Expression::Logical {operator, expr: literals})
    }


    pub fn return_arithmetic(&mut self) -> Result<Expression,Error> {
        let mut arith: Vec<Expression> = Vec::new();
        let operator = self.token_list[self.current as usize].clone();
        self.current+=1;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.is_at_end() {
                break;
            }
            if self.r#match(TokenTypes::LeftParen) {
                arith.push(self.equality()?);
            } else if self.r#match(TokenTypes::Number){
                arith.push(Expression::Literal {token: self.peek_before().clone()});
            } else if self.r#match(TokenTypes::IDENTIFIER){
                arith.push(Expression::Variable {name: self.peek_before().clone()});
            }
        }
        Ok(Expression::Arithmetic {operator, expr: arith})
    }


    pub fn local_declaration(&mut self) -> Result<Expression,Error> {
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
                    expr = Box::new(self.equality()?);
                } else {
                    expr = Box::new(Expression::Literal {token: self.peek().clone()});
                    self.current+=1;
                }
                local.push(Expression::Assignment {name , expr});
            }

            if body && !self.r#match(TokenTypes::RightParen){
                self.r#match(TokenTypes::LeftParen);
                bodyVec.push(self.equality()?);
                self.current += 1;
            }
        }
        Ok(Expression::Local {
            declarations: local,
            body: bodyVec
        })
    }


    pub fn set_declaration(&mut self) -> Result<Expression,Error> {
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
                    expr = Box::new(self.equality()?);
                } else {
                    expr = Box::new(Expression::Literal {token: self.peek().clone()});
                    self.current+=1;
                }
                local.push(Expression::Assignment {name , expr});
            }
        }
        Ok(Expression::Set {
            declarations: local
        })
    }

    pub fn print_declration(&mut self) -> Result<Expression,Error> {
        self.current+=1;
        let expr;
        if self.r#match(TokenTypes::LeftParen) {
            expr = Box::new(self.equality()?);
        } else if self.r#match(TokenTypes::IDENTIFIER) {
            self.current-=1;
            expr = Box::new(self.equality()?);
            self.current += 1
        } else {
            expr = Box::new(Expression::Literal {token: self.peek().clone()});
            self.current+=1;
        }
        Ok(Expression::Print {
            print: expr
        })
    }

    pub fn if_declaration(&mut self) -> Result<Expression,Error> {
        self.current+=2;
        let condition = self.equality()?;
        self.current+=1;
        let Body : Expression =  self.equality()?;
        let mut then : Option<Box<Expression>> = None;
        self.current -=1;
        if self.r#match(TokenTypes::LeftParen) {
            then = Some(Box::new(self.equality()?))
        }
        self.current+=1;
        Ok(Expression::If {
            condition: Box::new(condition),
            body: Box::new(Body),
            then
        })
    }

    pub fn variable(&mut self) -> Expression {
        let ret = Expression::Variable {name: self.peek().clone()};
        self.current+=1;
        return ret;
    }


    pub fn loop_declaration(&mut self) -> Result<Expression,Error> {
        let mut loopVec: Vec<Expression> = Vec::new();
        self.current+=1;
        self.matchPanic(TokenTypes::LeftParen)?;
        let name : Box<Expression> = Box::new(Expression::Variable {name: self.matchPanic(TokenTypes::IDENTIFIER)?});
        let start : Box<Expression> =  Box::new(Expression::Literal {token: self.matchPanic(TokenTypes::Number)?});
        let end : Box<Expression> =  Box::new(self.equality()?);
        self.matchPanic(TokenTypes::RightParen)?;
        loop {
            if self.r#match(TokenTypes::RightParen) || self.r#match(TokenTypes::LeftParen) { }
            if self.is_at_end() {
                break;
            } else {
                // println!("{:?}", self.peek());
                loopVec.push(self.equality()?);
                // println!("{:?}", loopVec);
                self.current += 1;
            }
        }
        Ok(Expression::Loop {
            variable: name,
            end,
            start,
            body: loopVec
        })
    }

    pub fn var_declaration(&mut self) -> Result<Expression,Error> {
        self.matchPanic(TokenTypes::VAR)?;
        let name = Expression::Variable {name: self.matchPanic(TokenTypes::IDENTIFIER)?};
        let expr;
        if self.r#match(TokenTypes::LeftParen) {
            expr = Box::new(self.equality()?);
        } else {
            expr = Box::new(Expression::Literal {token: self.peek().clone()});
            self.current+=1;
        }
        Ok(Expression::Global { name: Box::new(name), expr})
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

    pub fn matchPanic(&mut self, expected: TokenTypes) -> Result<Token,Error> {
        if self.is_at_end() || self.check(expected) == false {
            return Err(Error::Reason(format!("Excepted a {}. Got a {}", expected, self.peek()._type)))
        }
        self.current += 1;
        return Ok(self.peek_before().clone());
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

