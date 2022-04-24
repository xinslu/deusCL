use crate::types::{
    TokenTypes
};
use crate::expression::{
    Expression
};
use crate::visitors::{
    Visitor
};
use std::cmp;

pub struct Interpreter;
impl Interpreter {
    pub fn accept(&mut self, expression: Expression ) {
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                println!("{:?}", self.visit_logical(expression));
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                println!("{:?}", self.visit_arithmetic(expression));
            },
            _ => {
                println!("Unsupported Operation Right Now");
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
                            panic!("Parsing Error!");
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
                    TokenTypes::GreaterEqual => {
                        let mut temp = self.visit_literal(&expr[0]);
                        let mut _rolling_bool = true;
                        for i in &expr[1..] {
                            _rolling_bool = _rolling_bool && (self.visit_literal(i) <= temp);
                            temp = self.visit_literal(i);
                            if _rolling_bool == false {
                                return false;
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::LessEqual => {
                        let mut temp = self.visit_literal(&expr[0]);
                        let mut _rolling_bool = true;
                        for i in &expr[1..] {
                            _rolling_bool = _rolling_bool && (self.visit_literal(i) >= temp);
                            temp = self.visit_literal(i);
                            if _rolling_bool == false {
                                return false;
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::AND => {
                        let mut _rolling_bool = true;
                        for i in &expr {
                            match &i {
                                Expression::Literal {token} => {
                                    match token._type {
                                        TokenTypes::NIL => {
                                            return false;
                                        },
                                        _ => {
                                            let result = self.visit_literal(i);
                                            if result == 0 {
                                                _rolling_bool = _rolling_bool && 1 != 0;
                                            }
                                            _rolling_bool = _rolling_bool && (result != 0)
                                        }
                                    }
                                },
                                Expression::Logical { operator: _, expr: _ } => {
                                    _rolling_bool = _rolling_bool && self.visit_logical(i.clone());
                                },
                                _ => {
                                    self.visit_literal(i);
                                }
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::OR => {
                        let mut _rolling_bool = false;
                        for i in &expr {
                            match &i {
                                Expression::Literal {token} => {
                                    match token._type {
                                        TokenTypes::NIL => {
                                            _rolling_bool = _rolling_bool || false;
                                        },
                                        _ => {
                                            _rolling_bool = _rolling_bool || true;
                                        }
                                    }
                                },
                                Expression::Logical { operator: _, expr: _ } => {
                                    _rolling_bool = _rolling_bool || self.visit_logical(i.clone());
                                },
                                _ => {
                                    self.visit_literal(i);
                                }
                            }
                        }
                        return _rolling_bool;
                    },
                    TokenTypes::NOT => {
                        if expr.len() > 1 {
                            panic!("Cannot Have more than 1 Arguement");
                        }
                        match &expr[0] {
                            Expression::Literal {token} => {
                                    match token._type {
                                        TokenTypes::NIL => {
                                            return true;
                                        },
                                        _ => {
                                            let result = self.visit_literal(&expr[0]);
                                            if result == 0 {
                                                return false;
                                            }
                                            return !(result != 0)
                                        }
                                    }
                                },
                                Expression::Logical { operator: _, expr: _ } => {
                                    return !self.visit_logical(expr[0].clone());
                                },
                                _ => {
                                    return true
                                }
                        }
                    }
                    _ => {
                        panic!("Unsupported Operator!");
                    }
                }
            },
            _=> {
                panic!("Not a Logical Expression!");
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
                        panic!("Should Be a Number Only!");
                    }
                }
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                return self.visit_arithmetic(lit.clone());
            },
            _ => {
                panic!("Not a Literal!");
            }
        }
    }

    fn visit_arithmetic(&mut self, arith: Expression) -> i64 {
        match arith {
            Expression::Arithmetic { operator, expr } => {
                match operator._type {
                    TokenTypes::MAX => {
                        let mut temp = self.visit_literal(&expr[0]);
                        for i in &expr[1..] {
                            temp = cmp::max(self.visit_literal(i), temp);
                        }
                        return temp;
                    },
                    TokenTypes::MIN => {
                        let mut temp = self.visit_literal(&expr[0]);
                        for i in &expr[1..] {
                            temp = cmp::min(self.visit_literal(i), temp);
                        }
                        return temp;
                    },
                    TokenTypes::PLUS => {
                        let mut temp = 0;
                        for i in &expr {
                            temp += self.visit_literal(i);
                        }
                        return temp;
                    },
                    TokenTypes::MINUS => {
                        let mut temp = self.visit_literal(&expr[0]);
                        for i in &expr[1..] {
                            temp -= self.visit_literal(i);
                        }
                        return temp;
                    },
                    TokenTypes::STAR => {
                        let mut temp = 1;
                        for i in &expr {
                            temp *= self.visit_literal(i);
                        }
                        return temp;
                    },
                    TokenTypes::SLASH => {
                        let mut temp = self.visit_literal(&expr[0]);
                        for i in &expr[1..] {
                            temp /= self.visit_literal(i);
                        }
                        return temp;
                    },
                    _ => {
                        panic!("Not a Valid Type!");
                    }
                }
            }
            _ => {
                panic!("Not an Arithmetic Expression!");
            }

        }
    }

}

