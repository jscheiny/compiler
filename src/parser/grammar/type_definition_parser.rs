use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        LocatedNode, TokenTraverser, TypeDefinitionParseNode, UserDefinedTypeParseNode,
        grammar::comma_separated_list,
    },
};

pub fn type_definition(tokens: &mut TokenTraverser) -> Result<TypeDefinitionParseNode, ()> {
    primitive_type_definition(tokens).or_else(|_| user_type_definition(tokens))
}

fn primitive_type_definition(tokens: &mut TokenTraverser) -> Result<TypeDefinitionParseNode, ()> {
    if let Token::Keyword(keyword) = tokens.peek() {
        match keyword {
            KeywordToken::Bool | KeywordToken::Int | KeywordToken::Float => {
                let keyword = *keyword;
                tokens.next();
                Ok(TypeDefinitionParseNode::Primitive(keyword))
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

fn user_type_definition(tokens: &mut TokenTraverser) -> Result<TypeDefinitionParseNode, ()> {
    let identifier = tokens.identifier()?;
    let generic_params = tokens.maybe_located(generic_type_params)?;

    Ok(TypeDefinitionParseNode::User(UserDefinedTypeParseNode {
        identifier,
        generic_params,
    }))
}

fn generic_type_params(
    tokens: &mut TokenTraverser,
) -> Result<Option<Vec<LocatedNode<TypeDefinitionParseNode>>>, ()> {
    if tokens.accept(&OperatorToken::OpenBracket) {
        let params = comma_separated_list(tokens, OperatorToken::CloseBracket, type_definition)?;
        Ok(Some(params))
    } else {
        Ok(None)
    }
}
