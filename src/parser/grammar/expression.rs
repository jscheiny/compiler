use crate::{
    lexer::{IdentifierToken, IntegerLiteralToken, OperatorToken, StringLiteralToken, Token},
    parser::{ExpressionParseNode, StatementParseNode, TokenTraverser, grammar::statement},
};

pub fn expression(tokens: &mut TokenTraverser) -> Result<ExpressionParseNode, ()> {
    match tokens.peek() {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(ExpressionParseNode::Identifier(identifier.clone()))
        }
        Token::IntegerLiteral(IntegerLiteralToken(literal)) => {
            let literal = *literal;
            tokens.next();
            Ok(ExpressionParseNode::IntegerLiteral(literal))
        }
        Token::StringLiteral(StringLiteralToken(literal)) => {
            let literal = literal.clone();
            tokens.next();
            Ok(ExpressionParseNode::StringLiteral(literal.clone()))
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            tokens.next();
            Ok(ExpressionParseNode::Block(block(tokens)?))
        }
        _ => Err(()),
    }
}

pub fn block(tokens: &mut TokenTraverser) -> Result<Vec<StatementParseNode>, ()> {
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(statement(tokens)?);
    }
    Ok(statements)
}
