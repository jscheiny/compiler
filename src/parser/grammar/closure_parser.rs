use crate::parser::{
    ClosureExpressionNode, ClosureParameterExpressionNode, ExpressionNode, LocatedSyntaxError,
    NameType, Node, ParseResult, SyntaxError, TokenStream, grammar::expression,
};

// A closure on a single untyped parameter
pub fn simple_closure(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
) -> ParseResult<Node<ExpressionNode>> {
    tokens.next();
    let parameter = match left.value {
        ExpressionNode::Name(name) => Ok(ExpressionNode::ClosureParameter(
            ClosureParameterExpressionNode {
                name,
                parameter_type: None,
            },
        )),
        _ => Err(LocatedSyntaxError {
            span: left.span,
            error: SyntaxError::ExpectedName(NameType::Parameter),
        }),
    }?;

    let parameters = vec![left.span.wrap(parameter)];
    let closure = closure(tokens, parameters)?;
    let full_span = left.span.expand_to(tokens);
    Ok(full_span.wrap(closure))
}

pub fn closure(
    tokens: &mut TokenStream,
    parameters: Vec<Node<ExpressionNode>>,
) -> ParseResult<ExpressionNode> {
    let parameters = parameters
        .into_iter()
        .map(|parameter| {
            if let ExpressionNode::Name(name) = parameter.value {
                Some(parameter.span.wrap(ClosureParameterExpressionNode {
                    name,
                    parameter_type: None,
                }))
            } else if let ExpressionNode::ClosureParameter(param) = parameter.value {
                Some(parameter.span.wrap(param))
            } else {
                tokens.errors.push(LocatedSyntaxError {
                    span: parameter.span,
                    error: SyntaxError::ExpectedClosureParameter,
                });
                None
            }
        })
        .collect();

    let body = tokens.located(expression)?;
    Ok(ExpressionNode::Closure(ClosureExpressionNode {
        parameters,
        body: Box::new(body),
    }))
}
