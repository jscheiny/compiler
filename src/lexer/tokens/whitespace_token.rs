use crate::lexer::{Token, TokenParse};

#[derive(Debug, Clone)]
pub struct WhitespaceToken {
    pub new_lines: usize,
    pub columns_since_last_new_line: usize,
}

impl TokenParse for WhitespaceToken {
    fn try_tokenize(text: &str) -> Option<(Token, usize)> {
        let mut len = 0;
        let mut new_lines = 0;
        let mut columns_since_last_new_line = 0;
        for character in text.chars() {
            if character.is_whitespace() {
                len += character.len_utf8();
                if character == '\n' {
                    new_lines += 1;
                    columns_since_last_new_line = 0;
                } else {
                    columns_since_last_new_line += 1;
                }
            } else {
                break;
            }
        }

        if len != 0 {
            let token = WhitespaceToken {
                new_lines,
                columns_since_last_new_line,
            };
            Some((Token::Whitespace(token), len))
        } else {
            None
        }
    }
}
