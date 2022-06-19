use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use std::io::{stdin,stdout,Write};
pub fn repl() {
    let mut _interpreter = Interpreter::new();
    loop {
        let mut tokenizer = Tokenizer::new();
        let mut s=String::new();
        print!("DEUS_USER> ");
        let _=stdout().flush();
        match stdin().read_line(&mut s) {
            Ok(line) => {
                let processed_line = str::replace(s.as_str(), "\n", " ");
                if let Err(error) = tokenizer.tokenize(processed_line.to_string()) {
                    println!("{}", error);
                } else {
                    // println!("created parser");
                    // tokenizer.print_tokens();
                    // println!("right after print");
                    match Parser::new(tokenizer.tokens).parse() {
                        Ok(parserresult) => {
                            // println!("{:?}", parserresult);
                            if let Err(error) = _interpreter.accept(parserresult.clone()) {
                                println!("{}", error);
                            }
                            _interpreter.clean_env();
                        }
                        Err(error) => {
                            println!("{:?}", error);
                        }
                    }
                }
            }
            _ => {
                println!("Exiting");
                break;
            }
        }
    }
}
