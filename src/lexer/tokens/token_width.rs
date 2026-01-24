#[derive(Default)]
pub struct TokenWidth {
    pub bytes: usize,
    pub characters: usize,
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
    }

    pub fn add_str(&mut self, text: &str) {
        self.bytes += text.len();
        self.characters += text.chars().count()
    }
}
