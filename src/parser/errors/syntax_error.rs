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
        match self.error {
            SyntaxError::Expected(_) => {
                print!("{}", self.error);
                self.print_found_token(tokens);
            }
            SyntaxError::Unimplemented => print!("Unimplemented syntax error type"),
        }
    }

    pub fn print_found_token(&self, tokens: &Vec<LocatedToken>) {
        let LocatedToken { token, .. } = &tokens[self.span.start_index];
        print!(", found ");
        match token {
            Token::Identifier(identifier) => print!("identifier '{}'", identifier),
            Token::IntegerLiteral(literal) => print!("integer literal '{}'", literal),
            Token::StringLiteral(literal) => print!("string literal {}", literal),
            Token::Operator(operator) => print!("operator: '{}'", operator.to_string()),
            Token::Keyword(keyword) => print!("keyword: '{}'", keyword.as_str()),
            Token::EndOfFile => print!("end of file"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SyntaxError {
    Expected(ExpectedSyntax),
    Unimplemented,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expected(expected) => write!(f, "expected {}", expected),
            Self::Unimplemented => write!(f, "Unimplemented syntax error type"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ExpectedSyntax {
    FunctionBody,
    Identifier,
    Members,
    Methods,
    TopLevelDefinition,
    Type,
    Variants,
}

impl Display for ExpectedSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OperatorToken as O;
        match self {
            Self::FunctionBody => write!(
                f,
                "function body with `{}` or `{}`",
                O::FunctionDefinition,
                O::OpenBrace
            ),
            Self::Members => write!(f, "members"),
            Self::Identifier => write!(f, "identifier"),
            Self::Methods => write!(f, "methods block or `{}`", O::EndStatement),
            Self::TopLevelDefinition => write!(f, "struct, tuple, enum, or function"),
            Self::Type => write!(f, "type"),
            Self::Variants => write!(f, "enum variants"),
        }
    }
}
