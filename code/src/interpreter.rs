use crate::types::{
    TokenTypes
};
use crate::expression::{
    Expression
};
use crate::visitors::{
    Visitor
};

use crate::environment::{
    Environment
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

    pub fn comparision_lambda(&mut self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> bool) -> bool  {
        if expr.len() as i32 == 0 {
            panic!("Parsing Error!");
        }
        let mut temp = self.visit_literal(&expr[0]);
        let mut rBool = true;
        for i in &expr[1..] {
            rBool = rBool && func(temp, self.visit_literal(i));
            temp = self.visit_literal(i);
            if rBool == false {
                return false;
            }
        }
        return rBool;
    }

    pub fn logical_lambda(&mut self, expr: Vec<Expression>, mut rBool: bool,func: &dyn Fn(bool, bool) -> bool) -> bool  {
        for i in &expr {
            match &i {
                Expression::Literal {token} => {
                    match token._type {
                        TokenTypes::NIL => {
                            return false;
                        },
                        _ => {
                            self.visit_literal(i);
                            rBool = func(rBool,true);
                        }
                    }
                },
                Expression::Logical { operator: _, expr: _ } => {
                    rBool = func(rBool, self.visit_logical(i.clone()));
                },
                _ => {
                    self.visit_literal(i);
                }
            }
        }
        return rBool;
    }

    pub fn artihmetic_lambda(&mut self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> i64) -> i64  {
        let mut temp = self.visit_literal(&expr[0]);
        for i in &expr[1..] {
            temp = func(temp, self.visit_literal(i));
        }
        return temp;
    }

}
impl Visitor for Interpreter {
    fn visit_logical(&mut self, log: Expression) -> bool {
        let notequals = |a, b| a != b;
        let equals = |a, b| a == b;
        let greater = |a, b| a > b;
        let lesser = |a, b| a < b;
        let greaterequal = |a, b| a >= b;
        let lesserequal = |a, b| a <= b;
        let and = |a,b| a && b;
        let or = |a,b| a || b;
        match log {
            Expression::Logical { operator, expr } => {
                match operator._type {
                    TokenTypes::EQUAL => {
                        return self.comparision_lambda(expr, &equals);
                    },
                    TokenTypes::SLASHEQUAL => {
                        return self.comparision_lambda(expr, &notequals);
                    },
                    TokenTypes::GREATER => {
                        return self.comparision_lambda(expr, &greater);
                    },
                    TokenTypes::LESS => {
                        return self.comparision_lambda(expr, &lesser);
                    },
                    TokenTypes::GreaterEqual => {
                       return self.comparision_lambda(expr, &greaterequal);
                    },
                    TokenTypes::LessEqual => {
                        return self.comparision_lambda(expr, &lesserequal);
                    },
                    TokenTypes::AND => {
                        return self.logical_lambda(expr, true, &and);
                    },
                    TokenTypes::OR => {
                        return self.logical_lambda(expr, false, &or);
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
        let add = |a, b| a + b;
        let minus = |a, b| a - b;
        let multiply = |a, b| a * b;
        let divide = |a, b| a / b;
        let modulus = |a, b| a % b;
        match arith {
            Expression::Arithmetic { operator, expr } => {
                match operator._type {
                    TokenTypes::MAX => {
                        return self.artihmetic_lambda(expr, &cmp::max);
                    },
                    TokenTypes::MIN => {
                        return self.artihmetic_lambda(expr, &cmp::min);
                    },
                    TokenTypes::PLUS => {
                        return self.artihmetic_lambda(expr, &add);
                    },
                    TokenTypes::MINUS => {
                        return self.artihmetic_lambda(expr, &minus);
                    },
                    TokenTypes::STAR => {
                        return self.artihmetic_lambda(expr, &multiply);
                    },
                    TokenTypes::SLASH => {
                        return self.artihmetic_lambda(expr, &divide);
                    },
                    TokenTypes::MOD => {
                        return self.artihmetic_lambda(expr, &modulus);
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

