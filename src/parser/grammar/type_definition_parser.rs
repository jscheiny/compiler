use crate::{
    lexer::{IdentifierToken, KeywordToken, OperatorToken, Token},
    parser::{
        FunctionTypeNode, Node, ParseResult, PrimitiveType, SyntaxError, TokenStream,
        TupleTypeNode, TypeNode, grammar::comma_separated_list,
    },
};

pub fn type_definition(tokens: &mut TokenStream) -> ParseResult<TypeNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(TypeNode::UserDefined(identifier))
        }
        Token::Keyword(KeywordToken::Void) => {
            tokens.next();
            Ok(TypeNode::Void)
        }
        Token::Keyword(keyword) => {
            let keyword = *keyword;
            tokens.next();
            match PrimitiveType::from_token(keyword) {
                Some(primitive_type) => Ok(TypeNode::Primitive(primitive_type)),
                None => Err(tokens.make_error(SyntaxError::ExpectedType)),
            }
        }
        Token::Operator(OperatorToken::OpenParen) => function_or_tuple_type(tokens),
        _ => Err(tokens.make_error(SyntaxError::ExpectedType)),
    }
}

fn function_or_tuple_type(tokens: &mut TokenStream) -> ParseResult<TypeNode> {
    tokens.next();
    let parameters = tokens.located(type_list)?;
    if tokens.accept(&OperatorToken::SkinnyArrow) {
        let return_type = tokens.located(type_definition)?;
        Ok(TypeNode::Function(FunctionTypeNode::new(
            parameters,
            Box::new(return_type),
        )))
    } else {
        Ok(TypeNode::Tuple(TupleTypeNode::new(parameters.value)))
    }
}

fn type_list(tokens: &mut TokenStream) -> ParseResult<Vec<Node<TypeNode>>> {
    comma_separated_list(tokens, OperatorToken::CloseParen, type_definition)
}
