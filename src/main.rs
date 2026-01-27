use std::env;

use crate::{
    lexer::{Severity, SourceCode},
    parser::{Traverse, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let source = SourceCode::read(&args[1].as_str()).unwrap();

    let mut tokens = source.token_stream();
    let program = program(&mut tokens).unwrap();
    program.traverse(&|name, span| {
        if source.is_single_line(span) {
            source.print_span(span, '^', name, Severity::Error);
            println!();
        }
    });
}
