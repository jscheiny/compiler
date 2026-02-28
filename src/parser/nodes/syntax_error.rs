use std::{fmt::Display, rc::Rc};

use crate::{
    lexer::{Keyword, LocatedToken, Symbol, Token},
    parser::TokenSpan,
};

#[derive(Clone, Copy)]
pub enum SyntaxError {
    BlockReturnEarly,
    ExpectedBlock,
    ExpectedCloseBracket,
    ExpectedCloseParen,
    ExpectedClosureBody,
    ExpectedClosureParameter,
    ExpectedElse,
    ExpectedEndStatement,
    ExpectedExpression,
    ExpectedFields,
    ExpectedFunctionBody,
    ExpectedIdentifier(IdentifierType),
    ExpectedInitializer,
    ExpectedMatchBlock,
    ExpectedMatchExpression,
    ExpectedMatchPattern,
    ExpectedMethods,
    ExpectedParameters,
    ExpectedThen,
    ExpectedTopLevelDefinition,
    ExpectedType,
    ExpectedVariants,
    UnexpectedBindingPattern,
    UnexpectedBlockReturn(StatementType),
    UnexpectedTypeExpression,
}

#[derive(Clone, Copy)]
pub enum IdentifierType {
    Field,
    Function,
    Method,
    Parameter,
    PatternBinding,
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
            Self::PatternBinding => "pattern binding",
            Self::Struct => "struct",
            Self::Variable => "variable",
            Self::Variant => "variant",
        };
        write!(f, "{} name", message)
    }
}

#[derive(Clone, Copy)]
pub enum StatementType {
    Block,
    If,
    WhileLoop,
}

impl Display for StatementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block => write!(f, "statement block"),
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
            E::ExpectedClosureBody => write!(f, "expected closure body"),
            E::ExpectedClosureParameter => write!(f, "expected parameter"),
            E::ExpectedElse => write!(f, "`{}` following true branch expression", Keyword::Else),
            E::ExpectedEndStatement => write!(f, "expected end of statement"),
            E::ExpectedExpression => write!(f, "expected expression"),
            E::ExpectedFields => write!(f, "expected fields"),
            E::ExpectedFunctionBody => write!(f, "expected function body"),
            E::ExpectedIdentifier(id_type) => write!(f, "expected {}", id_type),
            E::ExpectedInitializer => write!(f, "expected initializer"),
            E::ExpectedMatchBlock => write!(f, "expected match block"),
            E::ExpectedMatchExpression => write!(f, "expected expression"),
            E::ExpectedMatchPattern => write!(f, "expected match pattern"),
            E::ExpectedMethods => write!(f, "expected methods block"),
            E::ExpectedParameters => write!(f, "expected parameters"),
            E::ExpectedThen => write!(f, "expected `{}` following predicate`", Keyword::Then),
            E::ExpectedTopLevelDefinition => write!(f, "expected struct, tuple, enum, or function"),
            E::ExpectedType => write!(f, "expected type name"),
            E::ExpectedVariants => write!(f, "expected enum variants"),
            E::UnexpectedBindingPattern => {
                return write!(f, "unexpected top level binding pattern");
            }
            E::UnexpectedBlockReturn(statement_type) => {
                return write!(f, "unexpected block return in {}", statement_type);
            }
            E::UnexpectedTypeExpression => return write!(f, "unexpected type declaration"),
        }?;
        write!(f, ", found ")?;

        let LocatedToken { token, .. } = &self.tokens[self.error.span.start_index];
        use Token as T;
        match token {
            T::CharacterLiteral(_) => write!(f, "character literal"),
            T::Identifier(identifier) => write!(f, "identifier `{}`", identifier),
            T::IntegerLiteral(literal) => write!(f, "integer literal `{}`", literal),
            T::StringLiteral(literal) => write!(f, "string literal {}", literal),
            T::Symbol(symbol) => write!(f, "`{}`", symbol),
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
        use Symbol as S;
        use SyntaxError as E;
        match self.error.error {
            E::BlockReturnEarly => write!(f, "block return must be the last statement in a block"),
            E::ExpectedBlock => fmt_symbol(f, S::OpenBrace),
            E::ExpectedCloseBracket => fmt_symbol(f, S::CloseBracket),
            E::ExpectedCloseParen => fmt_symbol(f, S::CloseParen),
            E::ExpectedClosureBody => fmt_symbol(f, S::SkinnyArrow),
            E::ExpectedClosureParameter => write!(f, "expected parameter for closure"),
            E::ExpectedElse => write!(f, "expected `{}`", Keyword::Else),
            E::ExpectedEndStatement => fmt_symbol(f, S::Semicolon),
            E::ExpectedExpression => write!(f, "expected expression"),
            E::ExpectedFields => fmt_symbol(f, S::OpenParen),
            E::ExpectedFunctionBody => fmt_symbols(f, S::SkinnyArrow, S::OpenBrace),
            E::ExpectedIdentifier(id_type) => write!(f, "expected {}", id_type),
            E::ExpectedInitializer => fmt_symbol(f, S::Equal),
            E::ExpectedMatchBlock => fmt_symbol(f, S::OpenBrace),
            E::ExpectedMatchExpression => fmt_symbol(f, S::SkinnyArrow),
            E::ExpectedMatchPattern => write!(f, "expected pattern e.g. Variant(let binding)"),
            E::ExpectedMethods => fmt_symbols(f, S::OpenBrace, S::Semicolon),
            E::ExpectedParameters => fmt_symbol(f, S::OpenParen),
            E::ExpectedThen => write!(f, "expected `{}`", Keyword::Then),
            E::ExpectedTopLevelDefinition => write!(f, "expected struct, tuple, enum, or function"),
            E::ExpectedType => write!(f, "expected type name"),
            E::ExpectedVariants => fmt_symbol(f, S::OpenParen),
            E::UnexpectedBindingPattern => {
                write!(
                    f,
                    "will match all cases, use `{}` case instead",
                    Keyword::Else
                )
            }
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

fn fmt_symbol(f: &mut std::fmt::Formatter<'_>, symbol: Symbol) -> std::fmt::Result {
    write!(f, "expected `{}`", symbol)
}

fn fmt_symbols(f: &mut std::fmt::Formatter<'_>, s1: Symbol, s2: Symbol) -> std::fmt::Result {
    write!(f, "expected `{}` or `{}`", s1, s2)
}
