use std::fmt::Display;

use crate::lexer::TokenWidth;

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
    pub fn add(&self, width: TokenWidth) -> Self {
        if width.new_lines == 0 {
            CharacterLocation {
                line: self.line,
                column: self.column + width.characters,
                byte: self.byte + width.bytes,
            }
        } else {
            CharacterLocation {
                line: self.line + width.new_lines,
                column: width.columns_since_last_new_line,
                byte: self.byte + width.bytes,
            }
        }
    }

    pub fn add_byte(&self) -> Self {
        CharacterLocation {
            line: self.line,
            column: self.column + 1,
            byte: self.byte + 1,
        }
    }
}

impl Display for CharacterLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
