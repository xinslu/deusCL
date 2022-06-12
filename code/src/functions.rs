use crate::types::Error;
use crate::expression::Expression;

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
