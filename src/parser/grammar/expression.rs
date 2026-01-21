use crate::{
    lexer::OperatorToken,
    parser::{ExpressionParseNode, StatementParseNode, TokenTraverser, grammar::statement},
};

pub fn expression(tokens: &mut TokenTraverser) -> Result<ExpressionParseNode, ()> {
    if tokens.accept(&OperatorToken::OpenBrace) {
        Ok(ExpressionParseNode::Block(block(tokens)?))
    } else {
        Ok(ExpressionParseNode::Identifier(String::new()))
    }
}

pub fn block(tokens: &mut TokenTraverser) -> Result<Vec<StatementParseNode>, ()> {
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(statement(tokens)?);
    }
    Ok(statements)
}
