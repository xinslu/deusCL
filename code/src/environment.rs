use std::collections::HashMap;
use std::fmt::Display;

pub use crate::token:: {
    Token
};
pub struct Environment {
    map: HashMap<String, Box<dyn Display + 'static>>,
    enclosing: Box<Option<Environment>>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Environment{
        Environment {
            map: HashMap::new(),
            enclosing: Box::new(enclosing)
        }
    }

    pub fn define<T: 'static>(&mut self, name: String, value: T) where T:  Display {
        self.map.insert(name, Box::new(value));
    }


    pub fn get(&mut self, name: String) -> Box<dyn Display + '_> {
        match self.map.get(&name) {
            Some(value) => return Box::new(value),
            None => {
                match *self.enclosing {
                    Some(_) => {
                        return (*self.enclosing).as_ref().unwrap().get(name);
                    },
                    None => {
                        panic!("Variable not Defined")
                    }
                }
                }
            }

    }
}
