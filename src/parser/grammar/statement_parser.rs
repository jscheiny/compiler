use crate::{
    lexer::{KeywordToken, OperatorToken, Token, TokenMatch},
    parser::{
        DeclarationNode, ExpressionNode, IdentifierType, IfStatementConditionNode, IfStatementNode,
        Node, ParseResult, StatementNode, SyntaxError, TokenStream, WhileLoopNode,
        grammar::{block, expression, type_definition},
    },
};

pub fn statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
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
            OperatorToken::SkinnyArrow => block_return(tokens),
            _ => expression_statement(tokens),
        },
        _ => expression_statement(tokens),
    }
}

fn declaration(tokens: &mut TokenStream, mutable: bool) -> ParseResult<StatementNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Variable)?;
    let type_def = if tokens.accept(&OperatorToken::Colon) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };

    let initializer = initializer(tokens)?;
    end_statement(tokens);
    Ok(StatementNode::Declaration(DeclarationNode {
        mutable,
        identifier,
        type_def,
        initializer,
    }))
}

fn initializer(tokens: &mut TokenStream) -> ParseResult<Option<Node<ExpressionNode>>> {
    let error = SyntaxError::ExpectedInitializer;
    match tokens.peek() {
        Token::Operator(OperatorToken::Equal) => {
            tokens.next();
            Ok(Some(tokens.located(expression)?))
        }
        Token::Operator(OperatorToken::Semicolon) => {
            tokens.push_error(error);
            Ok(None)
        }
        _ => Err(tokens.make_error(error)),
    }
}

fn function_return(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    if tokens.accept(&OperatorToken::Semicolon) {
        Ok(StatementNode::FunctionReturn(None))
    } else {
        let expression = tokens.located(expression)?;
        end_statement(tokens);
        Ok(StatementNode::FunctionReturn(Some(expression)))
    }
}

fn break_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    end_statement(tokens);
    Ok(StatementNode::Break())
}

fn continue_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    end_statement(tokens);
    Ok(StatementNode::Break())
}

fn while_loop(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    let predicate = tokens.located(expression)?;
    let body = tokens.located(block)?;
    Ok(StatementNode::WhileLoop(WhileLoopNode { predicate, body }))
}

fn if_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let mut conditions = vec![tokens.located(if_condition)?];
    let mut else_branch = None;

    while tokens.accept(&KeywordToken::Else) {
        if KeywordToken::If.matches(tokens.peek()) {
            conditions.push(tokens.located(if_condition)?);
        } else {
            else_branch = Some(tokens.located(block)?);
        }
    }

    Ok(StatementNode::If(IfStatementNode {
        conditions,
        else_branch,
    }))
}

fn if_condition(tokens: &mut TokenStream) -> ParseResult<IfStatementConditionNode> {
    tokens.next();
    let predicate = tokens.located(expression)?;
    let body = tokens.located(block)?;
    Ok(IfStatementConditionNode { predicate, body })
}

fn block_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let block = ExpressionNode::Block(block(tokens)?);
    Ok(StatementNode::Expression(block))
}

fn block_return(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    Ok(StatementNode::BlockReturn(expression))
}

fn expression_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    Ok(StatementNode::Expression(expression.value))
}

pub fn end_statement(tokens: &mut TokenStream) {
    if !tokens.accept(&OperatorToken::Semicolon) {
        tokens.push_error(SyntaxError::ExpectedEndStatement);
    }
}
