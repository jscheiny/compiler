use crate::{lexer::OperatorToken, parser::TokenTraverser};

pub fn comma_separated_list<T>(
    tokens: &mut TokenTraverser,
    close_symbol: OperatorToken,
    parse_entry: impl Fn(&mut TokenTraverser) -> Result<T, ()>,
) -> Result<Vec<T>, ()> {
    let mut entries = vec![];
    while !tokens.accept(&close_symbol) {
        entries.push(parse_entry(tokens)?);
        if tokens.accept(&OperatorToken::Comma) {
            continue;
        }
    }
    Ok(entries)
}
