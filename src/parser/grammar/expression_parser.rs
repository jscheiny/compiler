use crate::{
    lexer::{
        IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken, StringLiteralToken,
        Token,
    },
    parser::{
        BinaryOpExpressionParseNode, BlockParseNode, ExpressionParseNode, IfExpressionParseNode,
        ParseResult, PostfixOpExpressionParseNode, PrefixOpExpressionParseNode, SyntaxError,
        TokenSpan, TokenStream,
        grammar::statement,
        operator::{Associativity, BinaryOperator, Operator, PostfixOperator, PrefixOperator},
    },
};

pub fn expression(tokens: &mut TokenStream) -> ParseResult<ExpressionParseNode> {
    sub_expression(tokens, 0)
}

fn sub_expression(
    tokens: &mut TokenStream,
    min_precedence: i32,
) -> ParseResult<ExpressionParseNode> {
    let mut left = tokens.located(expression_atom)?;
    loop {
        let token = tokens.peek();
        if let Some(operator) = PostfixOperator::from_token(token) {
            if operator.precedence() < min_precedence {
                break;
            }

            let operator = TokenSpan::singleton(tokens).wrap(operator);
            tokens.next();

            let span = left.span.expand_to(tokens);
            left = span.wrap(ExpressionParseNode::PostfixOp(
                PostfixOpExpressionParseNode {
                    expression: Box::new(left),
                    operator,
                },
            ));
        } else if let Some(operator) = BinaryOperator::from_token(token) {
            if operator.precedence() < min_precedence {
                break;
            }

            let is_function_call = operator == BinaryOperator::FunctionCall;
            let next_min_precedence = if is_function_call {
                0
            } else {
                operator.precedence()
                    + match operator.associativity() {
                        Associativity::Left => 1,
                        Associativity::Right => 0,
                    }
            };

            let operator = TokenSpan::singleton(tokens).wrap(operator);
            tokens.next();

            let right = tokens.located_with(sub_expression, next_min_precedence)?;
            if is_function_call {
                tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
            }
            let span = left.span.expand_to(tokens);
            left = span.wrap(ExpressionParseNode::BinaryOp(BinaryOpExpressionParseNode {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }));
        } else {
            break;
        }
    }

    Ok(left.value)
}

fn expression_atom(tokens: &mut TokenStream) -> ParseResult<ExpressionParseNode> {
    let operator = PrefixOperator::from_token(tokens.peek());
    if let Some(operator) = operator {
        let precedence = operator.precedence();
        let operator = TokenSpan::singleton(tokens).wrap(operator);
        tokens.next();
        let expression = tokens.located_with(sub_expression, precedence)?;
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
            let expression = expression(tokens)?;
            tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
            Ok(expression)
        }
        Token::Keyword(KeywordToken::If) => {
            tokens.next();
            let predicate = tokens.located(expression)?;
            tokens.expect(&KeywordToken::Then, SyntaxError::ExpectedThen)?;
            let if_true = tokens.located(expression)?;
            tokens.expect(&KeywordToken::Else, SyntaxError::ExpectedElse)?;
            let if_false = tokens.located(expression)?;
            Ok(ExpressionParseNode::IfExpression(IfExpressionParseNode {
                predicate: Box::new(predicate),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            }))
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
