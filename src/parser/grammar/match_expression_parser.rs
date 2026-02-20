use crate::{
    lexer::{Symbol, TokenMatch},
    parser::{
        ExpressionNode, IdentifierType, MatchCaseNode, MatchExpressionNode, MatchPatternNode, Node,
        ParseResult, SyntaxError, TokenStream,
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

    Ok(ExpressionNode::Match(MatchExpressionNode {
        subject: Box::new(subject),
        cases,
    }))
}

fn match_case(tokens: &mut TokenStream) -> ParseResult<MatchCaseNode> {
    let patterns = match_patterns(tokens)?;
    let expect_semicolon = !Symbol::OpenBrace.matches(tokens.peek());
    let if_match = tokens.located(expression)?;
    if expect_semicolon {
        end_statement(tokens);
    }
    Ok(MatchCaseNode { patterns, if_match })
}

fn match_patterns(tokens: &mut TokenStream) -> ParseResult<Vec<Node<MatchPatternNode>>> {
    let first_pattern = tokens.located(match_pattern)?;
    let mut patterns = vec![first_pattern];

    while !tokens.accept(&Symbol::SkinnyArrow) {
        tokens.expect(&Symbol::Comma, SyntaxError::ExpectedMatchExpression)?;
        patterns.push(tokens.located(match_pattern)?);
    }

    Ok(patterns)
}

fn match_pattern(tokens: &mut TokenStream) -> ParseResult<MatchPatternNode> {
    // TODO match patterns should be much more capable...
    let identifier = tokens.identifier(IdentifierType::Variant)?;
    Ok(MatchPatternNode { identifier })
}
