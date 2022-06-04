#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#[feature(new_uninit)]
mod repl;
mod tokenizer;
mod types;
mod parser;
mod token;
mod expression;
mod visitors;
mod interpreter;
mod environment;
fn main() {
    repl::repl();
}
