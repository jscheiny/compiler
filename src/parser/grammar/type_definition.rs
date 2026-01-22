use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        LocatedNodeVec, ParseResult, TokenTraverser, TypeDefinitionParseNode,
        UserDefinedTypeParseNode, grammar::comma_separated_list,
    },
};

pub fn type_definition(tokens: &mut TokenTraverser) -> ParseResult<TypeDefinitionParseNode> {
    primitive_type_definition(tokens).or_else(|_| user_type_definition(tokens))
}

fn primitive_type_definition(tokens: &mut TokenTraverser) -> ParseResult<TypeDefinitionParseNode> {
    if let Token::Keyword(keyword) = tokens.peek() {
        match keyword {
            KeywordToken::Bool | KeywordToken::Int | KeywordToken::Float => {
                let span = tokens.start_span();
                let keyword = *keyword;
                tokens.next();
                Ok(span.singleton(TypeDefinitionParseNode::Primitive(keyword)))
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

fn user_type_definition(tokens: &mut TokenTraverser) -> ParseResult<TypeDefinitionParseNode> {
    let span = tokens.start_span();
    let identifier = tokens.identifier().ok_or(())?;
    let generic_params = generic_type_params(tokens)?;

    Ok(span.close(
        tokens,
        TypeDefinitionParseNode::User(UserDefinedTypeParseNode {
            identifier,
            generic_params,
        }),
    ))
}

fn generic_type_params(
    tokens: &mut TokenTraverser,
) -> Result<Option<LocatedNodeVec<TypeDefinitionParseNode>>, ()> {
    let span = tokens.start_span();
    if tokens.accept(&OperatorToken::OpenBracket) {
        let params = comma_separated_list(tokens, OperatorToken::CloseBracket, type_definition)?;
        Ok(Some(span.close(tokens, params)))
    } else {
        Ok(None)
    }
}
