pub use crate::token::Token;
pub use crate::types::{Error, TokenTypes};
pub struct Tokenizer {
    line: Vec<String>,
    pub tokens: Vec<Token>,
    current: i32,
    start: i32,
}
use regex::Regex;

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            line: vec![],
            tokens: vec![],
            current: 0,
            start: 0,
        }
    }

    pub fn print_tokens(&mut self) {
        println!("{:?}", self.tokens);
    }

    pub fn tokenize(&mut self, expr: String) -> Result<&Vec<String>, Error> {
        self.line = expr
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        while !self.is_at_end() {
            let text: String = self.advance().to_string();
            match &text[..] {
                "+" => self.tokens.push(Token {
                    _type: TokenTypes::PLUS,
                    lexeme: text,
                }),
                "-" => self.tokens.push(Token {
                    _type: TokenTypes::MINUS,
                    lexeme: text,
                }),
                "*" => self.tokens.push(Token {
                    _type: TokenTypes::STAR,
                    lexeme: text,
                }),
                "/" => self.tokens.push(Token {
                    _type: TokenTypes::SLASH,
                    lexeme: text,
                }),
                "/=" => self.tokens.push(Token {
                    _type: TokenTypes::SLASHEQUAL,
                    lexeme: text,
                }),
                "=" => self.tokens.push(Token {
                    _type: TokenTypes::EQUAL,
                    lexeme: text,
                }),
                ">" => self.tokens.push(Token {
                    _type: TokenTypes::GREATER,
                    lexeme: text,
                }),
                "<" => self.tokens.push(Token {
                    _type: TokenTypes::LESS,
                    lexeme: text,
                }),
                ">=" => self.tokens.push(Token {
                    _type: TokenTypes::GreaterEqual,
                    lexeme: text,
                }),
                "<=" => self.tokens.push(Token {
                    _type: TokenTypes::LessEqual,
                    lexeme: text,
                }),
                "(" => self.tokens.push(Token {
                    _type: TokenTypes::LeftParen,
                    lexeme: text,
                }),
                ")" => self.tokens.push(Token {
                    _type: TokenTypes::RightParen,
                    lexeme: text,
                }),
                "%" => self.tokens.push(Token {
                    _type: TokenTypes::MOD,
                    lexeme: text,
                }),
                "^+" => self.tokens.push(Token {
                    _type: TokenTypes::CONCAT,
                    lexeme: text,
                }),
                "max" => self.tokens.push(Token {
                    _type: TokenTypes::MAX,
                    lexeme: text,
                }),
                "min" => self.tokens.push(Token {
                    _type: TokenTypes::MIN,
                    lexeme: text,
                }),
                "and" => self.tokens.push(Token {
                    _type: TokenTypes::AND,
                    lexeme: text,
                }),
                "or" => self.tokens.push(Token {
                    _type: TokenTypes::OR,
                    lexeme: text,
                }),
                "not" => self.tokens.push(Token {
                    _type: TokenTypes::NOT,
                    lexeme: text,
                }),
                "nil" => self.tokens.push(Token {
                    _type: TokenTypes::NIL,
                    lexeme: text,
                }),
                "let" => self.tokens.push(Token {
                    _type: TokenTypes::LET,
                    lexeme: text,
                }),
                "NIL" => self.tokens.push(Token {
                    _type: TokenTypes::NIL,
                    lexeme: text,
                }),
                "set" => self.tokens.push(Token {
                    _type: TokenTypes::SET,
                    lexeme: text,
                }),
                "print" => self.tokens.push(Token {
                    _type: TokenTypes::PRINT,
                    lexeme: text,
                }),
                "if" => self.tokens.push(Token {
                    _type: TokenTypes::IF,
                    lexeme: text,
                }),
                "loop" => self.tokens.push(Token {
                    _type: TokenTypes::LOOP,
                    lexeme: text,
                }),
                "return" => self.tokens.push(Token {
                    _type: TokenTypes::RETURN,
                    lexeme: text,
                }),
                "defun" => self.tokens.push(Token {
                    _type: TokenTypes::DEFUN,
                    lexeme: text,
                }),
                "call" => self.tokens.push(Token {
                    _type: TokenTypes::CALL,
                    lexeme: text,
                }),
                "var" => self.tokens.push(Token {
                    _type: TokenTypes::VAR,
                    lexeme: text,
                }),
                _ => {
                    if Tokenizer::is_digit(text.to_string()) {
                        self.tokens.push(Token {
                            _type: TokenTypes::Number,
                            lexeme: text,
                        });
                    } else if text.chars().nth(0).unwrap() == '\"' {
                        if let Some(value) = text.chars().last() {
                            if value == '\"' && text.len() != 1 {
                                let len = text.len();
                                let slice = &text[1..(len - 1)];
                                self.tokens.push(Token {
                                    _type: TokenTypes::STRINGLITERAL,
                                    lexeme: slice.to_string(),
                                });
                            } else {
                                self.current -= 1;
                                let _slice: &str = "";
                                if self.peek() == "\"" {
                                    self.current += 1;
                                    let multiWordString = self.mutliWordString();
                                    let len = multiWordString.len();
                                    let _slice = &multiWordString[0..(len - 1)];
                                } else {
                                    let multiWordString = self.mutliWordString();
                                    let len = multiWordString.len();
                                    let _slice = &multiWordString[1..(len - 1)];
                                }

                                self.tokens.push(Token {
                                    _type: TokenTypes::STRINGLITERAL,
                                    lexeme: _slice.to_string(),
                                });
                            }
                        }
                    } else if text.chars().all(char::is_alphanumeric) {
                        self.tokens.push(Token {
                            _type: TokenTypes::IDENTIFIER,
                            lexeme: text,
                        });
                    } else {
                        return Err(Error::Reason(format!("Cannot Recognize token {}", text)));
                    }
                }
            }
        }
        return Ok(&self.line);
    }

    pub fn mutliWordString(&mut self) -> String {
        let mut string: String = "".to_string();
        let mut text: String;
        loop {
            text = self.advance().to_string();
            println!("{:?}", text);
            if self.is_at_end() {
                break;
            }
            if text.clone().chars().nth(text.len() - 1).unwrap() == '\"' {
                string += &text;
                break;
            }
            string += &(text + " ");
        }
        return string;
    }

    pub fn is_at_end(&mut self) -> bool {
        return self.current >= self.line.len() as i32;
    }

    pub fn advance(&mut self) -> &String {
        if self.current < self.line.len() as i32 {
            self.current += 1;
            return &self.line[(self.current - 1) as usize];
        }
        return &self.line[(self.line.len() - 1) as usize];
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
        return &self.line[(self.current + 1) as usize];
    }

    pub fn add_token(&mut self, token_type: TokenTypes) {
        let text: String = self.line[self.current as usize].clone();
        self.tokens.push(Token {
            _type: token_type,
            lexeme: text,
        })
    }

    pub fn is_digit(mut number: String) -> bool {
        if number.chars().nth(0).unwrap() == '-' {
            number = number[1..].to_string();
        }
        for character in number.chars() {
            if !character.is_ascii_digit() && character != '.' {
                return false;
            }
        }
        return true;
    }
}
