use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        IdentifierType, ParseNode, ParseResult, RecordDefinitionParseNode, RecordFieldParseNode,
        RecordType, SyntaxError, TokenStream,
        grammar::{comma_separated_list, methods, type_definition},
    },
};

pub fn structure(tokens: &mut TokenStream) -> ParseResult<RecordDefinitionParseNode> {
    record(tokens, RecordType::Struct)
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
        RecordType::Struct => IdentifierType::Struct,
        RecordType::Tuple => IdentifierType::Tuple,
    };
    let identifier = tokens.identifier(identifier_type)?;
    let fields = tokens.located(fields)?;

    let methods = methods(tokens)?;
    Ok(RecordDefinitionParseNode {
        record_type,
        identifier,
        fields,
        methods,
    })
}

fn fields(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<RecordFieldParseNode>>> {
    match tokens.peek() {
        Token::Operator(OperatorToken::OpenParen) => {
            tokens.next();
            comma_separated_list(tokens, OperatorToken::CloseParen, field)
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            tokens.push_error(SyntaxError::ExpectedFields);
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedFields)),
    }
}

fn field(tokens: &mut TokenStream) -> ParseResult<RecordFieldParseNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let identifier = tokens.identifier(IdentifierType::Field)?;
    let error = SyntaxError::ExpectedType;
    match tokens.peek() {
        Token::Operator(OperatorToken::Colon) => {
            tokens.next();
            let type_def = Some(tokens.located(type_definition)?);
            Ok(RecordFieldParseNode {
                public,
                identifier,
                type_def,
            })
        }
        Token::Operator(OperatorToken::Comma) | Token::Operator(OperatorToken::CloseParen) => {
            tokens.push_error(error);
            Ok(RecordFieldParseNode {
                public,
                identifier,
                type_def: None,
            })
        }
        _ => Err(tokens.make_error(error)),
    }
}
