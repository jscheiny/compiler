#[derive(Debug, Default)]
pub struct TokenWidth {
    pub bytes: usize,
    pub characters: usize,
    pub new_lines: usize,
    pub columns_since_last_new_line: usize,
    pub bytes_since_last_new_line: usize,
}

impl TokenWidth {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(text: &str) -> Self {
        let mut result = Self::new();
        result.add_str(text);
        result
    }

    pub fn add_char(&mut self, character: char) {
        self.bytes += character.len_utf8();
        self.characters += 1;
        if character == '\n' {
            self.new_lines += 1;
            self.columns_since_last_new_line = 0;
            self.bytes_since_last_new_line = 0;
        } else {
            self.columns_since_last_new_line += 1;
            self.bytes_since_last_new_line += character.len_utf8();
        }
    }

    pub fn add_str(&mut self, text: &str) {
        for character in text.chars() {
            self.add_char(character);
        }
    }
}
