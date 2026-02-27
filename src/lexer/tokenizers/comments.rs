use crate::lexer::{TokenWidth, Tokenizer, TryTokenizeResult};

const COMMENT_START: &str = "//";

pub struct SingleLineCommentTokenizer;

impl Tokenizer for SingleLineCommentTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        if !text.starts_with(COMMENT_START) {
            return None;
        }

        let mut width = TokenWidth::new();
        for character in text.chars() {
            width.add_char(character);
            if character == '\n' {
                break;
            }
        }

        Some(TryTokenizeResult { token: None, width })
    }
}

const MULTILINE_COMMENT_START: &str = "/*";
const MULTILINE_COMMENT_END: &str = "*/";

pub struct MultiLineCommentTokenizer;

impl Tokenizer for MultiLineCommentTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        if !text.starts_with(MULTILINE_COMMENT_START) {
            return None;
        }

        let mut width = TokenWidth::new();
        for character in text.chars() {
            if text[width.bytes..].starts_with(MULTILINE_COMMENT_END) {
                width.add_str(MULTILINE_COMMENT_END);
                break;
            }

            width.add_char(character);
        }

        Some(TryTokenizeResult { token: None, width })
    }
}
