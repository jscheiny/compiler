use std::fmt::Debug;

use crate::{
    lexer::OperatorToken,
    parser::{LocatedNode, TokenTraverser},
};

pub fn comma_separated_list<T: Debug>(
    tokens: &mut TokenTraverser,
    close_symbol: OperatorToken,
    parse_entry: impl Fn(&mut TokenTraverser) -> Result<T, ()>,
) -> Result<Vec<LocatedNode<T>>, ()> {
    let mut entries = vec![];
    while !tokens.accept(&close_symbol) {
        entries.push(tokens.located(&parse_entry)?);
        if tokens.accept(&OperatorToken::Comma) {
            continue;
        }
    }
    Ok(entries)
}
