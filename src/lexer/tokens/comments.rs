use crate::lexer::{IgnoredToken, Token};

const COMMENT_START: &str = "//";

pub fn try_tokenize_single_line_comment(text: &str) -> Option<(Token, usize)> {
    if !text.starts_with(COMMENT_START) {
        return None;
    }

    let mut bytes = 0;
    for character in text.chars() {
        bytes += character.len_utf8();
        if character == '\n' {
            break;
        }
    }

    Some((Token::Ignored(IgnoredToken::new()), bytes))
}

const MULTILINE_COMMENT_START: &str = "/*";
const MULTILINE_COMMENT_END: &str = "*/";

pub fn try_tokenize_multiline_comment(text: &str) -> Option<(Token, usize)> {
    if !text.starts_with(MULTILINE_COMMENT_START) {
        return None;
    }

    let mut bytes = 0;
    let mut token = IgnoredToken::new();
    for character in text.chars() {
        if text[bytes..].starts_with(MULTILINE_COMMENT_END) {
            bytes += MULTILINE_COMMENT_END.len();
            token.columns_since_last_new_line += MULTILINE_COMMENT_END.len();
            break;
        }

        bytes += character.len_utf8();
        token.add(character);
    }

    Some((Token::Ignored(token), bytes))
}
