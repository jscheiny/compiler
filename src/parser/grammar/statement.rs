use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        DeclarationParseNode, ExpressionParseNode, IfStatementConditionParseNode,
        IfStatementParseNode, ParserPredicate, StatementParseNode, TokenTraverser,
        WhileLoopParseNode,
        grammar::{block, expression, type_definition},
    },
};

pub fn statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
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

fn declaration(tokens: &mut TokenTraverser, mutable: bool) -> Result<StatementParseNode, ()> {
    tokens.next();
    let identifier = tokens.identifier().ok_or(())?;
    let type_def = if tokens.accept(&OperatorToken::Type) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };

    tokens.expect(&OperatorToken::Assign)?;
    let expression = tokens.located(expression)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::Declaration(DeclarationParseNode {
        mutable,
        identifier,
        type_def,
        expression,
    }))
}

fn function_return(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(StatementParseNode::FunctionReturn(None))
    } else {
        let expression = tokens.located(expression)?;
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
    let predicate = tokens.located(expression)?;
    tokens.expect(&OperatorToken::OpenBrace)?;
    let mut body = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        body.push(tokens.located(statement)?);
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
    let predicate = tokens.located(expression)?;
    let body = block(tokens)?;
    Ok(IfStatementConditionParseNode { predicate, body })
}

fn block_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    let block = ExpressionParseNode::Block(block(tokens)?);
    Ok(StatementParseNode::Expression(block))
}

fn block_return(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    let expression = tokens.located(expression)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::BlockReturn(expression))
}

fn expression_statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    let expression = tokens.located(expression)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::Expression(expression.value))
}
