use std::rc::Rc;

use crate::{
    lexer::tokenize,
    parser::{TokenTraverser, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize(
        "pub struct S(pub x: Y) {
            pub test() {
                while x {}
            }

            test() -> x;
        }
        
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

    let mut token_traverser = TokenTraverser::new(Rc::new(tokens));
    let program = program(&mut token_traverser).unwrap();
    println!("{:#?}", program);
}
