pub use crate::types::{
    TokenTypes
};
pub use crate::token:: {
    Token
};
pub struct Tokenizer {
    line: Vec<String>,
    tokens:Vec<Token>,
    current: i32,
    start: i32,
}

impl Tokenizer{
    pub fn create() -> Tokenizer{
        Tokenizer {
            line: vec![],
            tokens: vec![],
            current: 0,
            start: 0
        }
    }

    pub fn print_tokens(&mut self) {
        println!("{:?}", self.tokens);
    }

    pub fn tokenize(&mut self, expr: String) -> &Vec<String> {
        self.line = expr.replace("(", " ( ").replace(")", " ) ").split_whitespace().map(|x| x.to_string()).collect();
        for _item in self.line.iter() {
            if Tokenizer::is_digit(_item.to_string()) {
                let text: String = self.line[self.current as usize].clone();
                println!("{:?}", self.line);
                self.tokens.push(Token {_type: TokenTypes::Number, lexeme: text});
            }
        }
        return &self.line;
    }


    pub fn is_at_end(&mut self) -> bool {
        return self.current >= self.line.len() as i32;
    }


    pub fn advance(&mut self) -> &String {
        self.current += 1;
        return &self.line[self.current as usize]
    }

    pub fn r#match(&mut self, expected: String) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.line[self.current as usize] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    pub fn match_next(&mut self, expected: String) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.line[(self.current + 1) as usize] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    pub fn peek(&mut self) -> &String {
        return &self.line[self.current as usize];
    }


    pub fn peek_next(&mut self) -> &String {
        return &self.line[(self.current+1) as usize];
    }

    pub fn add_token(&mut self, token_type: TokenTypes) {
        let text: String = self.line[self.current as usize].clone();
        self.tokens.push(Token {_type: token_type, lexeme: text})
    }


    pub fn is_digit(number: String) -> bool {
        for character in number.chars() {
            if !character.is_ascii_digit() {
                return false;
            }
        }
        return true;
    }
}


