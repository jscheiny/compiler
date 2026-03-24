use strum_macros::EnumIter;

use crate::{
    lexer::{Symbol, Token, TokenMatch},
    parser::{
        AccessExpressionNode, ClosureParameterExpressionNode, ExpressionNode,
        FunctionCallExpressionNode, LocatedSyntaxError, NameType, Node, Operator, ParseResult,
        SyntaxError, TokenStream, TypeAccessExpressionNode, TypeBindingExpressionNode,
        grammar::{bound_type_parameters, function_arguments, simple_closure, type_definition},
    },
};

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum SpecialOperator {
    Closure,          // param -> expr
    ClosureParameter, // value : type
    FunctionCall,     // fn ( params )
    MemberType,       // Receiver :: field
    MemberValue,      // receiver . field
    TypeBinding,      // T [ Args ]
}

impl SpecialOperator {
    pub fn parse(
        &self,
        tokens: &mut TokenStream,
        left: Node<ExpressionNode>,
    ) -> ParseResult<Node<ExpressionNode>> {
        match self {
            Self::Closure => simple_closure(tokens, left),
            Self::ClosureParameter => closure_parameter(tokens, left),
            Self::FunctionCall => function_call(tokens, left),
            Self::MemberType => member_type(tokens, left),
            Self::MemberValue => member_value(tokens, left),
            Self::TypeBinding => type_binding(tokens, left),
        }
    }
}

impl Operator for SpecialOperator {
    fn as_token(&self) -> crate::lexer::Token {
        use Symbol as S;
        match self {
            Self::Closure => Token::Symbol(S::SkinnyArrow),
            Self::ClosureParameter => Token::Symbol(S::Colon),
            Self::FunctionCall => Token::Symbol(S::OpenParen),
            Self::MemberType => Token::Symbol(S::DoubleColon),
            Self::MemberValue => Token::Symbol(S::Dot),
            Self::TypeBinding => Token::Symbol(S::OpenBracket),
        }
    }

    fn precedence(&self) -> i32 {
        match self {
            Self::MemberType | Self::TypeBinding => 10,
            Self::Closure | Self::FunctionCall | Self::MemberValue => 9,
            Self::ClosureParameter => 0,
        }
    }
}

fn closure_parameter(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
) -> ParseResult<Node<ExpressionNode>> {
    tokens.next();
    if let ExpressionNode::Name(name) = left.value {
        let parameter_type = Some(tokens.located(type_definition)?);
        let parameter_span = left.span.expand_to(tokens);
        return Ok(parameter_span.wrap(ExpressionNode::ClosureParameter(
            ClosureParameterExpressionNode {
                name,
                parameter_type,
            },
        )));
    } else {
        tokens.errors.push(LocatedSyntaxError {
            span: left.span,
            error: SyntaxError::ExpectedName(NameType::Parameter),
        });
    }

    // Parse the type definition for errors and so we can continue parsing
    type_definition(tokens)?;
    let parameter_span = left.span.expand_to(tokens);
    Ok(parameter_span.wrap(ExpressionNode::Error))
}

fn function_call(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
) -> ParseResult<Node<ExpressionNode>> {
    let arguments = tokens.located(function_arguments)?;
    let span = left.span.expand_to(tokens);

    let result = ExpressionNode::FunctionCall(FunctionCallExpressionNode {
        function: Box::new(left),
        arguments,
    });
    Ok(span.wrap(result))
}

fn member_type(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
) -> ParseResult<Node<ExpressionNode>> {
    tokens.next();
    let field = tokens.name(NameType::Type)?;
    let span = left.span.expand_to(tokens);
    let result = ExpressionNode::TypeAccess(TypeAccessExpressionNode {
        left: Box::new(left),
        field,
    });
    Ok(span.wrap(result))
}

fn member_value(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
) -> ParseResult<Node<ExpressionNode>> {
    tokens.next();
    let field = tokens.name(NameType::Field)?;
    let arguments = if Symbol::OpenParen.matches(tokens.peek()) {
        Some(tokens.located(function_arguments)?)
    } else {
        None
    };
    let span = left.span.expand_to(tokens);
    let result = ExpressionNode::Access(AccessExpressionNode {
        left: Box::new(left),
        field,
        arguments,
    });
    Ok(span.wrap(result))
}

fn type_binding(
    tokens: &mut TokenStream,
    left: Node<ExpressionNode>,
) -> ParseResult<Node<ExpressionNode>> {
    tokens.next();
    let bound_type_parameters = tokens.located(bound_type_parameters)?;
    let span = left.span.expand_to(tokens);

    let result = ExpressionNode::TypeBinding(TypeBindingExpressionNode {
        left: Box::new(left),
        bound_type_parameters,
    });
    Ok(span.wrap(result))
}
