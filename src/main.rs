use std::rc::Rc;

use crate::{lexer::tokenize, parser::parse};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize(
        "tuple S(x: string) {}
        struct G(x: string, y:List[T]) {}
        tuple F() {}",
    );
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
    let x = parse(Rc::new(tokens)).unwrap();
    println!("{:#?}", x);
}
