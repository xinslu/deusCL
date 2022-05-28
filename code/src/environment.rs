use std::collections::HashMap;
use std::fmt::Display;

pub struct Environment<'a> {
    map: HashMap<&'a str, Box<dyn Display + 'static>>,
    enclosing: Box<Option<Environment<'a>>>
}

impl Environment<'_> {
    pub fn create(enclosing: Option<Environment>) -> Environment{
        Environment {
            map: HashMap::new(),
            enclosing: Box::new(enclosing)
        }
    }
}
