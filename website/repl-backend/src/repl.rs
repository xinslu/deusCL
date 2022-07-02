use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use std::io::{stdin, stdout, Write};
pub fn repl(s: String, interpreter: &mut Interpreter) -> String {
    let mut tokenizer = Tokenizer::new();
    let processed_line = str::replace(s.as_str(), "\n", " ");
    if let Err(error) = tokenizer.tokenize(processed_line.to_string()) {
        format!("{}", error)
    } else {
        // println!("created parser");
        // tokenizer.print_tokens();
        // println!("right after print");
        match Parser::new(tokenizer.tokens).parse() {
            Ok(parserresult) => {
                // println!("{:?}", parserresult);
                match interpreter.process(parserresult.clone()) {
                    Ok(optional) => match optional {
                        Some(result) => return format!("{}", result),
                        None => {
                            return "NOP".to_string();
                        }
                    },
                    Err(error) => {
                        format!("{}", error)
                    }
                }
            }
            Err(error) => {
                format!("{}", error)
            }
        }
    }
}
