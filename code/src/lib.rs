#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
pub mod repl;
mod tokenizer;
mod types;
mod parser;
mod token;
mod expression;
mod visitors;
mod interpreter;
mod environment;
mod functions;
pub fn main() {
    repl::repl();
}
