use crate::{
    lexer::{Keyword, Symbol, Token, TokenMatch},
    parser::{
        ExpressionNode, IdentifierNode, IdentifierType, MatchCaseNode, MatchNode, MatchPatternNode,
        ParseResult, SyntaxError, TokenStream, VariantMatchPattern,
        grammar::{expression_parser::expression, statement_parser::end_statement},
    },
};

pub fn match_expression(tokens: &mut TokenStream) -> ParseResult<ExpressionNode> {
    tokens.next();

    let subject = tokens.located(expression)?;
    tokens.expect(&Symbol::OpenBrace, SyntaxError::ExpectedMatchBlock)?;
    let mut cases = vec![];
    while !tokens.accept(&Symbol::CloseBrace) {
        cases.push(tokens.located(match_case)?);
    }

    Ok(ExpressionNode::Match(MatchNode {
        subject: Box::new(subject),
        cases,
    }))
}

fn match_case(tokens: &mut TokenStream) -> ParseResult<MatchCaseNode> {
    let pattern = if Keyword::Else.matches(tokens.peek()) {
        let pattern = tokens.current_span().wrap(MatchPatternNode::Else);
        tokens.next();
        pattern
    } else {
        tokens.located_with(match_pattern, true)?
    };
    tokens.expect(&Symbol::SkinnyArrow, SyntaxError::ExpectedMatchExpression)?;
    let expect_semicolon = !Symbol::OpenBrace.matches(tokens.peek());
    let if_match = tokens.located(expression)?;
    if expect_semicolon {
        end_statement(tokens);
    }
    Ok(MatchCaseNode { pattern, if_match })
}

// fn match_patterns(tokens: &mut TokenStream) -> ParseResult<Vec<Node<MatchPatternNode>>> {
//     let first_pattern = tokens.located(match_pattern)?;
//     let mut patterns = vec![first_pattern];

//     while !tokens.accept(&Symbol::SkinnyArrow) {
//         tokens.expect(&Symbol::Comma, SyntaxError::ExpectedMatchExpression)?;
//         patterns.push(tokens.located(match_pattern)?);
//     }

//     Ok(patterns)
// }

fn match_pattern(tokens: &mut TokenStream, top_level: bool) -> ParseResult<MatchPatternNode> {
    match tokens.peek() {
        Token::Identifier(identifier) => {
            let identifier = tokens
                .current_span()
                .wrap(IdentifierNode(identifier.clone()));
            tokens.next();
            // TODO accept / expect don't need to take references these are always copyable
            if tokens.accept(&Symbol::OpenParen) {
                let inner_pattern = tokens.located_with(match_pattern, false)?;
                tokens.expect(&Symbol::CloseParen, SyntaxError::ExpectedCloseParen)?;
                Ok(MatchPatternNode::Variant(VariantMatchPattern {
                    identifier,
                    inner_pattern: Some(Box::new(inner_pattern)),
                }))
            } else {
                Ok(MatchPatternNode::Variant(VariantMatchPattern {
                    identifier,
                    inner_pattern: None,
                }))
            }
        }
        Token::Keyword(Keyword::Let) => {
            if top_level {
                tokens.push_error(SyntaxError::UnexpectedBindingPattern);
            }
            tokens.next();
            let identifier = tokens.identifier(IdentifierType::PatternBinding)?;
            Ok(MatchPatternNode::Binding(identifier.value))
        }
        _ => Err(tokens.make_error(SyntaxError::ExpectedMatchPattern)),
    }
}
