use std::fmt::Debug;

use crate::lexer::WhitespaceToken;

#[derive(Clone, Copy)]
pub struct CharacterLocation {
    pub line: usize,
    pub column: usize,
}

impl CharacterLocation {
    pub fn add_columns(&self, columns: usize) -> Self {
        CharacterLocation {
            line: self.line,
            column: self.column + columns,
        }
    }

    pub fn add_lines(&self, whitespace: WhitespaceToken) -> Self {
        CharacterLocation {
            line: self.line + whitespace.new_lines,
            column: whitespace.columns_since_last_new_line,
        }
    }
}

impl Debug for CharacterLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
