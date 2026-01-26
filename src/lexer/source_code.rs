use colored::Colorize;
use std::{error::Error, fs::read_to_string, rc::Rc};

use crate::{
    lexer::{LocatedToken, tokenize},
    parser::{TokenSpan, TokenStream},
};

pub struct SourceCode {
    pub path: String,
    pub tokens: Rc<Vec<LocatedToken>>,
    pub source: String,
}

impl SourceCode {
    pub fn read(path: &str) -> Result<Self, Box<dyn Error>> {
        let source = read_to_string(path)?;
        let tokens = Rc::new(tokenize(&source));
        let path = path.to_owned();
        Ok(SourceCode {
            path,
            tokens,
            source,
        })
    }

    pub fn token_stream(&self) -> TokenStream {
        TokenStream::from(self.tokens.clone())
    }

    pub fn print_span(&self, span: TokenSpan) {
        let start_character = &self.tokens[span.start_index].span.start;
        let start_byte = start_character.byte;

        let end_character = &self.tokens[span.end_index].span.end;
        let end_byte = end_character.byte;

        let prefix = self.span_prefix(start_byte);
        let body = &self.source[start_byte..end_byte];
        let suffix = self.span_suffix(end_byte);

        println!(
            "  {} {}:{} -> {}:{}",
            "-->".cyan().bold(),
            self.path,
            start_character,
            self.path,
            end_character,
        );

        let mut line = start_character.line + 1;
        let mut first_line = true;
        for line_text in body.split('\n') {
            if !first_line {
                println!();
            }
            print_line_header(line);
            if first_line {
                print!("{}", prefix);
                first_line = false;
            }
            print!("{}", line_text.to_string().bold().yellow());
            line += 1;
        }
        println!("{}", suffix);
    }

    fn span_prefix(&self, start_byte: usize) -> &str {
        let prefix = &self.source[..start_byte];
        let start_line_byte = prefix.rfind('\n').map(|start| start + 1).unwrap_or(0);
        &prefix[start_line_byte..]
    }

    fn span_suffix(&self, end_byte: usize) -> &str {
        if end_byte == self.source.len() {
            ""
        } else {
            let suffix = &self.source[end_byte..];
            let end_line_byte = suffix.find('\n').unwrap_or(suffix.len() - 1);
            &suffix[..end_line_byte]
        }
    }
}

fn print_line_header(line: usize) {
    print!(
        "{:>2} {} ",
        line.to_string().bold().cyan(),
        "|".bold().cyan(),
    );
}
