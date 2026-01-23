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
    let record_span = tokens.start_span();
    let record_type = record_span.singleton(record_type);
    tokens.next();

    let identifier = tokens.identifier().ok_or(())?;

    let member_list = {
        let span = tokens.start_span();
        tokens.expect(&OperatorToken::OpenParen)?;
        let member_list = comma_separated_list(tokens, OperatorToken::CloseParen, member)?;
        span.close(tokens, member_list)
    };

    let methods_span = tokens.start_span();
    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(RecordDefinitionParseNode {
            record_type,
            identifier,
            member_list,
            methods: methods_span.close(tokens, vec![]),
        })
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let methods = {
            let methods = methods(tokens)?;
            methods_span.close(tokens, methods)
        };
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
