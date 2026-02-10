use std::{fmt::Display, rc::Rc};

use crate::{
    lexer::{KeywordToken, LocatedToken, OperatorToken, Token},
    parser::TokenSpan,
};

#[derive(Clone, Copy)]
pub enum SyntaxError {
    ExpectedBlock,
    ExpectedCloseParen,
    ExpectedElse,
    ExpectedEndStatement,
    ExpectedExpression,
    ExpectedFields,
    ExpectedFunctionBody,
    ExpectedIdentifier(IdentifierType),
    ExpectedInitializer,
    ExpectedMethods,
    ExpectedParameters,
    ExpectedReturnType,
    ExpectedThen,
    ExpectedTopLevelDefinition,
    ExpectedType,
    ExpectedVariants,
}

#[derive(Clone, Copy)]
pub enum IdentifierType {
    Field,
    Function,
    Method,
    Parameter,
    Struct,
    Type,
    TypeAlias,
    Variable,
    Variant,
}

impl Display for IdentifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Field => "field",
            Self::Function => "function",
            Self::Method => "method",
            Self::Parameter => "parameter",
            Self::Struct => "struct",
            Self::Type => "type",
            Self::TypeAlias => "type alias",
            Self::Variable => "variable",
            Self::Variant => "variant",
        };
        write!(f, "{} name", message)
    }
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
        use SyntaxError as E;
        write!(f, "expected ")?;
        match self.error.error {
            E::ExpectedBlock => write!(f, "statement block"),
            E::ExpectedCloseParen => write!(f, "close parenthesis"),
            E::ExpectedElse => write!(
                f,
                "`{}` following true branch expression",
                KeywordToken::Else
            ),
            E::ExpectedEndStatement => write!(f, "end of statement"),
            E::ExpectedExpression => write!(f, "expression"),
            E::ExpectedFields => write!(f, "fields"),
            E::ExpectedFunctionBody => write!(f, "function body"),
            E::ExpectedIdentifier(id_type) => write!(f, "{}", id_type),
            E::ExpectedInitializer => write!(f, "initializer"),
            E::ExpectedMethods => write!(f, "methods block"),
            E::ExpectedParameters => write!(f, "parameters"),
            E::ExpectedReturnType => write!(f, "return type"),
            E::ExpectedThen => write!(f, "`{}` following predicate`", KeywordToken::Then),
            E::ExpectedTopLevelDefinition => write!(f, "struct, tuple, enum, or function"),
            E::ExpectedType => write!(f, "type name"),
            E::ExpectedVariants => write!(f, "enum variants"),
        }?;
        write!(f, ", found ")?;

        let LocatedToken { token, .. } = &self.tokens[self.error.span.start_index];
        use Token as T;
        match token {
            T::Identifier(identifier) => write!(f, "identifier `{}`", identifier),
            T::IntegerLiteral(literal) => write!(f, "integer literal `{}`", literal),
            T::StringLiteral(literal) => write!(f, "string literal {}", literal),
            T::Operator(operator) => write!(f, "`{}`", operator),
            T::Keyword(keyword) => write!(f, "keyword `{}`", keyword),
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
            E::ExpectedElse => write!(f, "`{}`", KeywordToken::Else),
            E::ExpectedEndStatement => fmt_op(f, O::Semicolon),
            E::ExpectedExpression => write!(f, "expression"),
            E::ExpectedFunctionBody => fmt_ops(f, O::SkinnyArrow, O::OpenBrace),
            E::ExpectedIdentifier(id_type) => write!(f, "{}", id_type),
            E::ExpectedInitializer => fmt_op(f, O::Equal),
            E::ExpectedFields => fmt_op(f, O::OpenParen),
            E::ExpectedMethods => fmt_ops(f, O::OpenBrace, O::Semicolon),
            E::ExpectedParameters => fmt_op(f, O::OpenParen),
            E::ExpectedReturnType => fmt_op(f, O::Colon),
            E::ExpectedThen => write!(f, "`{}`", KeywordToken::Then),
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
