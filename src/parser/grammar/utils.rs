use std::fmt::Debug;

use crate::{
    lexer::OperatorToken,
    parser::{ParseNode, TokenStream},
};

pub fn comma_separated_list<T: Debug>(
    tokens: &mut TokenStream,
    close_symbol: OperatorToken,
    parse_entry: impl Fn(&mut TokenStream) -> Result<T, ()>,
) -> Result<Vec<ParseNode<T>>, ()> {
    let mut entries = vec![];
    while !tokens.accept(&close_symbol) {
        entries.push(tokens.located(&parse_entry)?);
        if tokens.accept(&OperatorToken::Comma) {
            continue;
        }
    }
    Ok(entries)
}
