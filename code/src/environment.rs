use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Values {
    Int(i64),
    Str(String),
    Bool(bool)
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
    pub fn matchInteger(&self) -> i64 {
        match self {
            Values::Int(integer) => {
                return *integer;
            },
            _ => {
                panic!("Wrong Type Excepted Integer");
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


    pub fn get(&self, name: String) -> Values {
        match self.map.get(&name) {
            Some(value) => { value.clone() },
            None => {
                match &*self.enclosing {
                    Some(enclosing) => {
                        return enclosing.get(name);
                    },
                    None => {
                        panic!("Variable not Defined")
                    }
                }
            }
        }
    }

    pub fn assign<T>(&mut self, name: String, value: T) where T:  Encapsulation {
        if let Some(newValue) = self.map.get_mut(&name) {
                *newValue = value.return_value();
        } else {
            match &mut *self.enclosing {
                Some(enclosing) => {
                    enclosing.assign(name, value);
                },
                None => {
                    panic!("Variable not Defined")
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

    pub fn getAt(&mut self, name: String, distance : i32) -> Values {
        if let Some(value) = self.ancestor(distance).map.get(&name) {
            return value.clone();
        } else {
            panic!("Variable Not Found!");
        }
    }


    pub fn assignAt<T: 'static>(&mut self, name: String, value: T, distance : i32) where T:  Encapsulation {
        let mut environment = &mut *self;
        for _i in 0..distance {
            match &mut *environment.enclosing{
                Some(enclosing) => {
                    environment = enclosing
                },
                _ => {
                    panic!("Wrong Distance");
                }
            }
        }
        environment.assign(name,value);
    }
}
