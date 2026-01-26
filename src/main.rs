use std::env;

use crate::{
    lexer::SourceCode,
    parser::{Traverse, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let source = SourceCode::read(&args[1].as_str()).unwrap();

    // for token in source.tokens.iter() {
    //     println!("{}", token)
    // }

    let mut tokens = source.token_stream();
    let program = program(&mut tokens).unwrap();
    program.traverse(&|name, span| {
        println!("{} @ {:?}", name, span);
        source.print_span(span, name);
        println!();
    });
}
