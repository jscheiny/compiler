use crate::{
    lexer::SourceCode,
    parser::{Traverse, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let source = SourceCode::from(
        "struct Test() { // with a comment!
        test() {/*
            multiline!
        */}
    }",
    );

    for token in source.tokens.iter() {
        println!("{}", token)
    }

    let mut tokens = source.token_stream();
    let program = program(&mut tokens).unwrap();
    println!("{:#?}", program);
    program.traverse(&|span| {
        println!("{:?}", span);
        println!("Source:[{}]", source.get_span_string(span));
    });
}
