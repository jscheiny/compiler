use crate::{
    lexer::SourceCode,
    parser::{Traverse, program},
};

pub mod lexer;
pub mod parser;

fn main() {
    let source = SourceCode::from(
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

    let mut tokens = source.token_stream();
    let program = program(&mut tokens).unwrap();
    println!("{:#?}", program);
    program.traverse(&|span| {
        println!("{:?}", span);
        println!("Source:[{}]", source.get_span_string(span));
    });
}
