use colored::{ColoredString, Colorize};
use std::{cmp::min, error::Error, fs::read_to_string, rc::Rc};

use crate::{
    lexer::{CharacterSpan, LocatedToken, TokenizerResult, tokenize},
    parser::{TokenSpan, TokenStream},
};

pub struct SourceCode {
    pub path: String,
    pub tokens: Rc<Vec<LocatedToken>>,
    pub tokenizer_errors: Vec<CharacterSpan>,
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
        let TokenizerResult { tokens, errors } = tokenize(&source);
        let path = path.to_owned();
        Ok(SourceCode {
            path,
            tokens: Rc::new(tokens),
            tokenizer_errors: errors,
            source,
        })
    }

    pub fn token_stream(&self) -> TokenStream {
        TokenStream::from(self.tokens.clone())
    }

    pub fn print_type_error(&self, span: TokenSpan, message: &str, inline_message: &str) {
        println!("{} {}", "Error:".red().bold(), message,);
        self.print_token_span(span, '^', inline_message, Severity::Error);
        println!();
    }

    pub fn print_token_span(
        &self,
        span: TokenSpan,
        underline: char,
        message: &str,
        severity: Severity,
    ) {
        let start = self.tokens[span.start_index].span.start;
        let end = self.tokens[span.end_index].span.end;
        let character_span = CharacterSpan { start, end };
        self.print_character_span(character_span, underline, message, severity);
    }

    pub fn print_character_span(
        &self,
        span: CharacterSpan,
        underline: char,
        message: &str,
        severity: Severity,
    ) {
        let CharacterSpan {
            start: start_character,
            end: end_character,
        } = span;
        let start_byte = start_character.byte;
        let end_byte = end_character.byte;
        let end_byte_clamped = min(end_byte, self.source.len());

        let prefix = self.span_prefix(start_byte);
        let body = &self.source[start_byte..min(end_byte, end_byte_clamped)];
        let suffix = self.span_suffix(end_byte);

        println!(
            "  {} {}:{}",
            "-->".cyan().bold(),
            self.path,
            start_character,
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
            let span_bytes = end_byte - start_byte;
            for _ in 0..span_bytes {
                print!("{}", apply_severity(underline.as_str(), severity));
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
        if end_byte >= self.source.len() {
            ""
        } else {
            let suffix = &self.source[end_byte..];
            let end_line_byte = suffix.find('\n').unwrap_or(suffix.len());
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
