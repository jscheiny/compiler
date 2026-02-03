use std::fmt::Display;

use crate::{
    lexer::{LocatedToken, OperatorToken, Token},
    parser::TokenSpan,
};

pub struct LocatedSyntaxError {
    pub span: TokenSpan,
    pub error: SyntaxError,
}

impl LocatedSyntaxError {
    pub fn print(&self, tokens: &Vec<LocatedToken>) {
        print!("{}", self.error);
        self.print_found_token(tokens);
    }

    pub fn print_found_token(&self, tokens: &Vec<LocatedToken>) {
        let LocatedToken { token, .. } = &tokens[self.span.start_index];
        print!(", found ");
        match token {
            Token::Identifier(identifier) => print!("identifier `{}`", identifier),
            Token::IntegerLiteral(literal) => print!("integer literal `{}`", literal),
            Token::StringLiteral(literal) => print!("string literal {}", literal),
            Token::Operator(operator) => print!("operator: `{}`", operator.to_string()),
            Token::Keyword(keyword) => print!("keyword: `{}`", keyword.as_str()),
            Token::EndOfFile => print!("end of file"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SyntaxError {
    ExpectedBlock,
    ExpectedCloseParen,
    ExpectedEndStatement,
    ExpectedExpression,
    ExpectedFunctionBody,
    ExpectedIdentifier,
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

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OperatorToken as O;
        match self {
            Self::ExpectedBlock => write!(f, "expected`{}`", O::OpenBrace),
            Self::ExpectedCloseParen => write!(f, "expectedclosing parenthesis"),
            Self::ExpectedEndStatement => write!(f, "expected`{}`", O::EndStatement),
            Self::ExpectedExpression => write!(f, "expectedexpression"),
            Self::ExpectedFunctionBody => write!(
                f,
                "expected function body with `{}` or `{}`",
                O::FunctionDefinition,
                O::OpenBrace
            ),
            Self::ExpectedIdentifier => write!(f, "expectedidentifier"),
            Self::ExpectedInitializer => write!(f, "expectedinitializer with `{}`", O::Assign),
            Self::ExpectedMembers => write!(f, "expectedmembers"),
            Self::ExpectedMethods => write!(f, "expectedmethods block or `{}`", O::EndStatement),
            Self::ExpectedMethodSignatures => write!(f, "expectedmethod signatures block"),
            Self::ExpectedParameters => {
                write!(f, "expectedfunction parameters with `{}`", O::OpenParen)
            }
            Self::ExpectedReturnType => write!(f, "expectedreturn type"),
            Self::ExpectedTopLevelDefinition => {
                write!(f, "expectedstruct, tuple, enum, or function")
            }
            Self::ExpectedType => write!(f, "expectedtype"),
            Self::ExpectedVariants => write!(f, "expectedenum variants"),
        }
    }
}
