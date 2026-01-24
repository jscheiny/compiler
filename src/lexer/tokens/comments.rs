use crate::lexer::{IgnoredToken, Token, TokenWidth, TryTokenizeResult};

const COMMENT_START: &str = "//";

pub fn try_tokenize_single_line_comment(text: &str) -> Option<TryTokenizeResult> {
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

    Some(TryTokenizeResult {
        token: Token::Ignored(IgnoredToken::new()),
        width,
    })
}

const MULTILINE_COMMENT_START: &str = "/*";
const MULTILINE_COMMENT_END: &str = "*/";

pub fn try_tokenize_multiline_comment(text: &str) -> Option<TryTokenizeResult> {
    if !text.starts_with(MULTILINE_COMMENT_START) {
        return None;
    }

    let mut width = TokenWidth::new();
    let mut token = IgnoredToken::new();
    for character in text.chars() {
        if text[width.bytes..].starts_with(MULTILINE_COMMENT_END) {
            let adjustment = MULTILINE_COMMENT_END.len();
            width.add_str(MULTILINE_COMMENT_END);
            token.columns_since_last_new_line += adjustment;
            break;
        }

        width.add_char(character);
        token.add(character);
    }

    Some(TryTokenizeResult {
        token: Token::Ignored(token),
        width,
    })
}
