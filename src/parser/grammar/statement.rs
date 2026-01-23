use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        DeclarationParseNode, ExpressionParseNode, IfStatementConditionParseNode,
        IfStatementParseNode, ParseResult, ParserPredicate, StatementParseNode, TokenTraverser,
        WhileLoopParseNode,
        grammar::{block, expression, type_definition},
    },
};

pub fn statement(tokens: &mut TokenTraverser) -> ParseResult<StatementParseNode> {
    match tokens.peek() {
        Token::Keyword(keyword) => match keyword {
            KeywordToken::Let => declaration(tokens, false),
            KeywordToken::Mut => declaration(tokens, true),
            KeywordToken::Return => tokens.located(function_return),
            KeywordToken::Break => tokens.located(break_statement),
            KeywordToken::Continue => tokens.located(continue_statement),
            KeywordToken::While => tokens.located(while_loop),
            KeywordToken::If => tokens.located(if_statement),
            _ => tokens.located(expression_statement),
        },
        Token::Operator(operator) => match operator {
            OperatorToken::OpenBrace => tokens.located(block_statement),
            OperatorToken::FunctionDefinition => tokens.located(block_return),
            _ => tokens.located(expression_statement),
        },
        _ => tokens.located(expression_statement),
    }
}

// todo refactor this
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

fn function_return(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(StatementParseNode::FunctionReturn(None))
    } else {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(StatementParseNode::FunctionReturn(Some(expression)))
    }
}

fn break_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::Break())
}

fn continue_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::Break())
}

fn while_loop(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    let predicate = expression(tokens)?;
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut body = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        body.push(statement(tokens)?);
    }
    Ok(StatementParseNode::WhileLoop(WhileLoopParseNode {
        predicate,
        body,
    }))
}

fn if_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    let mut conditions = vec![tokens.located(if_condition)?];
    let mut else_branch = None;

    while tokens.accept(&KeywordToken::Else) {
        if KeywordToken::If.is_match(tokens.peek()) {
            conditions.push(tokens.located(if_condition)?);
        } else {
            else_branch = Some(block(tokens)?);
        }
    }

    Ok(StatementParseNode::If(IfStatementParseNode {
        conditions,
        else_branch,
    }))
}

fn if_condition(tokens: &mut TokenTraverser) -> Result<IfStatementConditionParseNode, ()> {
    tokens.next();
    let predicate = expression(tokens)?;
    let body = block(tokens)?;
    Ok(IfStatementConditionParseNode { predicate, body })
}

fn block_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    let block = ExpressionParseNode::Block(block(tokens)?);
    Ok(StatementParseNode::Expression(block))
}

fn block_return(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::BlockReturn(expression))
}

fn expression_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::Expression(expression.value))
}
