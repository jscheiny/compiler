use crate::lexer::{Token, TokenWidth, TryTokenizeResult};

pub trait EnumToken {
    fn as_str(&self) -> &str;
}

pub fn try_tokenize_enum<T: EnumToken + Copy>(
    text: &str,
    token_iterator: impl Iterator<Item = T>,
    make_token: impl FnOnce(T) -> Token,
) -> Option<TryTokenizeResult> {
    let mut found_token = None;
    let mut found_length = 0;
    for token in token_iterator {
        let token_str = token.as_str();
        if text.starts_with(token_str) && token_str.len() > found_length {
            found_token = Some(token);
            found_length = token_str.chars().count();
        }
    }

    found_token.map(|token| TryTokenizeResult {
        token: Some(make_token(token)),
        width: TokenWidth::from(token.as_str()),
    })
}
