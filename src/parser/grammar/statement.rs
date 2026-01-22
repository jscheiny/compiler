use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        DeclarationParseNode, ExpressionParseNode, IfStatementConditionParseNode,
        IfStatementParseNode, ParseResult, StatementParseNode, TokenTraverser, WhileLoopParseNode,
        grammar::{block, expression, type_definition},
    },
};

pub fn statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    match tokens.peek() {
        Token::Keyword(keyword) => match keyword {
            KeywordToken::Let => declaration(tokens, false),
            KeywordToken::Mut => declaration(tokens, true),
            KeywordToken::Return => function_return(tokens),
            KeywordToken::Break => break_statement(tokens),
            KeywordToken::Continue => continue_statement(tokens),
            KeywordToken::While => while_loop(tokens),
            KeywordToken::If => if_statement(tokens),
            _ => expression_statement(tokens),
        },
        Token::Operator(operator) => match operator {
            OperatorToken::OpenBrace => block_statement(tokens),
            OperatorToken::FunctionDefinition => block_return(tokens),
            _ => expression_statement(tokens),
        },
        _ => expression_statement(tokens),
    }
}

fn declaration(tokens: &mut TokenTraverser, mutable: bool) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    let identifier = tokens.identifier().ok_or(())?;
    let type_def = if tokens.accept(&OperatorToken::Type) {
        Some(type_definition(tokens)?)
    } else {
        None
    };

    tokens.expect(&OperatorToken::Assign)?;
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(span.close(
        tokens,
        StatementParseNode::Declaration(DeclarationParseNode {
            mutable,
            identifier,
            type_def,
            expression,
        }),
    ))
}

fn function_return(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(span.close(tokens, StatementParseNode::FunctionReturn(None)))
    } else {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(span.close(tokens, StatementParseNode::FunctionReturn(Some(expression))))
    }
}

fn break_statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(span.close(tokens, StatementParseNode::Break()))
}

fn continue_statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(span.close(tokens, StatementParseNode::Break()))
}

fn while_loop(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    let predicate = expression(tokens)?;
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut body = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        body.push(statement(tokens)?);
    }
    Ok(span.close(
        tokens,
        StatementParseNode::WhileLoop(WhileLoopParseNode { predicate, body }),
    ))
}

fn if_statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    let mut conditions = vec![if_condition(tokens)?];
    let mut else_branch = None;

    while tokens.accept(&KeywordToken::Else) {
        if tokens.accept(&KeywordToken::If) {
            conditions.push(if_condition(tokens)?);
        } else if tokens.accept(&OperatorToken::OpenBrace) {
            else_branch = Some(block(tokens)?);
        } else {
            return Err(());
        }
    }

    Ok(span.close(
        tokens,
        StatementParseNode::If(IfStatementParseNode {
            conditions,
            else_branch,
        }),
    ))
}

fn if_condition(tokens: &mut TokenTraverser) -> Result<IfStatementConditionParseNode, ()> {
    let predicate = expression(tokens)?;
    tokens.expect(&OperatorToken::OpenBrace)?;
    let body = block(tokens)?;
    Ok(IfStatementConditionParseNode { predicate, body })
}

fn block_statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    let block = ExpressionParseNode::Block(block(tokens)?);
    Ok(span.close(tokens, StatementParseNode::Expression(block)))
}

fn block_return(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    tokens.next();
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(span.close(tokens, StatementParseNode::BlockReturn(expression)))
}

fn expression_statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    let span = tokens.start_span();
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(span.close(tokens, StatementParseNode::Expression(expression.value)))
}
