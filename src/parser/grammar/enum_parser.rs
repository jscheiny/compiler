use crate::{
    lexer::{Symbol, Token},
    parser::{
        EnumNode, EnumVariantNode, IdentifierType, Node, ParseResult, SyntaxError, TokenStream,
        grammar::{comma_separated_list, methods, type_definition},
    },
};

pub fn enumeration(tokens: &mut TokenStream) -> ParseResult<EnumNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Variant)?;
    let variants = tokens.located(enum_variants)?;
    let methods = methods(tokens)?;
    Ok(EnumNode::new(identifier, variants, methods))
}

fn enum_variants(tokens: &mut TokenStream) -> ParseResult<Vec<Node<EnumVariantNode>>> {
    match tokens.peek() {
        Token::Symbol(Symbol::OpenParen) => {
            tokens.next();
            comma_separated_list(tokens, Symbol::CloseParen, enum_variant)
        }
        Token::Symbol(Symbol::OpenBrace) => {
            tokens.push_error(SyntaxError::ExpectedVariants);
            Ok(vec![])
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedVariants)),
    }
}

fn enum_variant(tokens: &mut TokenStream) -> ParseResult<EnumVariantNode> {
    let identifier = tokens.identifier(IdentifierType::Variant)?;
    let type_def = if tokens.accept(Symbol::OpenParen) {
        let type_def = tokens.located(type_definition)?;
        tokens.expect(Symbol::CloseParen, SyntaxError::ExpectedCloseParen)?;
        Some(type_def)
    } else {
        None
    };
    Ok(EnumVariantNode::new(identifier, type_def))
}
