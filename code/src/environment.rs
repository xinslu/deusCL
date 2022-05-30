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


    pub fn get(&self, name: String) -> Box<dyn Display + '_> {
        match self.map.get(&name) {
            Some(value) => return Box::new(value),
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

    pub fn assign<T: 'static>(&mut self, name: String, value: T) where T:  Display {
        if let Some(newValue) = self.map.get_mut(&name) {
                *newValue = Box::new(value);
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

    pub fn getAt(&mut self, name: String, distance : i32) -> Box<dyn Display + '_> {
        if let Some(value) = self.ancestor(distance).map.get(&name) {
            return Box::new(value);
        } else {
            panic!("Variable Not Found!");
        }
    }


    pub fn assignAt<T: 'static>(&mut self, name: String, value: T, distance : i32) where T:  Display {
        let mut environment = &mut *self;
        for _i in 0..distance {
            match &mut *environment.enclosing{
                Some(enclosing) => {
                    environment = &mut *enclosing
                },
                _ => {
                    panic!("Wrong Distance");
                }
            }
        }
        environment.assign(name,value);
    }

}
