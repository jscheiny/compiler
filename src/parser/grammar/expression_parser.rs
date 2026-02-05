use crate::{
    lexer::{IdentifierToken, IntegerLiteralToken, OperatorToken, StringLiteralToken, Token},
    parser::{
        BinaryOpExpressionParseNode, BlockParseNode, ExpressionParseNode, ParseResult,
        PostfixOpExpressionParseNode, PrefixOpExpressionParseNode, SyntaxError, TokenStream,
        grammar::statement,
        operator::{Associativity, BinaryOperator, Operator, PostfixOperator, PrefixOperator},
    },
};

pub fn expression(tokens: &mut TokenStream) -> ParseResult<ExpressionParseNode> {
    sub_expression(tokens, 1)
}

fn sub_expression(
    tokens: &mut TokenStream,
    min_precedence: i32,
) -> ParseResult<ExpressionParseNode> {
    let mut left = expression_atom(tokens)?;
    loop {
        let token = tokens.peek();
        if let Some(operator) = PostfixOperator::from_token(token) {
            if operator.precedence() < min_precedence {
                break;
            }

            tokens.next();
            left = ExpressionParseNode::PostfixOp(PostfixOpExpressionParseNode {
                expression: Box::new(left),
                operator,
            });
        } else if let Some(operator) = BinaryOperator::from_token(token) {
            if operator.precedence() < min_precedence {
                break;
            }

            let next_min_precedence = operator.precedence()
                + match operator.associativity() {
                    Associativity::Left => 0,
                    Associativity::Right => 1,
                };
            tokens.next();
            let right = sub_expression(tokens, next_min_precedence)?;
            left = ExpressionParseNode::BinaryOp(BinaryOpExpressionParseNode {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        } else {
            break;
        }
    }

    Ok(left)
}

fn expression_atom(tokens: &mut TokenStream) -> ParseResult<ExpressionParseNode> {
    let operator = PrefixOperator::from_token(tokens.peek());
    if let Some(operator) = operator {
        tokens.next();
        let expression = sub_expression(tokens, operator.precedence())?;
        return Ok(ExpressionParseNode::PrefixOp(PrefixOpExpressionParseNode {
            operator,
            expression: Box::new(expression),
        }));
    }
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
        Token::Operator(OperatorToken::OpenParen) => {
            tokens.next();
            let expression = sub_expression(tokens, 1)?;
            tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
            Ok(expression)
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedExpression)),
    }
}

pub fn block(tokens: &mut TokenStream) -> ParseResult<BlockParseNode> {
    tokens.expect(&OperatorToken::OpenBrace, SyntaxError::ExpectedBlock)?;
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(tokens.located(statement)?);
    }
    Ok(BlockParseNode { statements })
}
