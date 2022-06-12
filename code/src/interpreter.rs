use crate::environment::Encapsulation;
use crate::environment::Values;
use crate::{expression::Expression, types::{TokenTypes, Error}, environment::Environment, visitors::Visitor};

use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use std::any::Any;
#[derive(Debug, Clone)]
pub struct Interpreter {
    pub environment: Environment,
    locals: HashMap<String, Values>
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(None),
            locals: HashMap::new()
        }
    }
    pub fn accept(&mut self, expression: Expression ) -> Result<(), Error>{
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
            },
            Expression::Block {..} => {
                self.visit_block(expression)?;
                Ok(())
            },
            Expression::StringMan {..} => {
                println!("{:?}", self.visit_string(expression)?);
                Ok(())
            },
            Expression::Return {..} => {
                println!("{:?}", self.visit_return(expression)?);
                Ok(())
            },
            Expression::Function {..} => {
                self.visit_function_dec(expression)?;
                Ok(())
            },
            Expression::Call {..} => {
                self.visit_call(expression)?;
                Ok(())
            }
            _ => {
                Err(Error::Reason(format!("{:?} Unsupported Operation Right Now", expression)))
            }
        }
    }

    pub fn process(&mut self, expression: Expression ) -> Result<Option<Values>, Error> {
        match expression {
            Expression::Logical {operator: _, expr: _} => {
                Ok(Some((self.visit_logical(expression)?).return_value()))
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                return Ok(Some( self.visit_arithmetic(expression)?.return_value()));
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
            Expression::Loop {..} => {
                if let Err(error) = self.visit_for(expression) {
                    return Err(error);
                }
                Ok(None)
            },
            Expression::Block {..} => {
                self.visit_block(expression)?;
                Ok(None)
            },
            Expression::StringMan {..} => {
                Ok(Some(self.visit_string_man(expression)?.return_value()))
            },
            Expression::Return {..} => {
                self.visit_return(expression)?;
                Ok(None)
            },
            Expression::Function {..} => {
                self.visit_function_dec(expression)?;
                Ok(None)
            },
            Expression::Call {..} => {
               Ok(Some(self.visit_call(expression)?))
            }
            _ => {
               Err(Error::Reason(format!("{:?} Unsupported Operation Right Now", expression)))
            }
        }
    }

    pub fn evaluateFunction(&mut self, statements: Expression, environment: Environment) -> Result<Values, Error>{
        let previous = self.environment.clone();
        self.environment = environment;
        match self.accept(statements.clone()) {
                Err(error) => {
                    match error {
                        Error::Reason(x)=> {
                            self.environment = previous;
                            return Err(Error::Reason(x))
                        }
                        Error::Return(x) => {
                            self.environment = previous;
                            return Err(Error::Return(x))
                        }
                    }
                },
                _ => {
                    self.environment = previous;
                    Ok(Values::None)
                }
        }
    }

    pub fn clean_env(&mut self) {
        self.locals = HashMap::new();
    }

    pub fn comparision_lambda(&mut self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> bool) -> Result<bool,Error>  {
        if expr.len() as i32 == 0 {
            return Err(Error::Reason("Parsing Error, not enough arguements".to_string()))
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

    pub fn logical_lambda(&mut self, expr: Vec<Expression>, mut rBool: bool,func: &dyn Fn(bool, bool) -> bool) -> Result<bool, Error>  {
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

    pub fn artihmetic_lambda(&self, expr: Vec<Expression>, func: &dyn Fn(i64, i64) -> i64) -> Result<i64, Error>  {
        let mut temp = (self.visit_literal(&expr[0])?).matchInteger()?;
        for i in &expr[1..] {
            temp = func(temp, (self.visit_literal(i)?).matchInteger()?);
        }
        return Ok(temp);
    }


    pub fn string_lambda(&self, expr: Vec<Expression>, func: &dyn Fn(String, String) -> String) -> Result<String, Error>  {
        let mut temp = (self.visit_literal(&expr[0])?).to_string();
        for i in &expr[1..] {
            temp = func(temp, (self.visit_literal(i)?).to_string());
        }
        return Ok(temp);
    }

}
impl Visitor for Interpreter {
    fn visit_logical(&mut self, log: Expression) -> Result<bool, Error> {
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
                            return Err(Error::Reason("Cannot Have more than 1 Arguement".to_string()))
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
                                        Err(Error::Reason("In processing Literal".to_string()))
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
                        Err(Error::Reason("Unsupported Operator".to_string()))
                    }
                }
            },
            _=> {
                Err(Error::Reason("Not a Logical Expression".to_string()))
            }
        }
    }

    fn visit_literal(&self, lit: &Expression) -> Result<Values, Error> {
        match &lit {
            Expression::Literal {token} => {
                if let TokenTypes::Number = token._type {
                    return Ok(Values::Int(token.lexeme.parse().unwrap()));
                } else if let TokenTypes::STRINGLITERAL = token._type {
                    return Ok(Values::Str(token.lexeme.parse().unwrap()));
                }
                Err(Error::Reason(format!("Unsupported Assignment to {}", token)))
            },
            Expression::Arithmetic { operator: _, expr: _ } => {
                return Ok(Values::Int(self.visit_arithmetic(lit.clone())?));
            },
            Expression::Variable { name } => {
                if let Some(value) = self.locals.get(&name.lexeme) {
                    return Ok(value.clone());
                }

               return self.environment.get(name.lexeme.clone())
            }
            _ => {
                Err(Error::Reason("Not a Literal!".to_string()))
            }
        }
    }

    fn visit_arithmetic(&self, arith: Expression) -> Result<i64, Error> {
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
                        Err(Error::Reason("Not a Valid Type!".to_string()))
                    }
                }
            }
            _ => {
                Err(Error::Reason("Not an Arithmetic Expression".to_string()))
            }

        }
    }

    fn visit_local(&mut self, local: Expression) -> Result<(), Error>{
        if let Expression::Local{declarations, body} = local {
            for i in declarations {
                if let Expression::Assignment { name, expr } = i {
                    if let Expression::Variable { name } = *name {
                        self.locals.insert(name.lexeme, self.visit_literal(&*expr)?);
                    } else {
                        return Err(Error::Reason("Not a Variable in declared local block".to_string()))
                    }
                } else {
                    return Err(Error::Reason("Not a Variable Assignment".to_string()))
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
            Err(Error::Reason("Wrong Type of Declaration".to_string()))
        }
    }

    fn visit_set(&mut self, set: Expression) -> Result<(), Error>{
        match set {
            Expression::Set { declarations } => {
                for i in declarations {
                    match i {
                        Expression::Assignment { name, expr } => {
                            if let Expression::Variable { name } = *name {
                                self.locals.insert(name.lexeme, self.visit_literal(&*expr)?);
                            } else {
                                return Err(Error::Reason("Not a Variable".to_string()));
                            }
                        },
                        _ => {
                            return Err(Error::Reason("Not an Assignment".to_string()));
                        }
                    }
                }
                Ok(())
            },
            _ => {
                Err(Error::Reason("Invalid Operation".to_string()))
            }
        }
    }


    fn visit_print(&mut self, print: Expression) -> Result<(), Error >{
        match print {
            Expression::Print { print } => {
                if let Some(value) = self.process(*print)? {
                    Ok(println!("{}", value))
                } else {
                    Err(Error::Reason("Cannot Read Value".to_string()))
                }
            },
            _ => {
                Err(Error::Reason("Illegal Operation".to_string()))
            }
        }
    }

    fn visit_if(&mut self, ifBlock: Expression) -> Result<(), Error>{
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
                        Err(Error::Reason("Not a logical condition".to_string()))
                    }
                }
            },
            _ => {
                Err(Error::Reason("Invalid Type of Operator".to_string()))
            }
        }
    }

    fn visit_string(&mut self, string: Expression) -> Result<String, Error>{
        match string {
            Expression::Literal{token} => {
                if let TokenTypes::STRINGLITERAL = token._type {
                    return Ok(token.lexeme);
                } else {
                    Err(Error::Reason("Not a string".to_string()))
                }
            }
            _ => {
                Err(Error::Reason("Not a String".to_string()))
            }
        }
    }
    fn visit_for(&mut self, loopExpr: Expression) -> Result<(), Error>{
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
                    _ => {Err(Error::Reason("Not an Intializer Variable".to_string()))}
                }
            },
            _ => {Err(Error::Reason("Not a For Loop".to_string()))}
        }

    }
    fn visit_global(&mut self, global: Expression) -> Result<(), Error>{
        if let Expression::Global { name, expr } = global {
            if let Expression::Variable { name } = *name {
                let value = self.visit_literal(&*expr)?;
                let nameVar = name.lexeme.clone();
                self.environment.define(nameVar, value.clone());
                return Ok(());
            }
            Err(Error::Reason("Not a Variable".to_string()))
        } else {
            Err(Error::Reason("Not a Global Variable Declaration".to_string()))
        }
    }
    fn visit_block(&mut self, block: Expression) -> Result<(), Error> {
        if let Expression::Block {expressions} = block {
            for i in expressions {
                self.process(i)?;
            }
            Ok(())
        } else {
            return Err(Error::Reason("Cannot Evaluate a Block".to_string()))
        }
    }

    fn visit_string_man(&mut self, string: Expression) -> Result<String, Error> {
        let add = |a: String,b: String| format!("{}{}",a, b);
        if let Expression::StringMan {operator, expr } = string {
            match operator._type {
                TokenTypes::CONCAT => {
                    return Ok(self.string_lambda(expr, &add)?);
                },
                _ => {
                    return Err(Error::Reason("Unsupported String Operation".to_string()));
                }
            }
        } else {
            return Err(Error::Reason("Has to be a string operation".to_string()));
        }
    }
    fn visit_return(&mut self, ret: Expression) -> Result<(),Error> {
        if let Expression::Return {result } = ret {
            match self.process(*result)? {
                Some(x) => {
                    return Err(Error::Return(x))
                },
                None => {
                    return Err(Error::Reason("Return cannot be empty".to_string()))
                }
            }
        } else {
            return Err(Error::Reason("Has to be a return operation".to_string()))
        }
    }

    fn visit_function_dec(&mut self, func: Expression) -> Result<(), Error> {
        if let Expression::Function { ref name, ..} = func {
            self.environment.define(name.to_string(), Values::Function(func));
            Ok(())
        } else {
            return Err(Error::Reason("Invalid Functional Declaration".to_string()))
        }
    }

    fn visit_call(&mut self, call: Expression) -> Result<Values, Error> {
        if let Expression::Call {name, parameters} = call {
            let mut function = self.environment.get(name.to_string())?.matchFunction()?;
            if parameters.len() as i64 == function.arity()? {
                let mut arguements = Vec::new();
                for i in parameters {
                    match self.process(i.clone())? {
                        Some(x) => arguements.push(x),
                        None => return Err(Error::Reason(format!("Function Argument {:?} does not evaluate to a valid value", i)))
                    }
                }
                let result = function.call(self.clone(), arguements )?;
                return Ok(result)
            } else {
                return Err(Error::Reason(format!("Invalid Call to function, expected {} arguements got {}", function.arity()?, parameters.len())))
            }
        } else {
            return Err(Error::Reason("Invalid Call to function".to_string()))
        }
    }
}

