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

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize_single_line_comment(text: &str) -> Option<TryTokenizeResult> {
        let tokenizer = SingleLineCommentTokenizer {};
        tokenizer.try_tokenize(text)
    }

    #[test]
    fn test_no_single_line_comment() {
        assert_eq!(tokenize_single_line_comment("/ no comment"), None);
        assert_eq!(tokenize_single_line_comment("/* no comment */"), None);
    }

    #[test]
    fn test_single_line_comment() {
        let source = "// this is a comment";
        assert_eq!(
            tokenize_single_line_comment(source),
            Some(TryTokenizeResult {
                token: None,
                width: TokenWidth {
                    bytes: source.len(),
                    characters: source.len(),
                    new_lines: 0,
                    columns_since_last_new_line: source.len(),
                    bytes_since_last_new_line: source.len(),
                }
            })
        );
    }

    #[test]
    fn test_single_line_comment_newline() {
        assert_eq!(
            tokenize_single_line_comment("// this is a comment\nthis isn't"),
            Some(TryTokenizeResult {
                token: None,
                width: TokenWidth {
                    bytes: 21,
                    characters: 21,
                    new_lines: 1,
                    columns_since_last_new_line: 0,
                    bytes_since_last_new_line: 0,
                }
            })
        );
    }

    fn tokenize_multi_line_comment(text: &str) -> Option<TryTokenizeResult> {
        let tokenizer = MultiLineCommentTokenizer {};
        tokenizer.try_tokenize(text)
    }

    #[test]
    fn test_no_multiline_comment() {
        assert_eq!(tokenize_multi_line_comment("* no comment"), None);
        assert_eq!(tokenize_multi_line_comment("// no comment"), None);
        assert_eq!(tokenize_multi_line_comment("/ no comment"), None);
    }

    #[test]
    fn test_multiline_comment_simple() {
        let source = "/* comment */";
        assert_eq!(
            tokenize_multi_line_comment(source),
            Some(TryTokenizeResult {
                token: None,
                width: TokenWidth {
                    bytes: source.len(),
                    characters: source.len(),
                    new_lines: 0,
                    columns_since_last_new_line: source.len(),
                    bytes_since_last_new_line: source.len(),
                }
            })
        );
    }
}
