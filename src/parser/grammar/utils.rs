use crate::{
    lexer::OperatorToken,
    parser::{Node, ParseResult, TokenStream},
};

pub fn comma_separated_list<T>(
    tokens: &mut TokenStream,
    close_symbol: OperatorToken,
    parse_entry: impl Fn(&mut TokenStream) -> ParseResult<T>,
) -> ParseResult<Vec<Node<T>>> {
    // TODO this could use better errors
    let mut entries = vec![];
    while !tokens.accept(&close_symbol) {
        entries.push(tokens.located(&parse_entry)?);
        if tokens.accept(&OperatorToken::Comma) {
            continue;
        }
    }
    Ok(entries)
}
