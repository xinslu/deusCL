use code::interpreter::Interpreter;
use code::parser::Parser;
use code::tokenizer::Tokenizer;
use log;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn main() {
    let mut rl = Editor::<()>::new();
    let mut _interpreter = Interpreter::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let mut counter = 0;
    let mut lines = String::new();
    let mut prompt = "DEUS-USER> ";
    loop {
        let readline = rl.readline(prompt);
        let mut tokenizer = Tokenizer::new();
        match readline {
            Ok(line) => {
                for i in line.chars() {
                    if i == '(' {
                        counter += 1;
                    } else if i == ')' {
                        counter -= 1;
                    }
                }
                lines.push_str(&str::replace(line.as_str(), "\n", " "));
                if counter == 0 {
                    let processed_line = str::replace(lines.as_str(), "\n", " ");
                    rl.add_history_entry(processed_line.clone());
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
                    lines = String::new();
                    counter = 0;
                    prompt = "DEUS-USER> ";
                } else if counter < 0 {
                    lines = String::new();
                    counter = 0;
                    prompt = "DEUS-USER> ";
                    println!("ERROR: Mismatched Closing Paranthesis")
                } else {
                    prompt = ".........> "
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
