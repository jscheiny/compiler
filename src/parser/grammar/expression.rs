use crate::{
    lexer::{IdentifierToken, IntegerLiteralToken, OperatorToken, StringLiteralToken, Token},
    parser::{
        ExpressionParseNode, LocatedNodeVec, ParseResult, StatementParseNode, TokenTraverser,
        grammar::statement,
    },
};

pub fn expression(tokens: &mut TokenTraverser) -> ParseResult<ExpressionParseNode> {
    let span = tokens.start_span();
    match tokens.peek() {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(span.close(tokens, ExpressionParseNode::Identifier(identifier)))
        }
        Token::IntegerLiteral(IntegerLiteralToken(literal)) => {
            let literal = *literal;
            tokens.next();
            Ok(span.close(tokens, ExpressionParseNode::IntegerLiteral(literal)))
        }
        Token::StringLiteral(StringLiteralToken(literal)) => {
            let literal = literal.clone();
            tokens.next();
            Ok(span.close(tokens, ExpressionParseNode::StringLiteral(literal)))
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            let block = block(tokens)?;
            Ok(span.close(tokens, ExpressionParseNode::Block(block)))
        }
        _ => Err(()),
    }
}

pub fn block(tokens: &mut TokenTraverser) -> Result<LocatedNodeVec<StatementParseNode>, ()> {
    let span = tokens.start_span();
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(statement(tokens)?);
    }
    Ok(span.close(tokens, statements))
}
