use crate::tokenizer::tokenizer::tokenize;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use log;
pub fn repl() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("DEUS-USER> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("OUT: {:?}", tokenize(line));
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
