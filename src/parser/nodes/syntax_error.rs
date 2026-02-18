use std::{fmt::Display, rc::Rc};

use crate::{
    lexer::{KeywordToken, LocatedToken, OperatorToken, Token},
    parser::TokenSpan,
};

#[derive(Clone, Copy)]
pub enum SyntaxError {
    BlockReturnEarly,
    ExpectedBlock,
    ExpectedCloseBracket,
    ExpectedCloseParen,
    ExpectedClosureParameter,
    ExpectedElse,
    ExpectedEndStatement,
    ExpectedExpression,
    ExpectedFields,
    ExpectedFunctionBody,
    ExpectedIdentifier(IdentifierType),
    ExpectedInitializer,
    ExpectedMethods,
    ExpectedParameters,
    ExpectedThen,
    ExpectedTopLevelDefinition,
    ExpectedType,
    ExpectedVariants,
    UnexpectedBlockReturn(StatementType),
    UnexpectedTypeExpression,
}

#[derive(Clone, Copy)]
pub enum IdentifierType {
    Field,
    Function,
    Method,
    Parameter,
    Struct,
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
            Self::Variable => "variable",
            Self::Variant => "variant",
        };
        write!(f, "{} name", message)
    }
}

#[derive(Clone, Copy)]
pub enum StatementType {
    If,
    WhileLoop,
}

impl Display for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::If => write!(f, "if statement"),
            Self::WhileLoop => write!(f, "while loop"),
        }
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
        match self.error.error {
            E::BlockReturnEarly => return write!(f, "early block return statement"),
            E::ExpectedBlock => write!(f, "expected statement block"),
            E::ExpectedCloseBracket => write!(f, "expected close bracket"),
            E::ExpectedCloseParen => write!(f, "expected close parenthesis"),
            E::ExpectedClosureParameter => write!(f, "expected parameter"),
            E::ExpectedElse => write!(
                f,
                "`{}` following true branch expression",
                KeywordToken::Else
            ),
            E::ExpectedEndStatement => write!(f, "expected end of statement"),
            E::ExpectedExpression => write!(f, "expected expression"),
            E::ExpectedFields => write!(f, "expected fields"),
            E::ExpectedFunctionBody => write!(f, "expected function body"),
            E::ExpectedIdentifier(id_type) => write!(f, "expected {}", id_type),
            E::ExpectedInitializer => write!(f, "expected initializer"),
            E::ExpectedMethods => write!(f, "expected methods block"),
            E::ExpectedParameters => write!(f, "expected parameters"),
            E::ExpectedThen => write!(f, "expected `{}` following predicate`", KeywordToken::Then),
            E::ExpectedTopLevelDefinition => write!(f, "expected struct, tuple, enum, or function"),
            E::ExpectedType => write!(f, "expected type name"),
            E::ExpectedVariants => write!(f, "expected enum variants"),
            E::UnexpectedBlockReturn(statement_type) => {
                return write!(f, "unexpected block return in {}", statement_type);
            }
            E::UnexpectedTypeExpression => return write!(f, "unexpected type declaration"),
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
        use OperatorToken as O;
        use SyntaxError as E;
        match self.error.error {
            E::BlockReturnEarly => write!(f, "block return must be the last statement in a block"),
            E::ExpectedBlock => fmt_op(f, O::OpenParen),
            E::ExpectedCloseBracket => fmt_op(f, O::CloseBracket),
            E::ExpectedCloseParen => fmt_op(f, O::CloseParen),
            E::ExpectedClosureParameter => write!(f, "expected parameter for closure"),
            E::ExpectedElse => write!(f, "expected `{}`", KeywordToken::Else),
            E::ExpectedEndStatement => fmt_op(f, O::Semicolon),
            E::ExpectedExpression => write!(f, "expected expression"),
            E::ExpectedFunctionBody => fmt_ops(f, O::SkinnyArrow, O::OpenBrace),
            E::ExpectedIdentifier(id_type) => write!(f, "expected {}", id_type),
            E::ExpectedInitializer => fmt_op(f, O::Equal),
            E::ExpectedFields => fmt_op(f, O::OpenParen),
            E::ExpectedMethods => fmt_ops(f, O::OpenBrace, O::Semicolon),
            E::ExpectedParameters => fmt_op(f, O::OpenParen),
            E::ExpectedThen => write!(f, "expected `{}`", KeywordToken::Then),
            E::ExpectedTopLevelDefinition => write!(f, "expected struct, tuple, enum, or function"),
            E::ExpectedType => write!(f, "expected type name"),
            E::ExpectedVariants => fmt_op(f, O::OpenParen),
            E::UnexpectedBlockReturn(_) => {
                write!(f, "block returns are only allowed in expressions")
            }
            E::UnexpectedTypeExpression => write!(
                f,
                "type declarations should only appear in closure parameter lists"
            ),
        }
    }
}

fn fmt_op(f: &mut std::fmt::Formatter<'_>, operator: OperatorToken) -> std::fmt::Result {
    write!(f, "expected `{}`", operator)
}

fn fmt_ops(
    f: &mut std::fmt::Formatter<'_>,
    op1: OperatorToken,
    op2: OperatorToken,
) -> std::fmt::Result {
    write!(f, "expected `{}` or `{}`", op1, op2)
}
