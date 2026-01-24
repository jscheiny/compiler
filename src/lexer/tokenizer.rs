use crate::lexer::{
    CharacterLocation, CharacterSpan, LocatedToken, Token, TokenWidth, try_tokenize_identifier,
    try_tokenize_integer_literal, try_tokenize_keyword, try_tokenize_multiline_comment,
    try_tokenize_operator, try_tokenize_single_line_comment, try_tokenize_string_literal,
    try_tokenize_whitespace,
};

pub fn tokenize(mut text: &str) -> Vec<LocatedToken> {
    let mut tokens = vec![];
    let mut start: CharacterLocation = CharacterLocation {
        line: 0,
        column: 0,
        byte: 0,
    };
    while let Some(NextToken { token, width, next }) = next_token(text) {
        let end: CharacterLocation = match token {
            Token::Ignored(token) => {
                if token.new_lines == 0 {
                    start.add_columns(width)
                } else {
                    start.add_lines(token, width.bytes)
                }
            }
            token => {
                let end = CharacterLocation {
                    line: start.line,
                    column: start.column + width.characters,
                    byte: start.byte + width.bytes,
                };
                let span = CharacterSpan { start, end };
                tokens.push(LocatedToken { token, span });
                end
            }
        };
        start = end;
        text = next;
    }
    if !text.is_empty() {
        println!("{}", text);
        panic!("Tokenizer error");
    }
    tokens
}

pub struct TryTokenizeResult {
    pub token: Token,
    pub width: TokenWidth,
}

struct NextToken<'a> {
    pub token: Token,
    pub width: TokenWidth,
    pub next: &'a str,
}

fn next_token(text: &str) -> Option<NextToken<'_>> {
    try_tokenize_single_line_comment(text)
        .or_else(|| try_tokenize_multiline_comment(text))
        .or_else(|| try_tokenize_operator(text))
        .or_else(|| try_tokenize_keyword(text))
        .or_else(|| try_tokenize_string_literal(text))
        .or_else(|| try_tokenize_integer_literal(text))
        .or_else(|| try_tokenize_identifier(text))
        .or_else(|| try_tokenize_whitespace(text))
        .map(|result| {
            let (_, next) = text.split_at(result.width.bytes);
            NextToken {
                token: result.token,
                width: result.width,
                next,
            }
        })
}
