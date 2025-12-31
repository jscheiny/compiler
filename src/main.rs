use std::rc::Rc;

use crate::{lexer::tokenize, parser::parse};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize(
        "struct S() {
            fn(x: String): String[A, B, C] {
                continue;
                break;
                return;
                -> ;
                let x = ;
                let y: String = ;
                mut z: Map[A, bool] = ;
                while {
                    break;
                    return;
                    {
                        let x = ; 
                    }
                }
            }
        }",
    );
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
    let x = parse(Rc::new(tokens)).unwrap();
    println!("{:#?}", x);
}
