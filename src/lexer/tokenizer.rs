use crate::lexer::{
    CharacterLocation, CharacterSpan, LocatedToken, Token, TokenWidth, try_tokenize_identifier,
    try_tokenize_integer_literal, try_tokenize_keyword, try_tokenize_multiline_comment,
    try_tokenize_single_line_comment, try_tokenize_string_literal, try_tokenize_symbol,
    try_tokenize_whitespace,
};

pub struct TokenizerResult {
    pub tokens: Vec<LocatedToken>,
    pub errors: Vec<CharacterSpan>,
}

pub fn tokenize(mut text: &str) -> TokenizerResult {
    let mut errors = vec![];
    let mut tokens = vec![];
    let mut start: CharacterLocation = CharacterLocation {
        line: 0,
        column: 0,
        byte: 0,
    };

    while !text.is_empty() {
        while let Some(token) = next_token(text) {
            let NextToken { token, width, next } = token;
            let end = start.add(width);
            if let Some(token) = token {
                let span = CharacterSpan { start, end };
                tokens.push(LocatedToken { token, span });
            }
            start = end;
            text = next;
        }

        let mut bad_token_start = 0;
        loop {
            let slice = &text[bad_token_start..];
            if slice.is_empty() {
                break;
            }
            let token = next_token(slice);
            if token.is_some() {
                break;
            }
            bad_token_start += 1;
        }

        let bad_token = &text[..bad_token_start];
        if !bad_token.is_empty() {
            let end = start.add(TokenWidth::from(bad_token));
            errors.push(CharacterSpan { start, end });
            start = end;
            text = &text[bad_token_start..];
        }
    }

    tokens.push(LocatedToken {
        token: Token::EndOfFile,
        span: CharacterSpan {
            start,
            end: start.add_byte(),
        },
    });
    TokenizerResult { tokens, errors }
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
        .or_else(|| try_tokenize_symbol(text))
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
