use crate::lexer::{
    CharacterLocation, CharacterSpan, IdentifierToken, IntegerLiteralToken, KeywordToken,
    LocatedToken, OperatorToken, StringLiteralToken, Token, TokenParse, WhitespaceToken,
};

pub fn tokenize(mut text: &str) -> Vec<LocatedToken> {
    let mut tokens = vec![];
    let mut start: CharacterLocation = CharacterLocation {
        line: 0,
        column: 0,
        byte: 0,
    };
    while let Some((token, token_len, next)) = next_token(text) {
        let end: CharacterLocation = match token {
            Token::Whitespace(token) => {
                if token.new_lines == 0 {
                    // TODO this may not actually be the number of columns (characters)
                    start.add_columns(token_len)
                } else {
                    start.add_lines(token, token_len)
                }
            }
            token => {
                let end = CharacterLocation {
                    line: start.line,
                    column: start.column + token_len,
                    byte: start.byte + token_len,
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

fn next_token(text: &str) -> Option<(Token, usize, &str)> {
    OperatorToken::try_tokenize(text)
        .or_else(|| KeywordToken::try_tokenize(text))
        .or_else(|| StringLiteralToken::try_tokenize(text))
        .or_else(|| IntegerLiteralToken::try_tokenize(text))
        .or_else(|| IdentifierToken::try_tokenize(text))
        .or_else(|| WhitespaceToken::try_tokenize(text))
        .map(|(token, len)| {
            let (_, end) = text.split_at(len);
            (token, len, end)
        })
}
