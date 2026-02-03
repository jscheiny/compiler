use std::{env, process::exit};

use colored::Colorize;

use crate::{
    lexer::{Severity, SourceCode},
    parser::program,
};

pub mod lexer;
pub mod parser;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        println!("Syntax: {} <source-files>", args[0]);
        exit(1);
    }

    let source = SourceCode::read(args[1].as_str()).unwrap();
    let mut tokens = source.token_stream();
    let result = program(&mut tokens);
    if let Err(syntax_err) = result {
        tokens.errors.push(syntax_err);
    }

    let error_count = tokens.errors.len() + source.tokenizer_errors.len();
    println!(
        "Found {} {}\n",
        error_count.to_string().cyan().red(),
        if tokens.errors.len() == 1 {
            "error"
        } else {
            "errors"
        },
    );

    for error in source.tokenizer_errors.iter() {
        println!("{} unexpected token", "Lexer error:".red().bold());
        source.print_character_span(*error, '^', "unexpected token", Severity::Error);
        println!();
    }

    for error in tokens.errors {
        print!(
            "{} {}",
            "Syntax error:".red().bold(),
            error.message(source.tokens.clone())
        );
        println!();
        source.print_token_span(
            error.span,
            '^',
            error.inline_message().to_string().as_str(),
            Severity::Error,
        );
        println!();
    }
}
