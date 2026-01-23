use std::rc::Rc;

use crate::{lexer::tokenize, parser::parse};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize(
        "struct S(pub x: Y) {}
        
        fn f() {
            if x {}
            else if y {}
            else if z {}
            else {}
        }",
    );

    for token in tokens.iter() {
        println!("{}", token);
    }
    let x = parse(Rc::new(tokens)).unwrap();
    println!("{:#?}", x);
}
