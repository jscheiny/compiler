use strum::IntoEnumIterator;

use crate::lexer::{Symbol, Token, Tokenizer, TryTokenizeResult, try_tokenize_enum};

pub struct SymbolTokenizer;

impl Tokenizer for SymbolTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        try_tokenize_enum(text, Symbol::iter(), Token::Symbol)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenWidth;

    use super::*;

    fn tokenize_symbol(text: &str) -> Option<TryTokenizeResult> {
        let tokenizer = SymbolTokenizer {};
        tokenizer.try_tokenize(text)
    }

    #[test]
    fn test_not_symbol() {
        assert_eq!(tokenize_symbol("x"), None);
        assert_eq!(tokenize_symbol("$"), None);
    }

    #[test]
    fn test_symbol() {
        assert_eq!(
            tokenize_symbol("+"),
            Some(TryTokenizeResult {
                token: Some(Token::Symbol(Symbol::Plus)),
                width: TokenWidth {
                    bytes: 1,
                    characters: 1,
                    new_lines: 0,
                    columns_since_last_new_line: 1,
                    bytes_since_last_new_line: 1
                }
            })
        );
        assert_eq!(
            tokenize_symbol("+=:"),
            Some(TryTokenizeResult {
                token: Some(Token::Symbol(Symbol::PlusEqual)),
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
