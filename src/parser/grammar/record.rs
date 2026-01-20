use crate::{
    lexer::{KeywordToken, OperatorToken},
    parser::{
        MethodBodyParseNode, MethodParseNode, RecordDefinitionParseNode, RecordMemberParseNode,
        RecordType, TokenTraverser,
        grammar::{comma_separated_list, expression, parameter, statement, type_definition},
    },
};

pub fn structure(tokens: &mut TokenTraverser) -> Result<RecordDefinitionParseNode, ()> {
    record(tokens, RecordType::Structure)
}

pub fn tuple(tokens: &mut TokenTraverser) -> Result<RecordDefinitionParseNode, ()> {
    record(tokens, RecordType::Tuple)
}

fn record(
    tokens: &mut TokenTraverser,
    record_type: RecordType,
) -> Result<RecordDefinitionParseNode, ()> {
    tokens.expect(match record_type {
        RecordType::Structure => &KeywordToken::Struct,
        RecordType::Tuple => &KeywordToken::Tuple,
    })?;
    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::OpenParen)?;
    let member_list = comma_separated_list(tokens, OperatorToken::CloseParen, member)?;

    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(RecordDefinitionParseNode {
            record_type,
            identifier,
            member_list,
            methods: vec![],
        })
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let methods = methods(tokens)?;
        Ok(RecordDefinitionParseNode {
            record_type,
            identifier,
            member_list,
            methods,
        })
    } else {
        Err(())
    }
}

fn member(tokens: &mut TokenTraverser) -> Result<RecordMemberParseNode, ()> {
    if tokens.accept(&KeywordToken::Pub) {
        let member = member(tokens)?;
        return Ok(RecordMemberParseNode {
            public: true,
            ..member
        });
    }

    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = type_definition(tokens)?;
    Ok(RecordMemberParseNode {
        identifier,
        type_def,
        public: false,
    })
}

fn methods(tokens: &mut TokenTraverser) -> Result<Vec<MethodParseNode>, ()> {
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(method(tokens)?);
    }
    Ok(methods)
}

fn method(tokens: &mut TokenTraverser) -> Result<MethodParseNode, ()> {
    if tokens.accept(&KeywordToken::Pub) {
        let method = method(tokens)?;
        return Ok(MethodParseNode {
            public: true,
            ..method
        });
    }

    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::OpenParen)?;
    let parameters = comma_separated_list(tokens, OperatorToken::CloseParen, parameter)?;

    let return_type = if tokens.accept(&OperatorToken::Type) {
        Some(type_definition(tokens)?)
    } else {
        None
    };

    let body: Result<MethodBodyParseNode, ()> = if tokens.accept(&OperatorToken::FunctionDefinition)
    {
        let expression = expression(tokens)?;
        tokens.expect(&OperatorToken::EndStatement)?;
        Ok(MethodBodyParseNode::Expression(expression))
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let mut statements = vec![];
        while !tokens.accept(&OperatorToken::CloseBrace) {
            statements.push(statement(tokens)?);
        }
        Ok(MethodBodyParseNode::Block(statements))
    } else {
        Err(())
    };

    Ok(MethodParseNode {
        public: false,
        identifier,
        parameters,
        return_type,
        body: body?,
    })
}
