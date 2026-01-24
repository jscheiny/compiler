use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        ParseNode, ParseResult, TokenStream, TypeParseNode, UserDefinedTypeParseNode,
        grammar::comma_separated_list,
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
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

fn user_defined_type(tokens: &mut TokenStream) -> ParseResult<TypeParseNode> {
    let identifier = tokens.identifier()?;
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
