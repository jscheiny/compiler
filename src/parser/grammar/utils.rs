use crate::{
    lexer::OperatorToken,
    parser::{ParseNode, ParseResult, TokenStream},
};

pub fn comma_separated_list<T>(
    tokens: &mut TokenStream,
    close_symbol: OperatorToken,
    parse_entry: impl Fn(&mut TokenStream) -> ParseResult<T>,
) -> ParseResult<Vec<ParseNode<T>>> {
    let mut entries = vec![];
    while !tokens.accept(&close_symbol) {
        entries.push(tokens.located(&parse_entry)?);
        if tokens.accept(&OperatorToken::Comma) {
            continue;
        }
    }
    Ok(entries)
}
