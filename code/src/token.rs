use core::fmt::Formatter;

use crate::types::{
    TokenTypes
};

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
