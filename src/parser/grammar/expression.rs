use crate::{
    lexer::OperatorToken,
    parser::{ExpressionParseNode, TokenTraverser, grammar::statement},
};

pub fn expression(tokens: &mut TokenTraverser) -> Result<ExpressionParseNode, ()> {
    if tokens.accept(&OperatorToken::OpenBrace) {
        block(tokens)
    } else {
        Ok(ExpressionParseNode::Identifier(String::new()))
    }
}

pub fn block(tokens: &mut TokenTraverser) -> Result<ExpressionParseNode, ()> {
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(statement(tokens)?);
    }
    Ok(ExpressionParseNode::Block(statements))
}
