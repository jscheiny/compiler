use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        IdentifierType, ParseNode, ParseResult, RecordDefinitionParseNode, RecordMemberParseNode,
        RecordType, SyntaxError, TokenStream,
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

    let identifier_type = match record_type {
        RecordType::Structure => IdentifierType::Struct,
        RecordType::Tuple => IdentifierType::Tuple,
    };
    let identifier = tokens.located_with(identifier, identifier_type)?;
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
    match tokens.peek() {
        Token::Operator(OperatorToken::OpenParen) => {
            tokens.next();
            comma_separated_list(tokens, OperatorToken::CloseParen, member)
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            tokens.push_error(SyntaxError::ExpectedMembers);
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedMembers)),
    }
}

fn member(tokens: &mut TokenStream) -> ParseResult<RecordMemberParseNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let identifier = tokens.located_with(identifier, IdentifierType::Member)?;
    let error = SyntaxError::ExpectedType;
    match tokens.peek() {
        Token::Operator(OperatorToken::Type) => {
            tokens.next();
            let type_def = Some(tokens.located(type_definition)?);
            Ok(RecordMemberParseNode {
                public,
                identifier,
                type_def,
            })
        }
        Token::Operator(OperatorToken::Comma) | Token::Operator(OperatorToken::CloseParen) => {
            tokens.push_error(error);
            Ok(RecordMemberParseNode {
                public,
                identifier,
                type_def: None,
            })
        }
        _ => Err(tokens.make_error(error)),
    }
}
