mod repl;
mod tokenizer;
mod types;
mod parser;
mod token;
mod expression;
mod visitors;
mod interpreter;
fn main() {
    repl::repl();
}
