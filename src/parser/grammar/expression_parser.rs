use crate::{
    lexer::{
        IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken, StringLiteralToken,
        Token, TokenMatch,
    },
    parser::{
        Associativity, BinaryOpExpressionNode, BinaryOperator, BlockNode, ExpressionNode,
        FunctionCallExpressionNode, IfExpressionNode, Node, Operator, ParseResult,
        PostfixOpExpressionNode, PostfixOperator, PrefixOpExpressionNode, PrefixOperator,
        SyntaxError, TokenSpan, TokenStream, grammar::statement,
    },
};

pub fn expression(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    sub_expression(tokens, 0)
}

fn sub_expression(tokens: &mut TokenStream, min_precedence: i32) -> ParseResult<ExpressionNode> {
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
            left = span.wrap(ExpressionNode::PostfixOp(PostfixOpExpressionNode {
                expression: Box::new(left),
                operator,
            }));
        } else if let Some(operator) = BinaryOperator::from_token(token) {
            if operator.precedence() < min_precedence {
                break;
            }

            let next_min_precedence = operator.precedence()
                + match operator.associativity() {
                    Associativity::Left => 1,
                    Associativity::Right => 0,
                };

            let operator = TokenSpan::singleton(tokens).wrap(operator);
            tokens.next();

            let right = tokens.located_with(sub_expression, next_min_precedence)?;
            let span = left.span.expand_to(tokens);
            left = span.wrap(ExpressionNode::BinaryOp(BinaryOpExpressionNode {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }));
        } else if OperatorToken::OpenParen.matches(token) {
            let precedence = BinaryOperator::Access.precedence();
            if precedence < min_precedence {
                break;
            }

            let arguments_span = TokenSpan::singleton(tokens);
            tokens.next();

            let right = tokens.located(expression)?;
            let arguments = flatten_arguments(right);

            tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
            let arguments_span = arguments_span.expand_to(tokens);
            let span = left.span.expand_to(tokens);

            left = span.wrap(ExpressionNode::FunctionCall(FunctionCallExpressionNode {
                function: Box::new(left),
                arguments: arguments_span.wrap(arguments),
            }))
        } else {
            break;
        }
    }

    Ok(left.value)
}

fn flatten_arguments(expression: Node<ExpressionNode>) -> Vec<Node<ExpressionNode>> {
    let mut arguments = vec![];
    let mut current = expression;
    loop {
        if let ExpressionNode::BinaryOp(BinaryOpExpressionNode {
            left,
            operator,
            right,
        }) = current.value
        {
            if operator.value != BinaryOperator::Comma {
                arguments.push(*right);
                break;
            }
            arguments.push(*left);
            current = *right;
        } else {
            arguments.push(current);
            break;
        }
    }

    arguments
}

fn expression_atom(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    let operator = PrefixOperator::from_token(tokens.peek());
    if let Some(operator) = operator {
        let precedence = operator.precedence();
        let operator = TokenSpan::singleton(tokens).wrap(operator);
        tokens.next();
        let expression = tokens.located_with(sub_expression, precedence)?;
        return Ok(ExpressionNode::PrefixOp(PrefixOpExpressionNode {
            operator,
            expression: Box::new(expression),
        }));
    }
    match tokens.peek() {
        Token::Identifier(IdentifierToken(identifier)) => {
            let identifier = identifier.clone();
            tokens.next();
            Ok(ExpressionNode::Identifier(identifier))
        }
        Token::IntegerLiteral(IntegerLiteralToken(literal)) => {
            let literal = *literal;
            tokens.next();
            Ok(ExpressionNode::IntegerLiteral(literal))
        }
        Token::StringLiteral(StringLiteralToken(literal)) => {
            let literal = literal.clone();
            tokens.next();
            Ok(ExpressionNode::StringLiteral(literal))
        }
        Token::Operator(OperatorToken::OpenBrace) => {
            let block = block(tokens)?;
            Ok(ExpressionNode::Block(block))
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
            Ok(ExpressionNode::IfExpression(IfExpressionNode {
                predicate: Box::new(predicate),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            }))
        }
        Token::Keyword(KeywordToken::True) => {
            tokens.next();
            Ok(ExpressionNode::BoolLiteral(true))
        }
        Token::Keyword(KeywordToken::False) => {
            tokens.next();
            Ok(ExpressionNode::BoolLiteral(false))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedExpression)),
    }
}

pub fn block(tokens: &mut TokenStream) -> ParseResult<BlockNode> {
    tokens.expect(&OperatorToken::OpenBrace, SyntaxError::ExpectedBlock)?;
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(tokens.located(statement)?);
    }
    Ok(BlockNode { statements })
}
