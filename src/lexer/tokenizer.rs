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
    while let Some(token) = next_token(text) {
        println!("{:?}", token.width);
        let NextToken { token, width, next } = token;
        let end = start.add(width);
        if let Some(token) = token {
            let span = CharacterSpan { start, end };
            tokens.push(LocatedToken { token, span });
        }
        start = end;
        text = next;
    }
    if !text.is_empty() {
        println!("{}", text);
        panic!("Tokenizer error");
    }
    tokens.push(LocatedToken {
        token: Token::EndOfFile,
        span: CharacterSpan {
            start,
            end: start.add_byte(),
        },
    });
    tokens
}

pub struct TryTokenizeResult {
    pub token: Option<Token>,
    pub width: TokenWidth,
}

struct NextToken<'a> {
    pub token: Option<Token>,
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
