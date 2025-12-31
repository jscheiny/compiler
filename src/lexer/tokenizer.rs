use crate::lexer::{
    CharacterLocation, IdentifierToken, IntegerLiteralToken, KeywordToken, LocatedToken,
    OperatorToken, Token, TokenParse, WhitespaceToken,
};

pub fn tokenize(mut text: &str) -> Vec<LocatedToken> {
    let mut tokens = vec![];
    let mut start: CharacterLocation = CharacterLocation { line: 0, column: 0 };
    while let Some((token, token_len, next)) = next_token(text) {
        let end: CharacterLocation = match token {
            Token::Whitespace(token) => {
                if token.new_lines == 0 {
                    start.add_columns(token_len)
                } else {
                    start.add_lines(token)
                }
            }
            token => {
                let end = CharacterLocation {
                    line: start.line,
                    column: start.column + token_len,
                };
                tokens.push(LocatedToken { token, start, end });
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
        .or_else(|| IntegerLiteralToken::try_tokenize(text))
        .or_else(|| IdentifierToken::try_tokenize(text))
        .or_else(|| WhitespaceToken::try_tokenize(text))
        .map(|(token, len)| {
            let (_, end) = text.split_at(len);
            return (token, len, end);
        })
}
