use crate::{
    lexer::{KeywordToken, Symbol, Token, TokenMatch},
    parser::{
        DeclarationNode, ExpressionNode, IdentifierType, IfStatementConditionNode, IfStatementNode,
        Node, ParseResult, StatementNode, StatementType, SyntaxError, TokenStream, WhileLoopNode,
        grammar::{BlockType, block, expression, type_definition},
    },
};

pub fn statement(tokens: &mut TokenStream, block_type: BlockType) -> ParseResult<StatementNode> {
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
        Token::Symbol(operator) => match operator {
            Symbol::OpenBrace => block_statement(tokens),
            Symbol::SkinnyArrow => block_return(tokens, block_type),
            _ => expression_statement(tokens),
        },
        _ => expression_statement(tokens),
    }
}

fn declaration(tokens: &mut TokenStream, mutable: bool) -> ParseResult<StatementNode> {
    tokens.next();
    let identifier = tokens.identifier(IdentifierType::Variable)?;
    let type_def = if tokens.accept(&Symbol::Colon) {
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
        Token::Symbol(Symbol::Equal) => {
            tokens.next();
            Ok(Some(tokens.located(expression)?))
        }
        Token::Symbol(Symbol::Semicolon) => {
            tokens.push_error(error);
            Ok(None)
        }
        _ => Err(tokens.make_error(error)),
    }
}

fn function_return(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    if tokens.accept(&Symbol::Semicolon) {
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
    Ok(StatementNode::Break)
}

fn continue_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    end_statement(tokens);
    Ok(StatementNode::Continue)
}

fn while_loop(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    tokens.next();
    let predicate = tokens.located(expression)?;
    let block_type = BlockType::Statement(StatementType::WhileLoop);
    let body = tokens.located_with(block, block_type)?;
    Ok(StatementNode::WhileLoop(WhileLoopNode { predicate, body }))
}

fn if_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let mut conditions = vec![tokens.located(if_condition)?];
    let mut else_branch = None;

    while tokens.accept(&KeywordToken::Else) {
        if KeywordToken::If.matches(tokens.peek()) {
            conditions.push(tokens.located(if_condition)?);
        } else {
            let block_type = BlockType::Statement(StatementType::If);
            else_branch = Some(tokens.located_with(block, block_type)?);
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
    let block_type = BlockType::Statement(StatementType::If);
    let body = tokens.located_with(block, block_type)?;
    Ok(IfStatementConditionNode { predicate, body })
}

fn block_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    // TODO is this an expression or a statement?? Maybe add a warning for block returns in this case?
    let block = ExpressionNode::Block(block(tokens, BlockType::Expression)?);
    Ok(StatementNode::Expression(block))
}

fn block_return(tokens: &mut TokenStream, block_type: BlockType) -> ParseResult<StatementNode> {
    if let BlockType::Statement(statement_type) = block_type {
        tokens.push_error(SyntaxError::UnexpectedBlockReturn(statement_type));
    }

    tokens.next();
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    match block_type {
        BlockType::Expression => Ok(StatementNode::BlockReturn(expression)),
        BlockType::Statement(_) => Ok(StatementNode::Expression(expression.value)),
    }
}

fn expression_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    Ok(StatementNode::Expression(expression.value))
}

pub fn end_statement(tokens: &mut TokenStream) {
    if !tokens.accept(&Symbol::Semicolon) {
        tokens.push_error(SyntaxError::ExpectedEndStatement);
    }
}
