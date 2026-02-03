use std::{fmt::Display, rc::Rc};

use crate::{
    lexer::{LocatedToken, OperatorToken, Token},
    parser::TokenSpan,
};

#[derive(Clone, Copy)]
pub enum SyntaxError {
    ExpectedBlock,
    ExpectedCloseParen,
    ExpectedEndStatement,
    ExpectedExpression,
    ExpectedFunctionBody,
    ExpectedIdentifier(IdentifierType),
    ExpectedInitializer,
    ExpectedMembers,
    ExpectedMethods,
    ExpectedMethodSignatures,
    ExpectedParameters,
    ExpectedReturnType,
    ExpectedTopLevelDefinition,
    ExpectedType,
    ExpectedVariants,
}

#[derive(Clone, Copy)]
pub enum IdentifierType {
    Function,
    Interface,
    Method,
    Member,
    Parameter,
    Struct,
    Tuple,
    Type,
    Variable,
    Variant,
}

pub struct LocatedSyntaxError {
    pub span: TokenSpan,
    pub error: SyntaxError,
}

impl LocatedSyntaxError {
    pub fn message(&self, tokens: Rc<Vec<LocatedToken>>) -> SyntaxErrorMessage<'_> {
        SyntaxErrorMessage {
            error: self,
            tokens,
        }
    }

    pub fn inline_message(&self) -> SyntaxErrorInlineMessage<'_> {
        SyntaxErrorInlineMessage { error: self }
    }
}

pub struct SyntaxErrorMessage<'a> {
    error: &'a LocatedSyntaxError,
    tokens: Rc<Vec<LocatedToken>>,
}

impl<'a> Display for SyntaxErrorMessage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected ")?;
        use SyntaxError as E;
        let message = match self.error.error {
            E::ExpectedBlock => "statement block",
            E::ExpectedCloseParen => "close parenthesis",
            E::ExpectedEndStatement => "end of statement",
            E::ExpectedExpression => "expression",
            E::ExpectedFunctionBody => "function body",
            E::ExpectedIdentifier(_) => "identifier", // TODO switch on type
            E::ExpectedInitializer => "initializer",
            E::ExpectedMembers => "member variables",
            E::ExpectedMethods => "methods block",
            E::ExpectedMethodSignatures => "method signatures block",
            E::ExpectedParameters => "parameters",
            E::ExpectedReturnType => "return type",
            E::ExpectedTopLevelDefinition => "struct, tuple, enum, or function",
            E::ExpectedType => "type name",
            E::ExpectedVariants => "enum variants",
        };
        write!(f, "{}, found ", message)?;

        let LocatedToken { token, .. } = &self.tokens[self.error.span.start_index];
        use Token as T;
        match token {
            T::Identifier(identifier) => write!(f, "identifier `{}`", identifier),
            T::IntegerLiteral(literal) => write!(f, "integer literal `{}`", literal),
            T::StringLiteral(literal) => write!(f, "string literal {}", literal),
            T::Operator(operator) => write!(f, "`{}`", operator),
            T::Keyword(keyword) => write!(f, "keyword: `{}`", keyword),
            T::EndOfFile => write!(f, "end of file"),
        }
    }
}

pub struct SyntaxErrorInlineMessage<'a> {
    error: &'a LocatedSyntaxError,
}

impl<'a> Display for SyntaxErrorInlineMessage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected ")?;
        use OperatorToken as O;
        use SyntaxError as E;
        match self.error.error {
            E::ExpectedBlock => fmt_op(f, O::OpenParen),
            E::ExpectedCloseParen => fmt_op(f, O::CloseParen),
            E::ExpectedEndStatement => fmt_op(f, O::EndStatement),
            E::ExpectedExpression => write!(f, "expression"),
            E::ExpectedFunctionBody => fmt_ops(f, O::FunctionDefinition, O::OpenBrace),
            E::ExpectedIdentifier(_) => write!(f, "identifier"), // TODO switch on type
            E::ExpectedInitializer => fmt_op(f, O::Assign),
            E::ExpectedMembers => fmt_op(f, O::OpenParen),
            E::ExpectedMethods => fmt_ops(f, O::OpenBrace, O::EndStatement),
            E::ExpectedMethodSignatures => fmt_op(f, O::OpenBrace),
            E::ExpectedParameters => fmt_op(f, O::OpenParen),
            E::ExpectedReturnType => fmt_op(f, O::Type),
            E::ExpectedTopLevelDefinition => write!(f, "struct, tuple, enum, or function"),
            E::ExpectedType => write!(f, "type name"),
            E::ExpectedVariants => fmt_op(f, O::OpenParen),
        }
    }
}

fn fmt_op(f: &mut std::fmt::Formatter<'_>, operator: OperatorToken) -> std::fmt::Result {
    write!(f, "`{}`", operator)
}

fn fmt_ops(
    f: &mut std::fmt::Formatter<'_>,
    op1: OperatorToken,
    op2: OperatorToken,
) -> std::fmt::Result {
    write!(f, "`{}` or `{}`", op1, op2)
}
