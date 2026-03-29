use crate::{
    lexer::Symbol,
    parser::{Node, ParseResult, SyntaxError, TokenStream},
};

pub fn comma_separated_list<T>(
    tokens: &mut TokenStream,
    close_symbol: Symbol,
    parse_entry: impl Fn(&mut TokenStream) -> ParseResult<T>,
) -> ParseResult<Vec<Node<T>>> {
    let mut entries = vec![];
    while !tokens.accept(close_symbol) {
        entries.push(tokens.located(&parse_entry)?);
        if tokens.accept(close_symbol) {
            break;
        }
        if !tokens.accept(Symbol::Comma) {
            tokens.push_error(SyntaxError::ExpectedComma);
        }
    }
    Ok(entries)
}
