use std::rc::Rc;

use crate::{lexer::tokenize, parser::parse};

pub mod lexer;
pub mod parser;

fn main() {
    let tokens = tokenize(
        "tuple S(x: string) {}
        struct G(x: string, y:List[T]) {}
        tuple F() {}
        interface I {
            a(): int;
            b(x: int): Map[T];
        }
        fn f() {
            let x = ;
        }
        fn g(x: int): int {}
        fn h() -> ;
        fn i(x: int): int ->;",
    );
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
    let x = parse(Rc::new(tokens)).unwrap();
    println!("{:#?}", x);
}
