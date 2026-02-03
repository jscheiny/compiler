use crate::{
    lexer::{OperatorToken, Token},
    parser::{
        EnumParseNode, EnumVariantParseNode, IdentifierType, ParseNode, ParseResult, SyntaxError,
        TokenStream,
        grammar::{comma_separated_list, identifier, methods, type_definition},
    },
};

pub fn enumeration(tokens: &mut TokenStream) -> ParseResult<EnumParseNode> {
    tokens.next();
    let identifier = tokens.located_with(identifier, IdentifierType::Variant)?;
    let variants = tokens.located(enum_variants)?;
    let methods = methods(tokens)?;
    Ok(EnumParseNode {
        identifier,
        variants,
        methods,
    })
}

fn enum_variants(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<EnumVariantParseNode>>> {
    match tokens.peek() {
        Token::Operator(OperatorToken::OpenParen) => {
            tokens.next();
            comma_separated_list(tokens, OperatorToken::CloseParen, enum_variant)
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            tokens.push_error(SyntaxError::ExpectedVariants);
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedVariants)),
    }
}

fn enum_variant(tokens: &mut TokenStream) -> ParseResult<EnumVariantParseNode> {
    let identifier = tokens.located_with(identifier, IdentifierType::BAD)?;
    let type_def = if tokens.accept(&OperatorToken::OpenParen) {
        let type_def = tokens.located(type_definition)?;
        tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
        Some(type_def)
    } else {
        None
    };
    Ok(EnumVariantParseNode {
        identifier,
        type_def,
    })
}
