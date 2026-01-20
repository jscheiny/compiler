use crate::{
    lexer::{KeywordToken, OperatorToken, Token},
    parser::{
        DeclarationParseNode, StatementParseNode, TokenTraverser, WhileLoopParseNode,
        grammar::{expression, type_definition},
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
            _ => Err(()),
        },
        Token::Operator(operator) => match operator {
            OperatorToken::OpenBrace => block(tokens),
            OperatorToken::FunctionDefinition => expression_return(tokens),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn declaration(tokens: &mut TokenTraverser, mutable: bool) -> Result<StatementParseNode, ()> {
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
    Ok(StatementParseNode::Continue())
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

fn block(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    let mut statements = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        statements.push(statement(tokens)?);
    }
    Ok(StatementParseNode::Block(statements))
}

fn expression_return(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    tokens.next();
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(StatementParseNode::ExpressionReturn(expression))
}
