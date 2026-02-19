use crate::{
    lexer::Symbol,
    parser::{Node, ParseResult, TokenStream},
};

pub fn comma_separated_list<T>(
    tokens: &mut TokenStream,
    close_symbol: Symbol,
    parse_entry: impl Fn(&mut TokenStream) -> ParseResult<T>,
) -> ParseResult<Vec<Node<T>>> {
    // TODO this could use better errors
    let mut entries = vec![];
    while !tokens.accept(&close_symbol) {
        entries.push(tokens.located(&parse_entry)?);
        if tokens.accept(&Symbol::Comma) {
            continue;
        }
    }
    Ok(entries)
}
