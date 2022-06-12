use crate::environment::Values;
use crate::interpreter::Interpreter;
use crate::types::Error;
use crate::expression::Expression;
use crate::environment::Environment;

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Expression
}

impl Function {
    pub fn new(declaration: Expression) -> Result<Function, Error> {
        if let Expression::Function {..} = declaration {
            Ok(Function {
                declaration
            })
        } else {
            return Err(Error::Reason("Invalid Function Declaration".to_string()))
        }

    }

    pub fn arity(&mut self) -> Result<i64, Error> {
        if let Expression::Function {name: _, parameters, body: _} = &self.declaration {
            return Ok(parameters.len() as i64)
       } else {
            return Err(Error::Reason("Invalid Function Declaration".to_string()))
       }
    }

    pub fn call(&mut self, mut intptr: Interpreter, arguements: Vec<Values>) -> Result<Values, Error>{
        let mut environment = Environment::new(Some(intptr.environment.clone()));
        if let Expression::Function {name: _, parameters, body} = &self.declaration {
            for (token, arg) in parameters.iter().zip(arguements.iter()) {
                if let Expression::Variable {name} = token{
                    environment.define(name.lexeme.clone(), arg.clone())
                }
            }
            match intptr.accept(*body.clone()) {
                Err(error) => {
                    match error {
                        Error::Reason(x)=> {
                            return Err(Error::Reason(x))
                        }
                        Error::Return(x) => {
                            return Ok(x)
                        }
                    }
                },
                _ => {
                    Ok(Values::None)
                }
            }
        } else {
            return Err(Error::Reason("Invalid Function Declaration".to_string()))
        }
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        if let Expression::Function {name, parameters: _, body: _} = &self.declaration {
            return "<Function \' ".to_owned() + &name.lexeme + "\'>";
        } else {
            return "<Function \' \'>".to_string()
        }
    }
}
