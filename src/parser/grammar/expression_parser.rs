use crate::{
    lexer::{
        IdentifierToken, IntegerLiteralToken, KeywordToken, OperatorToken, StringLiteralToken,
        Token, TokenMatch,
    },
    parser::{
        AccessExpressionNode, ArrayExpressionNode, Associativity, BinaryOpExpressionNode,
        BinaryOperator, BlockNode, ClosureExpressionNode, ClosureParameterExpressionNode,
        ExpressionNode, FunctionCallExpressionNode, Identified, IdentifierNode, IdentifierType,
        IfExpressionNode, LocatedSyntaxError, Node, Operator, ParseResult, PostfixOpExpressionNode,
        PostfixOperator, PrefixOpExpressionNode, PrefixOperator, StatementNode, StatementType,
        SyntaxError, TokenSpan, TokenStream,
        grammar::{statement, type_definition},
    },
};

#[derive(Clone, Copy, Default)]
struct ExpressionContext {
    pub min_precedence: i32,
    pub allow_commas: bool,
    pub allow_types: bool,
}

impl ExpressionContext {
    pub fn brackets() -> Self {
        Self {
            min_precedence: 0,
            allow_commas: true,
            allow_types: false,
        }
    }

    pub fn parentheses() -> Self {
        Self {
            min_precedence: 0,
            allow_commas: true,
            allow_types: true,
        }
    }

    pub fn with_precedence(self, min_precedence: i32, allow_types: bool) -> Self {
        Self {
            min_precedence,
            allow_types,
            ..self
        }
    }

    pub fn reset_precedence(self) -> Self {
        Self {
            min_precedence: 0,
            allow_types: false,
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
                break;
            }

            let operator = TokenSpan::singleton(tokens).wrap(operator);
            tokens.next();

            left = if operator.value == BinaryOperator::Type {
                complete_closure_parameter(tokens, left, context)
            } else {
                complete_binary_op(tokens, left, operator, context)
            }?
        } else if OperatorToken::OpenParen.matches(token) {
            // Function calls should be treated as the same precedence as a.b
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

fn complete_closure_parameter(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
    context: ExpressionContext,
) -> ParseResult<Node<ExpressionNode>> {
    if context.allow_types {
        if let ExpressionNode::Identifier(identifier) = left.value {
            let parameter_type = Some(tokens.located(type_definition)?);
            let identifier = left.span.wrap(IdentifierNode(identifier));
            let parameter_span = left.span.expand_to(tokens);
            return Ok(parameter_span.wrap(ExpressionNode::ClosureParameter(
                ClosureParameterExpressionNode {
                    identifier,
                    parameter_type,
                },
            )));
        }
    }

    if context.allow_types {
        tokens.errors.push(LocatedSyntaxError {
            span: left.span,
            error: SyntaxError::ExpectedIdentifier(IdentifierType::Parameter),
        });
    } else {
        tokens.push_error(SyntaxError::UnexpectedTypeExpression);
    }

    // Parse the type definition for errors and so we can continue parsing
    type_definition(tokens)?;
    let parameter_span = left.span.expand_to(tokens);
    Ok(parameter_span.wrap(ExpressionNode::Error))
}

fn complete_binary_op(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
    operator: Node<BinaryOperator>,
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

    let allow_types = operator.value == BinaryOperator::Comma;
    let context = context.with_precedence(next_min_precedence, allow_types);
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
        let context = context.with_precedence(precedence, false);
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
            // TODO handle empty tuple / function args
            tokens.next();
            let context = ExpressionContext::parentheses();
            let expression = tokens.located_with(sub_expression, context)?;
            tokens.expect(&OperatorToken::CloseParen, SyntaxError::ExpectedCloseParen)?;
            if tokens.accept(&OperatorToken::SkinnyArrow) {
                closure(tokens, expression)
            } else {
                Ok(expression.value)
            }
        }
        Token::Operator(OperatorToken::OpenBracket) => {
            tokens.next();
            let context = ExpressionContext::brackets();
            let elements = if tokens.accept(&OperatorToken::CloseBracket) {
                vec![]
            } else {
                let expression = tokens.located_with(sub_expression, context)?;
                tokens.expect(
                    &OperatorToken::CloseBracket,
                    SyntaxError::ExpectedCloseBracket,
                )?;
                flatten_commas(expression)
            };
            Ok(ExpressionNode::Array(ArrayExpressionNode { elements }))
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

fn closure(
    tokens: &mut TokenStream,
    args_expression: Node<ExpressionNode>,
) -> ParseResult<ExpressionNode> {
    let parameters = flatten_commas(args_expression)
        .into_iter()
        .map(|parameter| {
            if let ExpressionNode::Identifier(identifier) = parameter.value {
                Some(parameter.span.wrap(ClosureParameterExpressionNode {
                    identifier: parameter.span.wrap(IdentifierNode(identifier)),
                    parameter_type: None,
                }))
            } else if let ExpressionNode::ClosureParameter(param) = parameter.value {
                Some(parameter.span.wrap(param))
            } else {
                tokens.errors.push(LocatedSyntaxError {
                    span: parameter.span,
                    error: SyntaxError::ExpectedClosureParameter,
                });
                None
            }
        })
        .collect();

    let body = tokens.located(expression)?;
    Ok(ExpressionNode::Closure(ClosureExpressionNode {
        parameters,
        body: Box::new(body),
    }))
}
