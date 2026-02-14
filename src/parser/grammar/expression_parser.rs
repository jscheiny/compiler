use crate::{
    lexer::{
        IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken, StringLiteralToken,
        Token, TokenMatch,
    },
    parser::{
        AccessExpressionNode, Associativity, BinaryOpExpressionNode, BinaryOperator, BlockNode,
        ExpressionNode, FunctionCallExpressionNode, Identified, IdentifierType, IfExpressionNode,
        LocatedSyntaxError, Node, Operator, ParseResult, PostfixOpExpressionNode, PostfixOperator,
        PrefixOpExpressionNode, PrefixOperator, StatementNode, StatementType, SyntaxError,
        TokenSpan, TokenStream, grammar::statement,
    },
};

#[derive(Clone, Copy, Default)]
struct ExpressionContext {
    pub min_precedence: i32,
    pub allow_commas: bool,
}

impl ExpressionContext {
    pub fn parentheses() -> Self {
        Self {
            min_precedence: 0,
            allow_commas: true,
        }
    }

    pub fn with_precedence(self, min_precedence: i32) -> Self {
        Self {
            min_precedence,
            ..self
        }
    }

    pub fn reset_precedence(self) -> Self {
        Self {
            min_precedence: 0,
            ..self
        }
    }
}

pub fn expression(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    sub_expression(tokens, Default::default())
}

fn sub_expression(
    tokens: &mut TokenStream,
    context: ExpressionContext,
) -> ParseResult<ExpressionNode> {
    let mut left = tokens.located_with(expression_atom, context)?;
    loop {
        let token = tokens.peek();
        if let Some(operator) = PostfixOperator::from_token(token) {
            if operator.precedence() < context.min_precedence {
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
            if operator.precedence() < context.min_precedence {
                break;
            }

            // TODO This should probably still be allowed to continue on...
            if operator == BinaryOperator::Comma && !context.allow_commas {
                return Err(tokens.make_error(SyntaxError::UnexpectedComma));
            }

            let operator = TokenSpan::singleton(tokens).wrap(operator);
            tokens.next();
            left = complete_binary_op(tokens, operator, left, context)?;
        } else if OperatorToken::OpenParen.matches(token) {
            let precedence = BinaryOperator::Access.precedence();
            if precedence < context.min_precedence {
                break;
            }

            let arguments_span = TokenSpan::singleton(tokens);
            tokens.next();

            let arguments = if tokens.accept(&OperatorToken::CloseParen) {
                vec![]
            } else {
                let context = ExpressionContext::parentheses();
                let right = tokens.located_with(sub_expression, context)?;
                tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
                flatten_commas(right)
            };

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

fn complete_binary_op(
    tokens: &mut TokenStream,
    operator: Node<BinaryOperator>,
    left: Node<ExpressionNode>,
    context: ExpressionContext,
) -> ParseResult<Node<ExpressionNode>> {
    if operator.value == BinaryOperator::Access {
        let field = tokens.identifier(IdentifierType::Field)?;
        let span = left.span.expand_to(tokens);
        return Ok(span.wrap(ExpressionNode::Access(AccessExpressionNode {
            left: Box::new(left),
            field,
        })));
    }

    let next_min_precedence = operator.precedence()
        + match operator.associativity() {
            Associativity::Left => 1,
            Associativity::Right => 0,
        };

    let context = context.with_precedence(next_min_precedence);
    let right = tokens.located_with(sub_expression, context)?;
    let span = left.span.expand_to(tokens);
    Ok(span.wrap(ExpressionNode::BinaryOp(BinaryOpExpressionNode {
        left: Box::new(left),
        operator,
        right: Box::new(right),
    })))
}

fn flatten_commas(expression: Node<ExpressionNode>) -> Vec<Node<ExpressionNode>> {
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

fn expression_atom(
    tokens: &mut TokenStream,
    context: ExpressionContext,
) -> ParseResult<ExpressionNode> {
    let operator = PrefixOperator::from_token(tokens.peek());
    if let Some(operator) = operator {
        let precedence = operator.precedence();
        let operator = TokenSpan::singleton(tokens).wrap(operator);
        tokens.next();
        let context = context.with_precedence(precedence);
        let expression = tokens.located_with(sub_expression, context)?;
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
            let block = block(tokens, BlockType::Expression)?;
            Ok(ExpressionNode::Block(block))
        }
        Token::Operator(OperatorToken::At) => {
            tokens.next();
            let identifier = tokens.identifier(IdentifierType::Field)?;
            Ok(ExpressionNode::SelfRef(identifier.id().clone()))
        }
        Token::Operator(OperatorToken::OpenParen) => {
            tokens.next();
            let expression = sub_expression(tokens, ExpressionContext::parentheses())?;
            tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
            Ok(expression)
        }
        Token::Keyword(KeywordToken::If) => {
            tokens.next();
            let context = context.reset_precedence();
            let predicate = tokens.located_with(sub_expression, context)?;
            tokens.expect(&KeywordToken::Then, SyntaxError::ExpectedThen)?;
            let if_true = tokens.located_with(sub_expression, context)?;
            tokens.expect(&KeywordToken::Else, SyntaxError::ExpectedElse)?;
            let if_false = tokens.located_with(sub_expression, context)?;
            Ok(ExpressionNode::IfExpression(IfExpressionNode {
                predicate: Box::new(predicate),
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
            }))
        }
        Token::Keyword(KeywordToken::True) => {
            tokens.next();
            Ok(ExpressionNode::BooleanLiteral(true))
        }
        Token::Keyword(KeywordToken::False) => {
            tokens.next();
            Ok(ExpressionNode::BooleanLiteral(false))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedExpression)),
    }
}

#[derive(Clone, Copy)]
pub enum BlockType {
    Statement(StatementType),
    Expression,
}

pub fn block(tokens: &mut TokenStream, block_type: BlockType) -> ParseResult<BlockNode> {
    tokens.expect(&OperatorToken::OpenBrace, SyntaxError::ExpectedBlock)?;
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(tokens.located_with(statement, block_type)?);
    }

    if matches!(block_type, BlockType::Expression) {
        for (index, statement) in statements.iter().enumerate() {
            if index != statements.len() - 1
                && matches!(statement.value, StatementNode::BlockReturn(_))
            {
                let error_index = statement.span.start_index;
                let span = TokenSpan {
                    start_index: error_index,
                    end_index: error_index,
                };
                tokens.errors.push(LocatedSyntaxError {
                    span,
                    error: SyntaxError::BlockReturnEarly,
                });
            }
        }
    }

    Ok(BlockNode { statements })
}
