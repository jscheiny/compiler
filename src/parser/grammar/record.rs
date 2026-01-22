use crate::{
    lexer::{KeywordToken, OperatorToken},
    parser::{
        LocatedNode, MethodParseNode, ParseResult, RecordDefinitionParseNode,
        RecordMemberParseNode, RecordType, TokenTraverser,
        grammar::{comma_separated_list, function, type_definition},
    },
};

pub fn structure(tokens: &mut TokenTraverser) -> ParseResult<RecordDefinitionParseNode> {
    record(tokens, RecordType::Structure)
}

pub fn tuple(tokens: &mut TokenTraverser) -> ParseResult<RecordDefinitionParseNode> {
    record(tokens, RecordType::Tuple)
}

fn record(
    tokens: &mut TokenTraverser,
    record_type: RecordType,
) -> ParseResult<RecordDefinitionParseNode> {
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
        Ok(record_span.close(
            tokens,
            RecordDefinitionParseNode {
                record_type,
                identifier,
                member_list,
                methods: methods_span.close(tokens, vec![]),
            },
        ))
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let methods = {
            let methods = methods(tokens)?;
            methods_span.close(tokens, methods)
        };
        Ok(record_span.close(
            tokens,
            RecordDefinitionParseNode {
                record_type,
                identifier,
                member_list,
                methods,
            },
        ))
    } else {
        Err(())
    }
}

fn member(tokens: &mut TokenTraverser) -> ParseResult<RecordMemberParseNode> {
    let span = tokens.start_span();
    if tokens.accept(&KeywordToken::Pub) {
        let member = member(tokens)?;
        return Ok(span.close(
            tokens,
            RecordMemberParseNode {
                public: true,
                ..member.value
            },
        ));
    }

    let identifier = tokens.identifier().ok_or(())?;
    tokens.expect(&OperatorToken::Type)?;
    let type_def = type_definition(tokens)?;
    Ok(span.close(
        tokens,
        RecordMemberParseNode {
            identifier,
            type_def,
            public: false,
        },
    ))
}

fn methods(tokens: &mut TokenTraverser) -> Result<Vec<LocatedNode<MethodParseNode>>, ()> {
    let mut methods = vec![];
    while !tokens.accept(&OperatorToken::CloseBrace) {
        methods.push(method(tokens)?);
    }
    Ok(methods)
}

fn method(tokens: &mut TokenTraverser) -> ParseResult<MethodParseNode> {
    let span = tokens.start_span();
    if tokens.accept(&KeywordToken::Pub) {
        let method = method(tokens)?;
        return Ok(span.close(
            tokens,
            MethodParseNode {
                public: true,
                ..method.value
            },
        ));
    }

    let function = function(tokens, false)?;
    Ok(span.close(
        tokens,
        MethodParseNode {
            public: false,
            function,
        },
    ))
}
