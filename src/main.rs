use std::rc::Rc;

use crate::{lexer::tokenize, parser::parse};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize(
        "fn test() {
            let x = ;
            mut y = ;
            return;
            ->;
            break;
            continue;
            while {}
            {}
        }",
    );
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
    let x = parse(Rc::new(tokens)).unwrap();
    println!("{:#?}", x);
}
