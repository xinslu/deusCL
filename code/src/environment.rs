use std::collections::HashMap;
use std::fmt;
use crate::types::Error;
use crate::expression::Expression;
use crate::functions::Function;

#[derive(Debug, Clone)]
pub enum Values {
    Int(i64),
    Str(String),
    Bool(bool),
    Function(Expression)
}

pub trait Encapsulation {
    fn return_value(&self) -> Values;
}

impl Encapsulation for i64 {
    fn return_value(&self) -> Values {
        return Values::Int(*self);
    }

}

impl Encapsulation for String {
    fn return_value(&self) -> Values {
        return Values::Str(self.to_string());
    }
}

impl Encapsulation for bool {
    fn return_value(&self) -> Values {
        return Values::Bool(*self);
    }
}


impl Values {
    pub fn matchInteger(&self) -> Result<i64,Error> {
        if let Values::Int(integer) = self {
            return Ok(*integer);
        } else {
            Err(Error::Reason("Wrong Type Excepted Integer".to_string()))
        }
    }


    pub fn matchString(&self) -> Result<String,Error> {
        if let Values::Str(string) = self {
            return Ok(string.clone());
        } else {
            Err(Error::Reason("Wrong Type Excepted String".to_string()))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Values::Int(integer) => return integer.to_string(),
            Values::Str(string) => return string.to_string(),
            Values::Bool(boolean) => return boolean.to_string(),
            _ => {
                return "Cannot convert to string".to_string()
            }
        }
    }
}



impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Values::Int(integer) => {
                write!(f, "{}", integer)
            },
            Values::Str(string) => {
                write!(f, "{}", string)
            },
            Values::Bool(boolean) => {
                write!(f, "{}", boolean)
            }
            Values::Function(function) => {
                write!(f, "{:?}", function)
            }
        }

    }
}

pub use crate::token:: Token;
pub struct Environment {
    map: HashMap<String, Values>,
    enclosing: Box<Option<Environment>>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Environment{
        Environment {
            map: HashMap::new(),
            enclosing: Box::new(enclosing)
        }
    }

    pub fn define(&mut self, name: String, value: Values) {
        self.map.insert(name, value);
    }


    pub fn get(&self, name: String) -> Result<Values, Error> {
        match self.map.get(&name) {
            Some(value) => { Ok(value.clone()) },
            None => {
                match &*self.enclosing {
                    Some(enclosing) => {
                        return enclosing.get(name);
                    },
                    None => {
                        Err(Error::Reason(format!("Variable {} is not Defined", name)))
                    }
                }
            }
        }
    }

    pub fn assign<T>(&mut self, name: String, value: T) -> Result<(), Error> where T:  Encapsulation {
        if let Some(newValue) = self.map.get_mut(&name) {
                *newValue = value.return_value();
                Ok(())
        } else {
            match &mut *self.enclosing {
                Some(enclosing) => {
                    enclosing.assign(name, value)?;
                    return Ok(());
                },
                None => {
                    Err(Error::Reason("Variable not Defined".to_string()))
                }
            }
        }
    }

    pub fn ancestor(&mut self, distance : i32) -> &Environment {
        let mut environment = &*self;
        for _i in 0..distance {
            match &*environment.enclosing{
                Some(enclosing) => {
                    environment = &enclosing
                },
                _ => {
                    return environment;
                }
            }
        }
        return environment
    }

    pub fn getAt(&mut self, name: String, distance : i32) -> Result<Values, Error> {
        if let Some(value) = self.ancestor(distance).map.get(&name) {
            return Ok(value.clone());
        }
        Err(Error::Reason("Variable Not Found!".to_string()))
    }


    pub fn assignAt<T: 'static>(&mut self, name: String, value: T, distance : i32) -> Result<(), Error>where T:  Encapsulation {
        let mut environment = &mut *self;
        for _i in 0..distance {
            match &mut *environment.enclosing{
                Some(enclosing) => {
                    environment = enclosing
                },
                _ => {
                    return Err(Error::Reason("Wrong Distance".to_string()));
                }
            }
        }
        environment.assign(name,value)?;
        Ok(())
    }
}
