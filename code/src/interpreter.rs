use crate::token::Token;
use crate::types::{
    Errors, TokenTypes
};
use crate::expression::{
    Expression
};
use crate::visitors::{
    Visitor
};




pub struct Interpreter;
impl Interpreter {
    pub fn accept(&mut self, expression: Expression ) {
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                println!("{:?}", self.visit_logical(expression));
            },
            _ => {
                println!("panic mode");
            }
        }
    }
}
impl Visitor for Interpreter {
    fn visit_logical(&mut self, log: Expression) -> bool {
        match log {
            Expression::Logical { operator, expr } => {
                match operator._type {
                    TokenTypes::EQUAL => {
                        if expr.len() as i32 == 0 {
                            panic!("Bruh error");
                        }
                        let _first_num = self.visit_literal(&expr[0]);
                        let mut _rolling_bool = true;
                        for i in &expr {
                            _rolling_bool = _rolling_bool && (_first_num == self.visit_literal(i));
                            if _rolling_bool == false {
                                return false;
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::SLASHEQUAL => {
                        let _first_num = self.visit_literal(&expr[0]);
                        let mut _rolling_bool = true;
                        for i in &expr[1..] {
                            _rolling_bool = _rolling_bool && (_first_num != self.visit_literal(i));
                            if _rolling_bool == false {
                                return false;
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::GREATER => {
                        let mut temp = self.visit_literal(&expr[0]);
                        let mut _rolling_bool = true;
                        for i in &expr[1..] {
                            _rolling_bool = _rolling_bool && (self.visit_literal(i) < temp);
                            temp = self.visit_literal(i);
                            if _rolling_bool == false {
                                return false;
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::LESS => {
                        let mut temp = self.visit_literal(&expr[0]);
                        let mut _rolling_bool = true;
                        for i in &expr[1..] {
                            _rolling_bool = _rolling_bool && (self.visit_literal(i) > temp);
                            temp = self.visit_literal(i);
                            if _rolling_bool == false {
                                return false;
                            }
                        }
                        return _rolling_bool;
                    },
                    _ => {
                        panic!("bruh rip");
                    }
                }
            },
            _=> {
                panic!("bruh error");
            }
        }
    }

    fn visit_literal(&mut self, lit: &Expression) -> i64 {
        match &lit {
            Expression::Literal {token} => {
                match token._type {
                    TokenTypes::Number =>{
                        return token.lexeme.parse().unwrap();
                    },
                    _ => {
                        panic!("rip error");
                    }
                }
            },
            _ => {
                panic!("rip error");
            }
        }
    }

}

