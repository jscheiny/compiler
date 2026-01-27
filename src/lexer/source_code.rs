use colored::{ColoredString, Colorize};
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

#[derive(Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
    Note,
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

    pub fn print_span(&self, span: TokenSpan, underline: char, message: &str, severity: Severity) {
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

        let lines = body.split('\n').collect::<Vec<_>>();
        for (line_offset, &line_text) in lines.iter().enumerate() {
            let line = start_character.line + line_offset + 1;
            print_line_header(line);
            if line_offset == 0 {
                print!("{}", prefix);
            }

            print!("{}", line_text);
            if line_offset == lines.len() - 1 {
                println!("{}", suffix);
            } else {
                println!();
            }

            print!("   {} ", "|".bold().cyan());
            if line_offset == 0 {
                for _ in prefix.chars() {
                    print!(" ")
                }
            }

            let underline = underline.to_string();
            let mut is_initial_whitespace = true;
            for character in line_text.chars() {
                if !character.is_whitespace() {
                    is_initial_whitespace = false;
                }
                if is_initial_whitespace {
                    print!(" ");
                } else {
                    print!("{}", apply_severity(underline.as_str(), severity));
                }
            }

            if line_offset == lines.len() - 1 {
                print!(" {}", apply_severity(message, severity))
            }
            println!();
        }
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

fn apply_severity(text: &str, severity: Severity) -> ColoredString {
    match severity {
        Severity::Error => text.red().bold(),
        Severity::Warning => text.yellow().bold(),
        Severity::Note => text.green().bold(),
    }
}

fn print_line_header(line: usize) {
    print!(
        "{:>2} {} ",
        line.to_string().bold().cyan(),
        "|".bold().cyan(),
    );
}
