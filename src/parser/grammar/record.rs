use crate::{
    lexer::{KeywordToken, OperatorToken},
    parser::{
        MethodParseNode, ParseResult, RecordDefinitionParseNode, RecordMemberParseNode, RecordType,
        TokenTraverser,
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

    let identifier = {
        let span = tokens.start_span();
        let identifier = tokens.identifier().ok_or(())?;
        span.singleton(identifier)
    };

    let member_list = {
        let span = tokens.start_span();
        tokens.expect(&OperatorToken::OpenParen)?;
        let member_list = comma_separated_list(tokens, OperatorToken::CloseParen, member)?;
        span.close(tokens, member_list)
    };

    if tokens.accept(&OperatorToken::EndStatement) {
        Ok(record_span.close(
            tokens,
            RecordDefinitionParseNode {
                record_type,
                identifier,
                member_list,
                methods: vec![],
            },
        ))
    } else if tokens.accept(&OperatorToken::OpenBrace) {
        let methods = methods(tokens)?;
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

    let function = function(tokens)?;
    Ok(MethodParseNode {
        public: false,
        function,
    })
}
