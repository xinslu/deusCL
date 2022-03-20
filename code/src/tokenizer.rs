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
        println!("{:?}", self.line);
        while !self.is_at_end() {
            let text: String = self.advance().to_string();
            match &text[..] {
                "+" => self.tokens.push(Token {_type: TokenTypes::PLUS, lexeme: text}),
                "-" => self.tokens.push(Token {_type: TokenTypes::MINUS, lexeme: text}),
                "*" => self.tokens.push(Token {_type: TokenTypes::STAR, lexeme: text}),
                "/" => self.tokens.push(Token {_type: TokenTypes::SLASH, lexeme: text}),
                "/=" => self.tokens.push(Token {_type: TokenTypes::SLASHEQUAL, lexeme: text}),
                "=" => self.tokens.push(Token {_type: TokenTypes::EQUAL, lexeme: text}),
                ">" => self.tokens.push(Token {_type: TokenTypes::GREATER, lexeme: text}),
                "<" => self.tokens.push(Token {_type: TokenTypes::LESS, lexeme: text}),
                ">=" => self.tokens.push(Token {_type: TokenTypes::GreaterEqual, lexeme: text}),
                "<=" => self.tokens.push(Token {_type: TokenTypes::LessEqual, lexeme: text}),
                "(" => self.tokens.push(Token {_type: TokenTypes::LeftParen, lexeme: text}),
                ")" => self.tokens.push(Token {_type: TokenTypes::RightParen, lexeme: text}),
                _ => {
                    if Tokenizer::is_digit(text.to_string()) {
                        self.tokens.push(Token {_type: TokenTypes::Number, lexeme: text});
                    } else {
                        println!("Something wrong {}", text);
                    }
                }
            }
        }
        return &self.line;
    }


    pub fn is_at_end(&mut self) -> bool {
        return self.current >= self.line.len() as i32;
    }


    pub fn advance(&mut self) -> &String {
        if self.current < self.line.len() as i32 {
            self.current += 1;
            return &self.line[(self.current-1) as usize]
        }
        return &self.line[(self.line.len() - 1) as usize]
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


