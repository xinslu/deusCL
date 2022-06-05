use crate::environment::Encapsulation;
use crate::environment::Values;
use crate::{expression::Expression, types::TokenTypes, environment::Environment, visitors::Visitor};

use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use std::any::Any;

pub struct Interpreter {
    environment: Environment,
    locals: HashMap<String, Values>
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(None),
            locals: HashMap::new()
        }
    }
    pub fn accept(&mut self, expression: Expression ) -> Result<(), &'static str>{
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                println!("{:?}", self.visit_logical(expression)?);
                Ok(())
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                println!("{:?}", self.visit_arithmetic(expression)?);
                Ok(())
            },
            Expression::Local {declarations: _, body: _} => {
                self.visit_local(expression)
            },
            Expression::Set {declarations: _} => {
                self.visit_set(expression)?;
                Ok(())
            },
            Expression::Print { print: _ } => {
                self.visit_print(expression)
            },
            Expression::Literal { ref token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    self.visit_string(expression)?;
                    return Ok(());
                }
                self.visit_literal(&expression)?;
                Ok(())

            },
            Expression::If {condition: _, body: _, then: _} => {
                if let Err(error) = self.visit_if(expression) {
                    return Err(error);
                }
                Ok(())
            },
            Expression::Loop {..} => {
                if let Err(error) = self.visit_for(expression) {
                    return Err(error);
                }
                Ok(())
            },
            Expression::Global {..} => {
                self.visit_global(expression)?;
                Ok(())
            }
            _ => {
                Err("ERROR: Unsupported Operation Right Now")
            }
        }
    }

    pub fn process(&mut self, expression: Expression ) -> Result<Option<Values>, &'static str> {
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                Ok(Some((self.visit_logical(expression)?).return_value()))
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                if let Ok(value) = self.visit_arithmetic(expression) {
                    return Ok(Some(value.return_value()));
                }
                Err("ERROR: In Processsing Arithmetic Operation")

            },
            Expression::Local {declarations: _, body: _} => {
                if let Err(error) = self.visit_local(expression) {
                    return Err(error)
                }
                Ok(None)
            },
            Expression::Set {declarations: _} => {
                self.visit_set(expression)?;
                Ok(None)
            },
            Expression::Print { print: _ } => {
                if let Err(error) = self.visit_print(expression) {
                    return Err(error)
                }
                Ok(None)
            },
            Expression::Variable { name: _ } => {
                Ok(Some(self.visit_literal(&expression)?))
            },
            Expression::Literal { ref token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    Ok(Some(self.visit_string(expression)?.return_value()))
                } else {
                    Ok(Some(self.visit_literal(&expression)?))
                }

            },
            Expression::If {condition: _, body: _, then: _} => {
                if let Err(error) = self.visit_if(expression) {
                    return Err(error);
                }
                Ok(None)
            },
            _ => {
                Err("ERROR: Unsupported Operation")
            }
        }
    }

    pub fn clean_env(&mut self) {
        self.locals = HashMap::new();
    }

    pub fn comparision_lambda(&mut self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> bool) -> Result<bool,&'static str>  {
        if expr.len() as i32 == 0 {
            return Err("Error: Parsing Error, not enough arguements")
        }
        let mut temp = (self.visit_literal(&expr[0])?).matchInteger()?;
        let mut rBool = true;
        for i in &expr[1..] {
            rBool = rBool && func(temp, (self.visit_literal(i)?).matchInteger()?);
            temp = (self.visit_literal(i)?).matchInteger()?;
            if rBool == false {
                return Ok(false);
            }
        }
        return Ok(rBool);
    }

    pub fn logical_lambda(&mut self, expr: Vec<Expression>, mut rBool: bool,func: &dyn Fn(bool, bool) -> bool) -> Result<bool, &'static str>  {
        for i in &expr {
            match &i {
                Expression::Literal {token} => {
                    if let TokenTypes::NIL = token._type {
                        return Ok(false);
                    } else {
                        self.visit_literal(i)?;
                        rBool = func(rBool,true);
                    }
                },
                Expression::Logical { operator: _, expr: _ } => {
                    rBool = func(rBool, self.visit_logical(i.clone())?);
                },
                _ => {
                    self.visit_literal(i)?;
                }
            }
        }
        return Ok(rBool);
    }

    pub fn artihmetic_lambda(&self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> i64) -> Result<i64, &'static str>  {
        let mut temp = (self.visit_literal(&expr[0])?).matchInteger()?;
        for i in &expr[1..] {
            temp = func(temp, (self.visit_literal(i)?).matchInteger()?);
        }
        return Ok(temp);
    }

}
impl Visitor for Interpreter {
    fn visit_logical(&mut self, log: Expression) -> Result<bool, &'static str> {
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
                        return Ok(self.comparision_lambda(expr, &equals)?);
                    },
                    TokenTypes::SLASHEQUAL => {
                        return Ok(self.comparision_lambda(expr, &notequals)?);
                    },
                    TokenTypes::GREATER => {
                        return Ok(self.comparision_lambda(expr, &greater)?);
                    },
                    TokenTypes::LESS => {
                        return Ok(self.comparision_lambda(expr, &lesser)?);
                    },
                    TokenTypes::GreaterEqual => {
                       return Ok(self.comparision_lambda(expr, &greaterequal)?);
                    },
                    TokenTypes::LessEqual => {
                        return Ok(self.comparision_lambda(expr, &lesserequal)?);
                    },
                    TokenTypes::AND => {
                        return Ok(self.logical_lambda(expr, true, &and)?);
                    },
                    TokenTypes::OR => {
                        return Ok(self.logical_lambda(expr, false, &or)?);
                    },
                    TokenTypes::NOT => {
                        if expr.len() > 1 {
                            return Err("Cannot Have more than 1 Arguement");
                        }
                        match &expr[0] {
                            Expression::Literal {token} => {
                                    if let TokenTypes::NIL = token._type {
                                        return Ok(true);
                                    } else {
                                        if let Ok(result) = self.visit_literal(&expr[0]) {
                                            if let Ok(result) = result.matchInteger() {
                                                if result == 0 {
                                                    return Ok(false);
                                                }
                                                return Ok(!(result != 0))
                                            }
                                        }
                                        Err("ERROR: In processing Literal")
                                    }
                                },
                                Expression::Logical { operator: _, expr: _ } => {
                                    return Ok(!(self.visit_logical(expr[0].clone())?));
                                },
                                _ => {
                                    return Ok(true)
                                }
                        }
                    }
                    _ => {
                        Err("ERROR: Unsupported Operator")
                    }
                }
            },
            _=> {
                Err("ERROR: Not a Logical Expression")
            }
        }
    }

    fn visit_literal(&self, lit: &Expression) -> Result<Values, &'static str> {
        match &lit {
            Expression::Literal {token} => {
                if let TokenTypes::Number = token._type {
                    return Ok(Values::Int(token.lexeme.parse().unwrap()));
                }
                Err("Should Be a Number Only!")
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                if let Ok(value) = self.visit_arithmetic(lit.clone()) {
                    return Ok(Values::Int(value));
                }
                Err("ERROR: Error Processing Arithmetic Operation")

            },
            Expression::Variable { name } => {
                if let Some(value) = self.locals.get(&name.lexeme) {
                    return Ok(value.clone());
                }

               return self.environment.get(name.lexeme.clone())
            }
            _ => {
                Err("Not a Literal!")
            }
        }
    }

    fn visit_arithmetic(&self, arith: Expression) -> Result<i64, &'static str> {
        let add = |a, b| a + b;
        let minus = |a, b| a - b;
        let multiply = |a, b| a * b;
        let divide = |a, b| a / b;
        let modulus = |a, b| a % b;
        match arith {
            Expression::Arithmetic { operator, expr } => {
                match operator._type {
                    TokenTypes::MAX => {
                        return Ok(self.artihmetic_lambda(expr, &cmp::max)?);
                    },
                    TokenTypes::MIN => {
                        return Ok(self.artihmetic_lambda(expr, &cmp::min)?);
                    },
                    TokenTypes::PLUS => {
                        return Ok(self.artihmetic_lambda(expr, &add)?);
                    },
                    TokenTypes::MINUS => {
                        return Ok(self.artihmetic_lambda(expr, &minus)?);
                    },
                    TokenTypes::STAR => {
                        return Ok(self.artihmetic_lambda(expr, &multiply)?);
                    },
                    TokenTypes::SLASH => {
                        return Ok(self.artihmetic_lambda(expr, &divide)?);
                    },
                    TokenTypes::MOD => {
                        return Ok(self.artihmetic_lambda(expr, &modulus)?);
                    },
                    _ => {
                        Err("ERROR: Not a Valid Type!")
                    }
                }
            }
            _ => {
                Err("ERROR: Not an Arithmetic Expression")
            }

        }
    }

    fn visit_local(&mut self, local: Expression) -> Result<(), &'static str>{
        if let Expression::Local{declarations, body} = local {
            for i in declarations {
                if let Expression::Assignment { name, expr } = i {
                    if let Expression::Variable { name } = *name {
                        self.locals.insert(name.lexeme, self.visit_literal(&*expr)?);
                    } else {
                        return Err("ERROR: Not a Variable in declared local block")
                    }
                } else {
                    return Err("ERROR: Not a Variable Assignment")
                }
            }

            for j in body {
                if let Err(value) = self.process(j) {
                    return Err(value)
                 }

            }
            for (key, value) in &self.locals {
                println!("{} => {}", key, value);
            }
            Ok(())
        } else {
            Err("ERROR: Wrong Type of Declaration")
        }
    }

    fn visit_set(&mut self, set: Expression) -> Result<(), &'static str>{
        match set {
            Expression::Set { declarations } => {
                for i in declarations {
                    match i {
                        Expression::Assignment { name, expr } => {
                            if let Expression::Variable { name } = *name {
                                self.locals.insert(name.lexeme, self.visit_literal(&*expr)?);
                            } else {
                                return Err("ERROR: Not a Variable");
                            }
                        },
                        _ => {
                            return Err("ERROR: Not an Assignment");
                        }
                    }
                }
                Ok(())
            },
            _ => {
                Err("ERROR: Invalid Operation")
            }
        }
    }


    fn visit_print(&mut self, print: Expression) -> Result<(), &'static str >{
        match print {
            Expression::Print { print } => {
                if let Some(value) = self.process(*print)? {
                    Ok(println!("{}", value))
                } else {
                    Err("ERROR: Cannot Read Value")
                }
            },
            _ => {
                Err("ERROR: Illegal Operation")
            }
        }
    }

    fn visit_if(&mut self, ifBlock: Expression) -> Result<(), &'static str>{
        match ifBlock {
            Expression::If {condition, body, then} => {
                match *condition {
                    Expression::Logical {operator: _, expr: _} => {
                        let cond = self.visit_logical(*condition)?;
                        if cond == true {
                            if let Err(value) = self.process(*body) {
                                Err(value)
                            } else {Ok(())}
                        } else {
                            if let Some(thenBody) = then {
                                if let Err(value) = self.process(*thenBody) {
                                    Err(value)
                                } else {Ok(())}
                            } else {Ok(())}

                        }
                    },
                    _ => {
                        Err("ERROR: Not a logical condition")
                    }
                }
            },
            _ => {
                Err("ERROR: Invalid Type of Operator")
            }
        }
    }

    fn visit_string(&mut self, string: Expression) -> Result<String, &'static str>{
        match string {
            Expression::Literal{token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    return Ok(token.lexeme);
                } else {
                    Err("Not a string")
                }
            }
            _ => {
                Err("Not a String")
            }
        }
    }
    fn visit_for(&mut self, loopExpr: Expression) -> Result<(), &'static str>{
        match loopExpr {
            Expression::Loop {variable, start, end, body} => {
                match *variable {
                    Expression::Variable {name} => {
                        let startInt = (self.visit_literal(&*start)?).matchInteger()?;
                        let endInt = (self.visit_literal(&*end)?).matchInteger()?;
                        for i in startInt..endInt {
                            self.locals.insert(name.lexeme.clone(), Values::Int(i));
                            for j in &body {
                                if let Err(value) = self.process(j.clone()) {
                                    return Err(value)
                                }
                            }
                        }
                        Ok(())
                    },
                    _ => {Err("ERROR: Incorrect Variable")}
                }
            },
            _ => {Err("ERROR: Not a For Loop")}
        }

    }
    fn visit_global(&mut self, global: Expression) -> Result<(), &'static str>{
        if let Expression::Global { name, expr } = global {
            if let Expression::Variable { name } = *name {
                let value = self.visit_literal(&*expr)?;
                let nameVar = name.lexeme.clone();
                self.environment.define(nameVar, value.clone());
                return Ok(());
            }
            Err("ERROR: Not a Variable")
        } else {
            Err("ERROR: Not a GLobal Variable Declaration")
        }
    }
}

