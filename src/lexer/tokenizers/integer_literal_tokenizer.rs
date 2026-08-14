use crate::lexer::{Token, TokenWidth, Tokenizer, TryTokenizeResult};

pub struct IntegerLiteralTokenizer;

impl Tokenizer for IntegerLiteralTokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult> {
        let mut width = TokenWidth::new();
        for character in text.chars() {
            if width.bytes == 0 && !character.is_numeric() {
                return None;
            }

            if !character.is_numeric() {
                break;
            }

            width.add_char(character);
        }

        let maybe_value = &text[0..width.bytes].parse::<i64>().ok();
        maybe_value.as_ref().map(|value| TryTokenizeResult {
            token: Some(Token::IntegerLiteral(*value)),
            width,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenWidth;

    use super::*;

    fn tokenize_integer(text: &str) -> Option<TryTokenizeResult> {
        let tokenizer = IntegerLiteralTokenizer {};
        tokenizer.try_tokenize(text)
    }

    #[test]
    fn test_not_integer() {
        assert_eq!(tokenize_integer("a1"), None);
    }

    #[test]
    fn test_integer() {
        assert_eq!(
            tokenize_integer("123"),
            Some(TryTokenizeResult {
                token: Some(Token::IntegerLiteral(123)),
                width: TokenWidth {
                    bytes: 3,
                    characters: 3,
                    new_lines: 0,
                    columns_since_last_new_line: 3,
                    bytes_since_last_new_line: 3
                }
            })
        );
    }

    #[test]
    fn test_integer_end() {
        assert_eq!(
            tokenize_integer("1.2"),
            Some(TryTokenizeResult {
                token: Some(Token::IntegerLiteral(1)),
                width: TokenWidth {
                    bytes: 1,
                    characters: 1,
                    new_lines: 0,
                    columns_since_last_new_line: 1,
                    bytes_since_last_new_line: 1
                }
            })
        );
    }
}
