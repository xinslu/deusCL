#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
mod environment;
mod expression;
mod functions;
mod interpreter;
mod parser;
pub mod repl;
mod token;
mod tokenizer;
mod types;
mod visitors;
use environment::Environment;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn main(s: String) -> String {
    repl::repl(s)
}
