use crate::types::{
    Errors, TokenTypes
};
pub struct Parser {

}

impl Parser{
    pub fn parser<'a>(tokenized: &'a [String]) -> Result<(TokenTypes, &'a [String]), Errors> {
        let (token, rest) = tokenized.split_first().ok_or(Errors::ParseError("Couldn't get token!  ".to_string()))?;
        println!("{:?}",token);
        println!("{:?}",rest);
        match &token[..] {
        // "(" => read_sequence(rest),
        ")" => Err(Errors::ReadLineError("Unexpected \")\"".to_string())),
        _ => Ok((TokenTypes::Number, rest))
        }
  }

    // fn read_sequence<'a>(tokens: &'a [String]) -> Result<(TokenTypes, &'a [String]), Errors> {
    //     let mut res: Vec<TokenTypes> = vec![];
    //     let mut xs = tokens;
    //     loop {
    //       let (next_token, rest) = xs.split_first().ok_or(Errors::ReadLineError("could not find closing \")\"".to_string()))?;
    //       if next_token == ")" {
    //         return Ok()
    //       }
    //       let (exp, new_xs) = parser(&xs)?;
    //       res.push(exp);
    //       xs = new_xs;
    //     }
    // }
}

