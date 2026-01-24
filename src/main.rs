use crate::{
    lexer::SourceCode,
    parser::{Traverse, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let source = SourceCode::from(
        "fn test() {
        if x { let y = \"This is a test\"; }
        else if y { let z = x; }
        else {}
    }",
    );

    // for token in source.tokens.iter() {
    //     println!("{}", token)
    // }

    let mut tokens = source.token_stream();
    let program = program(&mut tokens).unwrap();
    program.traverse(&|name, span| {
        println!("{} @ {:?}", name, span);
        println!("[{}]\n", source.get_span_string(span));
    });
}
