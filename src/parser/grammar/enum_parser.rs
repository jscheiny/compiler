use crate::{
    lexer::OperatorToken,
    parser::{
        EnumParseNode, EnumVariantParseNode, ParseNode, ParseResult, TokenStream,
        grammar::{type_definition_parser::type_definition, utils::comma_separated_list},
    },
};

pub fn enumeration(tokens: &mut TokenStream) -> ParseResult<EnumParseNode> {
    tokens.next();
    let identifier = tokens.identifier()?;
    let variants = tokens.located(enum_variants)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(EnumParseNode {
        identifier,
        variants,
        methods: None, // todo fill this in
    })
}

fn enum_variants(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<EnumVariantParseNode>>> {
    tokens.expect(&OperatorToken::OpenParen)?;
    comma_separated_list(tokens, OperatorToken::CloseParen, enum_variant)
}

fn enum_variant(tokens: &mut TokenStream) -> ParseResult<EnumVariantParseNode> {
    let identifier = tokens.identifier()?;
    let type_def = if tokens.accept(&OperatorToken::OpenParen) {
        let type_def = tokens.located(type_definition)?;
        tokens.expect(&OperatorToken::CloseParen)?;
        Some(type_def)
    } else {
        None
    };
    Ok(EnumVariantParseNode {
        identifier,
        type_def,
    })
}
