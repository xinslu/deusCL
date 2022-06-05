use crate::environment::Values;
use crate::{expression::Expression, types::TokenTypes, environment::Environment, visitors::Visitor};

use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use std::any::Any;

pub struct Interpreter {
    environment: Environment,
    locals: HashMap<String, i64>
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(None),
            locals: HashMap::new()
        }
    }
    pub fn accept(&mut self, expression: Expression ) {
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                println!("{:?}", self.visit_logical(expression));
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                println!("{:?}", self.visit_arithmetic(expression));
            },
            Expression::Local {declarations: _, body: _} => {
                self.visit_local(expression);
            },
            Expression::Set {declarations: _} => {
                self.visit_set(expression);
            },
            Expression::Print { print: _ } => {
                self.visit_print(expression);
            },
            Expression::Literal { ref token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    self.visit_string(expression);
                } else {
                    self.visit_literal(&expression);
                }

            },
            Expression::If {condition: _, body: _, then: _} => {
                self.visit_if(expression);
            },
            Expression::Loop {..} => {
                self.visit_for(expression);
            },
            Expression::Global {..} => {
                self.visit_global(expression);
            }
            _ => {
                println!("Unsupported Operation Right Now");
            }
        }
    }

    pub fn process(&mut self, expression: Expression ) -> Box<dyn Display + 'static> {
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                Box::new(self.visit_logical(expression))
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                Box::new(self.visit_arithmetic(expression))
            },
            Expression::Local {declarations: _, body: _} => {
                self.visit_local(expression);
                Box::new("")
            },
            Expression::Set {declarations: _} => {
                self.visit_set(expression);
                Box::new("")
            },
            Expression::Print { print: _ } => {
                self.visit_print(expression);
                Box::new("")
            },
            Expression::Variable { name: _ } => {
                Box::new(self.visit_literal(&expression))
            },
            Expression::Literal { ref token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    Box::new(self.visit_string(expression))
                } else {
                    Box::new(self.visit_literal(&expression))
                }

            },
            Expression::If {condition: _, body: _, then: _} => {
                self.visit_if(expression);
                Box::new("")
            },
            _ => {
                println!("Unsupported Operation Right Now");
                Box::new("")
            }
        }
    }

    pub fn clean_env(&mut self) {
        self.locals = HashMap::new();
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
                    if let TokenTypes::NIL = token._type {
                        return false;
                    } else {
                        self.visit_literal(i);
                        rBool = func(rBool,true);
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

    pub fn artihmetic_lambda(&self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> i64) -> i64  {
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
                                    if let TokenTypes::NIL = token._type {
                                        return true;
                                    } else {
                                        let result = self.visit_literal(&expr[0]);
                                        if result == 0 {
                                            return false;
                                        }
                                        return !(result != 0)
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

    fn visit_literal(&self, lit: &Expression) -> i64 {
        match &lit {
            Expression::Literal {token} => {
                if let TokenTypes::Number = token._type {
                    return token.lexeme.parse().unwrap();
                } else {
                    panic!("Should Be a Number Only!");
                }
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                return self.visit_arithmetic(lit.clone());
            },
            Expression::Variable { name } => {
                if let Some(value) = self.locals.get(&name.lexeme) {
                    return *value;
                }
                match self.environment.get(name.lexeme.clone()) {
                    Values::Int(int) => {return int},
                    Values::Str(_) => {
                        panic!("Not of Right type")
                    }
                    Values::Bool(_) => {
                        panic!("Not of Right type")
                    }
                }
            }
            _ => {
                panic!("Not a Literal!");
            }
        }
    }

    fn visit_arithmetic(&self, arith: Expression) -> i64 {
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

    fn visit_local(&mut self, local: Expression) {
        if let Expression::Local{declarations, body} = local {
            for i in declarations {
                if let Expression::Assignment { name, expr } = i {
                    if let Expression::Variable { name } = *name {
                        self.locals.insert(name.lexeme, self.visit_literal(&*expr));
                    } else {
                        println!("Not a Variable");
                    }
                } else {
                    println!("Not a Variable");
                }
            }

            for j in body {
                self.process(j);
            }
            for (key, value) in &self.locals {
                println!("{} => {}", key, value);
            }
        } else {
            panic!("Wrong Type");
        }
    }

    fn visit_set(&mut self, set: Expression) {
        match set {
            Expression::Set { declarations } => {
                for i in declarations {
                    match i {
                        Expression::Assignment { name, expr } => {
                            if let Expression::Variable { name } = *name {
                                self.locals.insert(name.lexeme, self.visit_literal(&*expr));
                            } else {
                                panic!("bruh");
                            }
                        },
                        _ => {
                            panic!("Bruh");
                        }
                    }
                }
            },  _ => {
                panic!("Invalid Assignment")
            }
        }
    }


    fn visit_print(&mut self, print: Expression) {
        match print {
            Expression::Print { print } => {
                println!("{}", self.process(*print));
            },
            _ => {
                panic!("Illegal Operation");
            }
        }
    }

    fn visit_if(&mut self, ifBlock: Expression) {
        match ifBlock {
            Expression::If {condition, body, then} => {
                match *condition {
                    Expression::Logical {operator: _, expr: _} => {
                        let cond = self.visit_logical(*condition);
                        if cond == true {
                            self.process(*body);
                        } else {
                            if let Some(thenBody) = then {
                                self.process(*thenBody);
                            }
                        }
                    },
                    _ => {
                        panic!("Wrong Type Of Condition");
                    }
                }
            },
            _ => {
                panic!("Invalid Type")
            }
        }
    }

    fn visit_string(&mut self, string: Expression) -> String{
        match string {
            Expression::Literal{token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    return token.lexeme;
                } else {
                    panic!("Not a string");
                }
            }
            _ => {
                panic!("Not a String");
            }
        }
    }
    fn visit_for(&mut self, loopExpr: Expression) {
        match loopExpr {
            Expression::Loop {variable, start, end, body} => {
                match *variable {
                    Expression::Variable {name} => {
                        let startInt = self.visit_literal(&*start);
                        let endInt = self.visit_literal(&*end);
                        for i in startInt..endInt {
                            self.locals.insert(name.lexeme.clone(), i);
                            for j in &body {
                                self.process(j.clone());
                            }
                        }
                    },
                    _ => {panic!("not a variable")}
                }
            },
            _ => {panic!("Not a For Loop")}
        }

    }
    fn visit_global(&mut self, global: Expression) {
        if let Expression::Global { name, expr } = global {
            if let Expression::Variable { name } = *name {
                let value = self.visit_literal(&*expr);
                let nameVar = name.lexeme.clone();
                self.environment.define(nameVar, value);
                self.locals.insert(name.lexeme, value);
            }
        } else {
            panic!("Not a GLobal Variable declarations");
        }
    }
}

