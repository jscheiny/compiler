use strum_macros::EnumIter;

use crate::{
    lexer::{Symbol, Token},
    parser::{
        BinaryOperator, ClosureParameterExpressionNode, ExpressionNode, FunctionCallExpressionNode,
        LocatedSyntaxError, NameType, Node, Operator, ParseResult, SyntaxError, TokenStream,
        TypeBindingExpressionNode,
        grammar::{bound_type_parameters, function_arguments, simple_closure, type_definition},
    },
};

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum SpecialOperator {
    Closure,          // params -> expr
    ClosureParameter, // value : type
    FunctionCall,     // fn ( params )
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
            Self::TypeBinding => type_binding(tokens, left),
        }
    }

    fn get_equivalent_precedence_operator(&self) -> BinaryOperator {
        use BinaryOperator as B;
        match self {
            Self::Closure => B::Access,
            Self::ClosureParameter => B::Comma,
            Self::FunctionCall => B::Access,
            Self::TypeBinding => B::TypeAccess,
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
            Self::TypeBinding => Token::Symbol(S::OpenBracket),
        }
    }

    fn precedence(&self) -> i32 {
        self.get_equivalent_precedence_operator().precedence()
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
