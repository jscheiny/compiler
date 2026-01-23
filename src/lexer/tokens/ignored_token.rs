#[derive(Clone)]
pub struct IgnoredToken {
    pub new_lines: usize,
    pub columns_since_last_new_line: usize,
    pub bytes_since_last_new_line: usize,
}

impl IgnoredToken {
    pub fn new() -> Self {
        Self {
            new_lines: 0,
            columns_since_last_new_line: 0,
            bytes_since_last_new_line: 0,
        }
    }

    pub fn add(&mut self, character: char) {
        if character == '\n' {
            self.new_lines += 1;
            self.columns_since_last_new_line = 0;
            self.bytes_since_last_new_line = 0;
        } else {
            self.columns_since_last_new_line += 1;
            self.bytes_since_last_new_line += character.len_utf8();
        }
    }
}
