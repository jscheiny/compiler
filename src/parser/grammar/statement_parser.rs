use crate::{
    lexer::{Keyword, Symbol, Token, TokenMatch},
    parser::{
        DeclarationNode, ExpressionNode, IfStatementConditionNode, IfStatementNode, NameType, Node,
        ParseResult, StatementNode, StatementType, SyntaxError, TokenStream, WhileLoopNode,
        grammar::{BlockType, block, expression, match_statement, type_alias, type_definition},
    },
};

pub fn statement(tokens: &mut TokenStream, block_type: BlockType) -> ParseResult<StatementNode> {
    use Keyword as K;
    use Symbol as S;
    match tokens.peek() {
        Token::Keyword(K::Let) => declaration(tokens, false),
        Token::Keyword(K::Mut) => declaration(tokens, true),
        Token::Keyword(K::Return) => function_return(tokens),
        Token::Keyword(K::Break) => break_statement(tokens),
        Token::Keyword(K::Continue) => continue_statement(tokens),
        Token::Keyword(K::While) => while_loop(tokens),
        Token::Keyword(K::If) => if_statement(tokens),
        Token::Keyword(K::Match) => match_statement(tokens),
        Token::Keyword(K::Type) => type_alias_statement(tokens),
        Token::Symbol(S::OpenBrace) => block_statement(tokens),
        Token::Symbol(S::SkinnyArrow) => block_return(tokens, block_type),
        _ => expression_statement(tokens),
    }
}

fn declaration(tokens: &mut TokenStream, mutable: bool) -> ParseResult<StatementNode> {
    tokens.next();
    let name = tokens.name(NameType::Variable)?;
    let type_def = if tokens.accept(Symbol::Colon) {
        Some(tokens.located(type_definition)?)
    } else {
        None
    };

    let initializer = initializer(tokens)?;
    end_statement(tokens);
    Ok(StatementNode::Declaration(DeclarationNode {
        mutable,
        name,
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
    if tokens.accept(Symbol::Semicolon) {
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

    while tokens.accept(Keyword::Else) {
        if Keyword::If.matches(tokens.peek()) {
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
    let block = ExpressionNode::Block(block(tokens, BlockType::Statement(StatementType::Block))?);
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

fn type_alias_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let type_alias = type_alias(tokens)?;
    Ok(StatementNode::TypeAlias(type_alias))
}

fn expression_statement(tokens: &mut TokenStream) -> ParseResult<StatementNode> {
    let expression = tokens.located(expression)?;
    end_statement(tokens);
    Ok(StatementNode::Expression(expression.value))
}

pub fn end_statement(tokens: &mut TokenStream) {
    if !tokens.accept(Symbol::Semicolon) {
        tokens.push_error(SyntaxError::ExpectedEndStatement);
    }
}
