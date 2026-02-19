use crate::{
    lexer::{KeywordToken, Symbol, Token},
    parser::{
        IdentifierType, Node, ParseResult, StructFieldNode, StructNode, SyntaxError, TokenStream,
        grammar::{comma_separated_list, methods, type_definition},
    },
};

pub fn structure(tokens: &mut TokenStream) -> ParseResult<StructNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Struct)?;
    let fields = tokens.located(fields)?;

    let methods = methods(tokens)?;
    Ok(StructNode::new(identifier, fields, methods))
}

fn fields(tokens: &mut TokenStream) -> ParseResult<Vec<Node<StructFieldNode>>> {
    match tokens.peek() {
        Token::Operator(Symbol::OpenParen) => {
            tokens.next();
            comma_separated_list(tokens, Symbol::CloseParen, field)
        }
        Token::Operator(Symbol::OpenBrace) => {
            tokens.push_error(SyntaxError::ExpectedFields);
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedFields)),
    }
}

fn field(tokens: &mut TokenStream) -> ParseResult<StructFieldNode> {
    let public = tokens.accept(&KeywordToken::Pub);
    let identifier = tokens.identifier(IdentifierType::Field)?;
    let error = SyntaxError::ExpectedType;
    match tokens.peek() {
        Token::Operator(Symbol::Colon) => {
            tokens.next();
            let type_def = Some(tokens.located(type_definition)?);
            Ok(StructFieldNode::new(public, identifier, type_def))
        }
        Token::Operator(Symbol::Comma) | Token::Operator(Symbol::CloseParen) => {
            tokens.push_error(error);
            Ok(StructFieldNode::new(public, identifier, None))
        }
        _ => Err(tokens.make_error(error)),
    }
}
