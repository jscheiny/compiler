use crate::{
    lexer::OperatorToken,
    parser::{
        EnumParseNode, EnumVariantParseNode, ParseNode, ParseResult, SyntaxError, TokenStream,
        grammar::{comma_separated_list, identifier, methods, type_definition},
    },
};

pub fn enumeration(tokens: &mut TokenStream) -> ParseResult<EnumParseNode> {
    tokens.next();
    let identifier = tokens.located(identifier)?;
    let variants = tokens.located(enum_variants)?;
    let methods = methods(tokens)?;
    Ok(EnumParseNode {
        identifier,
        variants,
        methods,
    })
}

fn enum_variants(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<EnumVariantParseNode>>> {
    tokens.expect(&OperatorToken::OpenParen, SyntaxError::Unimplemented)?;
    comma_separated_list(tokens, OperatorToken::CloseParen, enum_variant)
}

fn enum_variant(tokens: &mut TokenStream) -> ParseResult<EnumVariantParseNode> {
    let identifier = tokens.located(identifier)?;
    let type_def = if tokens.accept(&OperatorToken::OpenParen) {
        let type_def = tokens.located(type_definition)?;
        tokens.expect(&OperatorToken::CloseParen, SyntaxError::Unimplemented)?;
        Some(type_def)
    } else {
        None
    };
    Ok(EnumVariantParseNode {
        identifier,
        type_def,
    })
}
