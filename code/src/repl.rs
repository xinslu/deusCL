use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;
use log;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::panic;

pub fn repl() {
    let mut rl = Editor::<()>::new();
    let mut _interpreter = Interpreter::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("DEUS-USER> ");
        let mut tokenizer = Tokenizer::new();
        match readline {
            Ok(line) => {
                let processed_line = str::replace(line.as_str(), "\n", " ");
                rl.add_history_entry(processed_line.clone());
                if let Err(error) = tokenizer.tokenize(processed_line.to_string()) {
                    println!("{}", error);
                } else {
                // println!("created parser");
                // tokenizer.print_tokens();
                // println!("right after print");
                match Parser::new(tokenizer.tokens).parse() {
                    Ok(parserresult) => {
                        println!("{:?}", parserresult);
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
            Err(ReadlineError::Interrupted) => {
                log::info!("Bye!!");
                break;
            }
            Err(ReadlineError::Eof) => {
                log::info!("CTRL-D");
                break;
            }
            Err(err) => {
                log::warn!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
