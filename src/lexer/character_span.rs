use std::fmt::Display;

use crate::lexer::WhitespaceToken;

#[derive(Clone, Copy)]
pub struct CharacterSpan {
    pub start: CharacterLocation,
    pub end: CharacterLocation,
}

impl Display for CharacterSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

#[derive(Clone, Copy)]
pub struct CharacterLocation {
    pub line: usize,
    pub column: usize,
    pub byte: usize,
}

impl CharacterLocation {
    pub fn add_columns(&self, columns: usize) -> Self {
        CharacterLocation {
            line: self.line,
            column: self.column + columns,
            byte: self.byte + columns,
        }
    }

    pub fn add_lines(&self, whitespace: WhitespaceToken, bytes: usize) -> Self {
        CharacterLocation {
            line: self.line + whitespace.new_lines,
            column: whitespace.columns_since_last_new_line,
            byte: self.byte + bytes,
        }
    }
}

impl Display for CharacterLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
