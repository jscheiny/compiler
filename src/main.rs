use crate::{
    lexer::SourceCode,
    parser::{Traverse, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let source = SourceCode::from("struct T(pub f: G[Y, int]) { pub f() -> x; }");

    // for token in source.tokens.iter() {
    //     println!("{}", token)
    // }

    let mut tokens = source.token_stream();
    let program = program(&mut tokens).unwrap();
    // println!("{:#?}", program);
    program.traverse(&|name, span| {
        println!("{} @ {:?}", name, span);
        println!("Source:[{}]\n", source.get_span_string(span));
    });
}
