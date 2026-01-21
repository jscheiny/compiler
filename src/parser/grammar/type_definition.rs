use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        TokenTraverser, TypeDefinitionParseNode, UserDefinedTypeParseNode,
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
                let keyword = keyword.clone();
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
    let identifier = tokens.identifier().ok_or(())?;
    let mut generic_params = vec![];
    if tokens.accept(&OperatorToken::OpenBracket) {
        generic_params =
            comma_separated_list(tokens, OperatorToken::CloseBracket, type_definition)?;
    }
    Ok(TypeDefinitionParseNode::User(UserDefinedTypeParseNode {
        identifier,
        generic_params,
    }))
}
