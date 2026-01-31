use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        ParseNode, ParseResult, SyntaxErrorType, TokenStream, TypeParseNode,
        UserDefinedTypeParseNode,
        grammar::{comma_separated_list, identifier_parser::identifier_with},
    },
};

pub fn type_definition(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    let token = tokens.peek();
    match token {
        Token::Identifier(_) => user_defined_type(tokens),
        Token::Keyword(keyword) => match keyword {
            KeywordToken::Bool | KeywordToken::Int | KeywordToken::Float => {
                let keyword = *keyword;
                tokens.next();
                Ok(TypeParseNode::Primitive(keyword))
            }
            _ => user_defined_type(tokens),
        },
        _ => Err(tokens.make_error(SyntaxErrorType::ExpectedType)),
    }
}

fn user_defined_type(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    let identifier = tokens.located_with(identifier_with, SyntaxErrorType::ExpectedType)?;
    let generic_params = tokens.maybe_located(generic_type_params)?;

    Ok(TypeParseNode::UserDefined(UserDefinedTypeParseNode {
        identifier,
        generic_params,
    }))
}

fn generic_type_params(
    tokens: &mut TokenStream,
) -> ParseResult<Option<Vec<ParseNode<TypeParseNode>>>> {
    if tokens.accept(&OperatorToken::OpenBracket) {
        let params = comma_separated_list(tokens, OperatorToken::CloseBracket, type_definition)?;
        Ok(Some(params))
    } else {
        Ok(None)
    }
}
