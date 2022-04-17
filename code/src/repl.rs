use crate::tokenizer::Tokenizer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::parser::Parser;
use log;
use crate::interpreter::{
    Interpreter
};


pub fn repl() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("DEUS-USER> ");
        let mut tokenizer = Tokenizer::create();
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let _result = tokenizer.tokenize(line);
                // tokenizer.print_tokens();
                let mut _parser = Parser::create(tokenizer.tokens);
                let _parseresult = _parser.parse();
                // print!("{:?}", _parseresult);
                let mut _interpreter = Interpreter {};
                _interpreter.accept(_parseresult.unwrap()[0].clone());
            },
            Err(ReadlineError::Interrupted) => {
                log::info!("Bye!!");
                break
            },
            Err(ReadlineError::Eof) => {
                log::info!("CTRL-D");
                break
            },
            Err(err) => {
                log::warn!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
