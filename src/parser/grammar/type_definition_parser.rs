use crate::{
    lexer::{IdentifierToken, KeywordToken, OperatorToken, Token},
    parser::{
        FunctionTypeParseNode, ParseNode, ParseResult, SyntaxError, TokenStream,
        TupleTypeParseNode, TypeParseNode, grammar::comma_separated_list,
    },
};

pub fn type_definition(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(TypeParseNode::UserDefined(identifier))
        }
        Token::Keyword(keyword) => match keyword {
            KeywordToken::Bool | KeywordToken::Int | KeywordToken::Float => {
                let keyword = *keyword;
                tokens.next();
                Ok(TypeParseNode::Primitive(keyword))
            }
            _ => Err(tokens.make_error(SyntaxError::ExpectedType)),
        },
        Token::Operator(OperatorToken::OpenParen) => function_or_tuple_type(tokens),
        _ => Err(tokens.make_error(SyntaxError::ExpectedType)),
    }
}

fn function_or_tuple_type(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    tokens.next();
    let parameters = tokens.located(type_list)?;
    if tokens.accept(&OperatorToken::SkinnyArrow) {
        let return_type = tokens.located(type_definition)?;
        Ok(TypeParseNode::Function(FunctionTypeParseNode {
            parameters,
            return_type: Box::new(return_type),
        }))
    } else {
        Ok(TypeParseNode::Tuple(TupleTypeParseNode {
            fields: parameters.value,
        }))
    }
}

fn type_list(tokens: &mut TokenStream) -> ParseResult<Vec<ParseNode<TypeParseNode>>> {
    comma_separated_list(tokens, OperatorToken::CloseParen, type_definition)
}
