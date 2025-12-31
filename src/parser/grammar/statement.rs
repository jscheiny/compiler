use crate::{
    lexer::{KeywordToken, OperatorToken},
    parser::{
        DeclarationParseNode, StatementParseNode, TokenTraverser, WhileLoopParseNode,
        grammar::{expression, type_definition},
    },
};

pub fn statement(tokens: &mut TokenTraverser) -> Result<StatementParseNode, ()> {
    if tokens.accept(&KeywordToken::Let) {
        let declaration = declaration(tokens, false)?;
        Ok(StatementParseNode::Declaration(declaration))
    } else if tokens.accept(&KeywordToken::Mut) {
        let declaration = declaration(tokens, true)?;
        Ok(StatementParseNode::Declaration(declaration))
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let mut statements = vec![];
        while !tokens.accept(&OperatorToken::CloseBrace) {
            statements.push(statement(tokens)?);
        }
        Ok(StatementParseNode::Block(statements))
    } else if tokens.accept(&OperatorToken::FunctionDefinition) {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(StatementParseNode::Return(Some(expression)))
    } else if tokens.accept(&KeywordToken::Return) {
        if tokens.accept(&OperatorToken::EndStatement) {
            Ok(StatementParseNode::Return(None))
        } else {
            let expression = expression(tokens)?;
            tokens.expect(&OperatorToken::EndStatement)?;
            Ok(StatementParseNode::Return(Some(expression)))
        }
    } else if tokens.accept(&KeywordToken::Break) {
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(StatementParseNode::Break())
    } else if tokens.accept(&KeywordToken::Continue) {
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(StatementParseNode::Continue())
    } else if tokens.accept(&KeywordToken::While) {
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
    } else {
        Err(())
    }
}

fn declaration(tokens: &mut TokenTraverser, mutable: bool) -> Result<DeclarationParseNode, ()> {
    let identifier = tokens.identifier().ok_or(())?;
    let type_def = if tokens.accept(&OperatorToken::Type) {
        Some(type_definition(tokens)?)
    } else {
        None
    };

    tokens.expect(&OperatorToken::Assign)?;
    let expression = expression(tokens)?;
    tokens.expect(&OperatorToken::EndStatement)?;
    Ok(DeclarationParseNode {
        mutable,
        identifier,
        type_def,
        expression,
    })
}
