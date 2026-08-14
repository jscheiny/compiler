use strum::IntoEnumIterator;

use crate::lexer::{Keyword, Token, Tokenizer, TryTokenizeResult, try_tokenize_enum};

pub struct KeywordTokenizer;

impl Tokenizer for KeywordTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        try_tokenize_enum(text, Keyword::iter(), Token::Keyword)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenWidth;

    use super::*;

    fn tokenize_keyword(text: &str) -> Option<TryTokenizeResult> {
        let tokenizer = KeywordTokenizer {};
        tokenizer.try_tokenize(text)
    }

    #[test]
    fn test_not_keyword() {
        assert_eq!(tokenize_keyword("neat"), None);
        assert_eq!(tokenize_keyword("+="), None);
    }

    #[test]
    fn test_keyword() {
        assert_eq!(
            tokenize_keyword("fnx"),
            Some(TryTokenizeResult {
                token: Some(Token::Keyword(Keyword::Fn)),
                width: TokenWidth {
                    bytes: 2,
                    characters: 2,
                    new_lines: 0,
                    columns_since_last_new_line: 2,
                    bytes_since_last_new_line: 2
                }
            })
        );
    }
}
