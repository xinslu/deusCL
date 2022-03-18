use crate::tokenizer::Tokenizer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
// use crate::parser::parser;
use log;
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
                tokenizer.print_tokens();
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
