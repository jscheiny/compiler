use crate::{
    lexer::{KeywordToken, OperatorToken},
    parser::{
        LocatedNode, MethodParseNode, RecordDefinitionParseNode, RecordMemberParseNode, RecordType,
        TokenTraverser,
        grammar::{comma_separated_list, nested_function, type_definition},
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
    tokens.next();

    let identifier = tokens.identifier()?;
    let member_list = tokens.located(member_list)?;

    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(RecordDefinitionParseNode {
            record_type,
            identifier,
            member_list,
            methods: None,
        })
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let methods = Some(tokens.located(methods)?);
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

fn member_list(tokens: &mut TokenTraverser) -> Result<Vec<LocatedNode<RecordMemberParseNode>>, ()> {
    tokens.expect(&OperatorToken::OpenParen)?;
    comma_separated_list(tokens, OperatorToken::CloseParen, member)
}

fn member(tokens: &mut TokenTraverser) -> Result<RecordMemberParseNode, ()> {
    if tokens.accept(&KeywordToken::Pub) {
        let member = member(tokens)?;
        return Ok(RecordMemberParseNode {
            public: true,
            ..member
        });
    }

    let identifier = tokens.identifier()?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = tokens.located(type_definition)?;
    Ok(RecordMemberParseNode {
        identifier,
        type_def,
        public: false,
    })
}

fn methods(tokens: &mut TokenTraverser) -> Result<Vec<LocatedNode<MethodParseNode>>, ()> {
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(tokens.located(method)?);
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

    let function = tokens.located(nested_function)?;
    Ok(MethodParseNode {
        public: false,
        function,
    })
}
