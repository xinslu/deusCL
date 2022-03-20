use crate::types::{
    Errors, TokenTypes
};
pub struct Parser {
    current: i32,
    token_list: Vec<TokenTypes>
}

impl Parser{
    pub fn create(tokens: Vec<TokenTypes>) -> Parser{
        Parser {
            current: 0,
            token_list: tokens
        }
    }
    pub fn parser(&mut self) -> Result<&Vec<TokenTypes>, Errors> {
        let (token, rest) = self.token_list.split_first().ok_or(Errors::ParseError("Couldn't get token!  ".to_string()))?;
        println!("{:?}",token);
        println!("{:?}",rest);
        Ok(&self.token_list)
  }
}

