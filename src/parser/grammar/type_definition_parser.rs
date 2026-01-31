use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        ParseNode, ParseResult, SyntaxError, SyntaxErrorType, TokenStream, TypeParseNode,
        UserDefinedTypeParseNode,
        grammar::{comma_separated_list, identifier_parser::identifier_fail},
    },
};

pub fn type_definition(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    primitive_type(tokens).or_else(|_| user_defined_type(tokens))
}

fn primitive_type(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    if let Token::Keyword(keyword) = tokens.peek() {
        match keyword {
            KeywordToken::Bool | KeywordToken::Int | KeywordToken::Float => {
                let keyword = *keyword;
                tokens.next();
                Ok(TypeParseNode::Primitive(keyword))
            }
            _ => Err(tokens.make_error(SyntaxErrorType::ExpectedType)),
        }
    } else {
        Err(tokens.make_error(SyntaxErrorType::ExpectedType))
    }
}

fn user_defined_type(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    let identifier = tokens.located(identifier_fail).map_err(|err| SyntaxError {
        kind: SyntaxErrorType::ExpectedType,
        ..err
    })?;
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
