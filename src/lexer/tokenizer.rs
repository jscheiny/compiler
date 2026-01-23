use crate::lexer::{
    CharacterLocation, CharacterSpan, IdentifierToken, IntegerLiteralToken, KeywordToken,
    LocatedToken, OperatorToken, StringLiteralToken, Token, TokenParse,
    try_tokenize_multiline_comment, try_tokenize_single_line_comment, try_tokenize_whitespace,
};

pub fn tokenize(mut text: &str) -> Vec<LocatedToken> {
    let mut tokens = vec![];
    let mut start: CharacterLocation = CharacterLocation {
        line: 0,
        column: 0,
        byte: 0,
    };
    while let Some((token, bytes, next)) = next_token(text) {
        let end: CharacterLocation = match token {
            Token::Ignored(token) => {
                if token.new_lines == 0 {
                    // TODO this may not actually be the number of columns (characters)
                    start.add_columns(bytes, bytes)
                } else {
                    start.add_lines(token, bytes)
                }
            }
            token => {
                let end = CharacterLocation {
                    line: start.line,
                    column: start.column + bytes, // this should be columns somehow
                    byte: start.byte + bytes,
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
    try_tokenize_single_line_comment(text)
        .or_else(|| try_tokenize_multiline_comment(text))
        .or_else(|| OperatorToken::try_tokenize(text))
        .or_else(|| KeywordToken::try_tokenize(text))
        .or_else(|| StringLiteralToken::try_tokenize(text))
        .or_else(|| IntegerLiteralToken::try_tokenize(text))
        .or_else(|| IdentifierToken::try_tokenize(text))
        .or_else(|| try_tokenize_whitespace(text))
        .map(|(token, len)| {
            let (_, end) = text.split_at(len);
            (token, len, end)
        })
}
