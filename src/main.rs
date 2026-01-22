use std::rc::Rc;

use crate::{lexer::tokenize, parser::parse};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize("struct T(){ f(x:int)->x;}");

    for token in tokens.iter() {
        println!("{}", token);
    }
    let x = parse(Rc::new(tokens)).unwrap();
    println!("{:#?}", x);
}
