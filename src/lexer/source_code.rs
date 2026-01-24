use colored::Colorize;
use std::rc::Rc;

use crate::{
    lexer::{LocatedToken, tokenize},
    parser::{TokenSpan, TokenStream},
};

pub struct SourceCode {
    pub tokens: Rc<Vec<LocatedToken>>,
    pub source: String,
}

impl SourceCode {
    pub fn from(text: &str) -> Self {
        let tokens = Rc::new(tokenize(text));
        let source = text.to_owned();
        Self { tokens, source }
    }

    pub fn token_stream(&self) -> TokenStream {
        TokenStream::from(self.tokens.clone())
    }

    pub fn get_span_string(&self, span: TokenSpan) -> String {
        let start_byte = self.tokens[span.start_index].span.start.byte;
        let end_byte = self.tokens[span.end_index].span.end.byte;
        let source_slice = self.source[start_byte..end_byte].to_owned();
        source_slice
    }

    pub fn print_span(&self, span: TokenSpan) {
        let start_character = &self.tokens[span.start_index].span.start;
        let start_byte = start_character.byte;

        let end_character = &self.tokens[span.end_index].span.end;
        let end_byte = end_character.byte;

        let prefix = &self.source[..start_byte];
        let start_line_byte = prefix.rfind('\n').map(|start| start + 1).unwrap_or(0);
        let prefix = &prefix[start_line_byte..];

        let body = &self.source[start_byte..end_byte];
        let suffix = if end_byte == self.source.len() {
            ""
        } else {
            let suffix = &self.source[end_byte..];
            let end_line_byte = suffix.find('\n').unwrap_or(suffix.len() - 1);
            &suffix[..end_line_byte]
        };

        println!(
            "  {} source/file:{}:{} -> source/file:{}:{}",
            "-->".magenta().bold(),
            start_character.line,
            start_character.column,
            end_character.line,
            end_character.column,
        );
        let mut line = start_character.line + 1;
        print!(
            "{:>2} {} {}",
            line.to_string().bold().purple(),
            "|".bold().purple(),
            prefix,
        );
        for character in body.chars() {
            if character == '\n' {
                line += 1;
                print!(
                    "\n{:>2} {} ",
                    line.to_string().bold().purple(),
                    "|".bold().purple(),
                )
            } else {
                print!("{}", character.to_string().bold().cyan());
            }
        }
        println!("{}", suffix);
    }
}
