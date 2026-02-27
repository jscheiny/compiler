use crate::lexer::{
    CharacterLiteralTokenizer, CharacterLocation, CharacterSpan, IdentifierTokenizer,
    IntegerLiteralTokenizer, KeywordTokenizer, LocatedToken, MultiLineCommentTokenizer,
    SingleLineCommentTokenizer, StringLiteralTokenizer, SymbolTokenizer, Token, TokenWidth,
    WhitespaceTokenizer,
};

pub trait Tokenizer {
    fn try_tokenize(&self, text: &str) -> Option<TryTokenizeResult>;
}

pub struct TokenizerResult {
    pub tokens: Vec<LocatedToken>,
    pub errors: Vec<CharacterSpan>,
}

pub fn tokenize(mut text: &str) -> TokenizerResult {
    let tokenizers: &[Box<dyn Tokenizer>] = &[
        Box::new(SingleLineCommentTokenizer),
        Box::new(MultiLineCommentTokenizer),
        Box::new(SymbolTokenizer),
        Box::new(KeywordTokenizer),
        Box::new(StringLiteralTokenizer),
        Box::new(CharacterLiteralTokenizer),
        Box::new(IntegerLiteralTokenizer),
        Box::new(IdentifierTokenizer),
        Box::new(WhitespaceTokenizer),
    ];
    let mut errors = vec![];
    let mut tokens = vec![];
    let mut start: CharacterLocation = CharacterLocation {
        line: 0,
        column: 0,
        byte: 0,
    };

    while !text.is_empty() {
        while let Some(token) = next_token(text, tokenizers) {
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
            let token = next_token(slice, tokenizers);
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

fn next_token<'a>(text: &'a str, tokenizers: &[Box<dyn Tokenizer>]) -> Option<NextToken<'a>> {
    let mut selected_token: Option<TryTokenizeResult> = None;
    for tokenizer in tokenizers {
        let maybe_result = tokenizer.try_tokenize(text);
        if let Some(result) = maybe_result.as_ref() {
            if let Some(token) = selected_token.as_ref() {
                if result.width.bytes > token.width.bytes {
                    selected_token = maybe_result;
                }
            } else {
                selected_token = maybe_result;
            }
        }
    }

    selected_token.map(|result| {
        let (_, next) = text.split_at(result.width.bytes);
        NextToken {
            token: result.token,
            width: result.width,
            next,
        }
    })
}
