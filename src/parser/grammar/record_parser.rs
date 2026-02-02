use crate::{
    lexer::{KeywordToken, OperatorToken},
    parser::{
        ParseNode, ParseResult, RecordDefinitionParseNode, RecordMemberParseNode, RecordType,
        SyntaxError, TokenStream,
        grammar::{comma_separated_list, identifier, methods, type_definition},
    },
};

pub fn structure(tokens: &mut TokenStream) -> ParseResult<RecordDefinitionParseNode> {
    record(tokens, RecordType::Structure)
}

pub fn tuple(tokens: &mut TokenStream) -> ParseResult<RecordDefinitionParseNode> {
    record(tokens, RecordType::Tuple)
}

fn record(
    tokens: &mut TokenStream,
    record_type: RecordType,
) -> ParseResult<RecordDefinitionParseNode> {
    tokens.next();

    let identifier = tokens.located(identifier)?;
    let members = tokens.located(members)?;

    let methods = methods(tokens)?;
    Ok(RecordDefinitionParseNode {
        record_type,
        identifier,
        members,
        methods,
    })
}

fn members(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<RecordMemberParseNode>>> {
    tokens.expect(&OperatorToken::OpenParen, SyntaxError::Unimplemented)?;
    comma_separated_list(tokens, OperatorToken::CloseParen, member)
}

fn member(tokens: &mut TokenStream) -> ParseResult<RecordMemberParseNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let identifier = tokens.located(identifier)?;
    tokens.expect(&OperatorToken::Type, SyntaxError::Unimplemented)?;
    let type_def = tokens.located(type_definition)?;
    Ok(RecordMemberParseNode {
        public,
        identifier,
        type_def,
    })
}
