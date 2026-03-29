use crate::{
    lexer::{Keyword, Symbol, Token, TokenMatch},
    parser::{
        ArrayExpressionNode, Associativity, BinaryOpExpressionNode, BinaryOperator, BlockNode,
        DeferredMemberExpressionNode, ExpressionNode, IfExpressionNode, LocatedSyntaxError,
        NameType, Node, Operator, ParseResult, PostfixOpExpressionNode, PostfixOperator,
        PrefixOpExpressionNode, PrefixOperator, StatementNode, StatementType, SyntaxError,
        TokenSpan, TokenStream,
        grammar::{SpecialOperator, closure, match_expression, statement},
    },
};

#[derive(Clone, Copy, Default)]
struct ExpressionContext {
    pub min_precedence: i32,
    pub allow_commas: bool,
}

impl ExpressionContext {
    pub fn brackets() -> Self {
        Self {
            min_precedence: 0,
            allow_commas: true,
        }
    }

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

            left = binary_op_expression(tokens, left, operator, context)?;
        } else if let Some(operator) = SpecialOperator::from_token(token) {
            if operator.precedence() < context.min_precedence {
                break;
            }

            left = operator.parse(tokens, left)?;
        } else {
            break;
        }
    }

    Ok(left.value)
}

fn binary_op_expression(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
    operator: Node<BinaryOperator>,
    context: ExpressionContext,
) -> ParseResult<Node<ExpressionNode>> {
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

pub fn function_arguments(tokens: &mut TokenStream) -> ParseResult<Vec<Node<ExpressionNode>>> {
    tokens.next();

    if tokens.accept(Symbol::CloseParen) {
        Ok(vec![])
    } else {
        let context = ExpressionContext::parentheses();
        let right = tokens.located_with(sub_expression, context)?;
        tokens.expect(Symbol::CloseParen, SyntaxError::ExpectedCloseParen)?;
        Ok(flatten_commas(right))
    }
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
        Token::Keyword(Keyword::SelfValue) => {
            let span = TokenSpan::singleton(tokens);
            tokens.next();
            Ok(ExpressionNode::SelfValue(span))
        }
        Token::Name(name) => {
            let span = TokenSpan::singleton(tokens);
            let name = span.wrap(name.clone());
            tokens.next();
            Ok(ExpressionNode::Name(name))
        }
        Token::CharacterLiteral(literal) => {
            let literal = literal.clone();
            // TODO syntax checking on literal...
            tokens.next();
            Ok(ExpressionNode::CharacterLiteral(literal))
        }
        Token::IntegerLiteral(literal) => {
            let literal = *literal;
            tokens.next();
            Ok(ExpressionNode::IntegerLiteral(literal))
        }
        Token::StringLiteral(literal) => {
            let literal = literal.clone();
            tokens.next();
            Ok(ExpressionNode::StringLiteral(literal))
        }
        Token::Symbol(Symbol::OpenBrace) => {
            let block = block(tokens, BlockType::Expression)?;
            Ok(ExpressionNode::Block(block))
        }
        Token::Symbol(Symbol::At) => {
            tokens.next();
            let name = tokens.name(NameType::Field)?;
            Ok(ExpressionNode::SelfRef(name))
        }
        Token::Symbol(Symbol::Dot) => deferred_member(tokens),
        Token::Symbol(Symbol::OpenParen) => closure_or_tuple(tokens),
        Token::Symbol(Symbol::OpenBracket) => array(tokens),
        Token::Keyword(Keyword::If) => if_expression(tokens),
        Token::Keyword(Keyword::Match) => match_expression(tokens),
        Token::Keyword(Keyword::True) => {
            tokens.next();
            Ok(ExpressionNode::BooleanLiteral(true))
        }
        Token::Keyword(Keyword::False) => {
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
    tokens.expect(Symbol::OpenBrace, SyntaxError::ExpectedBlock)?;
    let mut statements = vec![];
    while !tokens.accept(Symbol::CloseBrace) {
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

fn deferred_member(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    tokens.next();
    let field = tokens.name(NameType::Field)?;
    let arguments = if Symbol::OpenParen.matches(tokens.peek()) {
        Some(tokens.located(function_arguments)?)
    } else {
        None
    };

    Ok(ExpressionNode::DeferredMember(
        DeferredMemberExpressionNode { field, arguments },
    ))
}

fn closure_or_tuple(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    tokens.next();
    if tokens.accept(Symbol::CloseParen) {
        tokens.expect(Symbol::SkinnyArrow, SyntaxError::ExpectedClosureBody)?;
        return closure(tokens, vec![]);
    }

    let context = ExpressionContext::parentheses();
    let expression = tokens.located_with(sub_expression, context)?;
    tokens.expect(Symbol::CloseParen, SyntaxError::ExpectedCloseParen)?;
    if tokens.accept(Symbol::SkinnyArrow) {
        closure(tokens, flatten_commas(expression))
    } else {
        Ok(expression.value)
    }
}

fn array(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    let elements = tokens.located(array_elements)?;
    Ok(ExpressionNode::Array(ArrayExpressionNode { elements }))
}

fn array_elements(tokens: &mut TokenStream) -> ParseResult<Vec<Node<ExpressionNode>>> {
    tokens.next();
    let context = ExpressionContext::brackets();
    if tokens.accept(Symbol::CloseBracket) {
        Ok(vec![])
    } else {
        let expression = tokens.located_with(sub_expression, context)?;
        tokens.expect(Symbol::CloseBracket, SyntaxError::ExpectedCloseBracket)?;
        Ok(flatten_commas(expression))
    }
}

fn if_expression(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    tokens.next();
    let predicate = tokens.located(expression)?;
    tokens.expect(Keyword::Then, SyntaxError::ExpectedThen)?;
    let if_true = tokens.located(expression)?;
    tokens.expect(Keyword::Else, SyntaxError::ExpectedElse)?;
    let if_false = tokens.located(expression)?;
    Ok(ExpressionNode::IfExpression(IfExpressionNode {
        predicate: Box::new(predicate),
        if_true: Box::new(if_true),
        if_false: Box::new(if_false),
    }))
}

fn flatten_commas(expression: Node<ExpressionNode>) -> Vec<Node<ExpressionNode>> {
    let mut arguments: Vec<Node<ExpressionNode>> = vec![];
    let mut current = expression;
    loop {
        if let ExpressionNode::BinaryOp(BinaryOpExpressionNode {
            left,
            operator,
            right,
        }) = current.value
        {
            if operator.value != BinaryOperator::Comma {
                arguments.push(current.span.wrap(ExpressionNode::BinaryOp(
                    BinaryOpExpressionNode {
                        left,
                        operator,
                        right,
                    },
                )));
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
