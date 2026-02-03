use crate::{
    lexer::{KeywordToken, OperatorToken, Token, TokenMatch},
    parser::{
        DeclarationParseNode, ExpressionParseNode, IfStatementConditionParseNode,
        IfStatementParseNode, ParseNode, ParseResult, StatementParseNode, SyntaxError, TokenStream,
        WhileLoopParseNode,
        grammar::{block, expression, identifier, type_definition},
    },
};

pub fn statement(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
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

fn declaration(tokens: &mut TokenStream, mutable: bool) -> ParseResult<StatementParseNode> {
    tokens.next();
    let identifier = tokens.located(identifier)?;
    let type_def = if tokens.accept(&OperatorToken::Type) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };

    let initializer = initializer(tokens)?;
    end_statement(tokens);
    Ok(StatementParseNode::Declaration(DeclarationParseNode {
        mutable,
        identifier,
        type_def,
        initializer,
    }))
}

fn initializer(tokens: &mut TokenStream) -> ParseResult<Option<ParseNode<ExpressionParseNode>>> {
    let error = SyntaxError::ExpectedInitializer;
    match tokens.peek() {
        Token::Operator(OperatorToken::Assign) => {
            tokens.next();
            Ok(Some(tokens.located(expression)?))
        }
        Token::Operator(OperatorToken::EndStatement) => {
            tokens.push_error(error);
            Ok(None)
        }
        _ => Err(tokens.make_error(error)),
    }
}

fn function_return(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    tokens.next();
    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(StatementParseNode::FunctionReturn(None))
    } else {
        let expression = tokens.located(expression)?;
        end_statement(tokens);
        Ok(StatementParseNode::FunctionReturn(Some(expression)))
    }
}

fn break_statement(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    tokens.next();
    end_statement(tokens);
    Ok(StatementParseNode::Break())
}

fn continue_statement(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    tokens.next();
    end_statement(tokens);
    Ok(StatementParseNode::Break())
}

fn while_loop(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    tokens.next();
    let predicate = tokens.located(expression)?;
    let body = tokens.located(block)?;
    Ok(StatementParseNode::WhileLoop(WhileLoopParseNode {
        predicate,
        body,
    }))
}

fn if_statement(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    let mut conditions = vec![tokens.located(if_condition)?];
    let mut else_branch = None;

    while tokens.accept(&KeywordToken::Else) {
        if KeywordToken::If.matches(tokens.peek()) {
            conditions.push(tokens.located(if_condition)?);
        } else {
            else_branch = Some(tokens.located(block)?);
        }
    }

    Ok(StatementParseNode::If(IfStatementParseNode {
        conditions,
        else_branch,
    }))
}

fn if_condition(tokens: &mut TokenStream) -> ParseResult<IfStatementConditionParseNode> {
    tokens.next();
    let predicate = tokens.located(expression)?;
    let body = tokens.located(block)?;
    Ok(IfStatementConditionParseNode { predicate, body })
}

fn block_statement(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    let block = ExpressionParseNode::Block(block(tokens)?);
    Ok(StatementParseNode::Expression(block))
}

fn block_return(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    tokens.next();
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    Ok(StatementParseNode::BlockReturn(expression))
}

fn expression_statement(tokens: &mut TokenStream) -> ParseResult<StatementParseNode> {
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    Ok(StatementParseNode::Expression(expression.value))
}

pub fn end_statement(tokens: &mut TokenStream) {
    if !tokens.accept(&OperatorToken::EndStatement) {
        tokens.push_error(SyntaxError::ExpectedEndStatement);
    }
}
