use std::{env, process::exit};

use colored::Colorize;

use crate::{
    lexer::{Severity, SourceCode},
    parser::{LocatedSyntaxError, program},
};

pub mod checker;
pub mod lexer;
pub mod parser;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        println!("{} {} <source-file>", "Usage error:".bold().red(), args[0]);
        exit(1);
    }

    let source = SourceCode::read(args[1].as_str()).unwrap();
    let mut tokens = source.token_stream();
    let result = program(&mut tokens);

    for error in source.tokenizer_errors.iter() {
        println!("{} unexpected token", "Lexer error:".red().bold());
        source.print_character_span(*error, '^', "unexpected token", Severity::Error);
        println!();
    }

    for error in tokens.errors.iter() {
        print_err(&source, error);
    }

    match result {
        Ok(program) => {
            program.check();
        }
        Err(error) => {
            print_err(&source, &error);
        }
    }
}

fn print_err(source: &SourceCode, error: &LocatedSyntaxError) {
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
