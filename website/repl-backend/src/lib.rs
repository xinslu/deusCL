#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
mod environment;
mod expression;
mod functions;
pub mod interpreter;
pub mod parser;
pub mod repl;
mod token;
pub mod tokenizer;
mod types;
mod visitors;
