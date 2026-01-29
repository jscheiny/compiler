use std::env;

use colored::Colorize;

use crate::{
    lexer::{Severity, SourceCode},
    parser::program,
};

pub mod lexer;
pub mod parser;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let source = SourceCode::read(args[1].as_str()).unwrap();

    let mut tokens = source.token_stream();
    let result = program(&mut tokens);
    if let Err(syntax_err) = result {
        tokens.errors.push(syntax_err);
    }
    println!(
        "Found {} {}\n",
        tokens.errors.len().to_string().cyan().red(),
        if tokens.errors.len() == 1 {
            "error"
        } else {
            "errors"
        },
    );
    for error in tokens.errors {
        print!("{} ", "Error:".red().bold());
        error.print(source.tokens.as_ref());
        println!();
        source.print_span(
            error.span,
            '^',
            error.kind.to_string().as_str(),
            Severity::Error,
        );
        println!();
    }
}
