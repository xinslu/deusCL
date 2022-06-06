use core::fmt::Formatter;

use std::fmt;

use crate::types::TokenTypes;
#[derive(Clone)]
pub struct Token {
    pub _type: TokenTypes,
    pub lexeme: String
}

impl std::fmt::Debug for Token {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        print!("{} {:?}",self.lexeme, self._type);
        Ok(())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of type {}", self.lexeme, self._type)

    }
}
