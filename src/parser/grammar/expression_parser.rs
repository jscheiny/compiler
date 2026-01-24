use crate::{
    lexer::{IdentifierToken, IntegerLiteralToken, OperatorToken, StringLiteralToken, Token},
    parser::{BlockParseNode, ExpressionParseNode, ParseResult, TokenStream, grammar::statement},
};

pub fn expression(tokens: &mut TokenStream) -> ParseResult<ExpressionParseNode> {
    match tokens.peek() {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(ExpressionParseNode::Identifier(identifier))
        }
        Token::IntegerLiteral(IntegerLiteralToken(literal)) => {
            let literal = *literal;
            tokens.next();
            Ok(ExpressionParseNode::IntegerLiteral(literal))
        }
        Token::StringLiteral(StringLiteralToken(literal)) => {
            let literal = literal.clone();
            tokens.next();
            Ok(ExpressionParseNode::StringLiteral(literal))
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            let block = block(tokens)?;
            Ok(ExpressionParseNode::Block(block))
        }
        _ => Err(()),
    }
}

pub fn block(tokens: &mut TokenStream) -> ParseResult<BlockParseNode> {
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(tokens.located(statement)?);
    }
    Ok(BlockParseNode { statements })
}
