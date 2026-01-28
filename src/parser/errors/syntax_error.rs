use std::fmt::Display;

use crate::{
    lexer::{LocatedToken, OperatorToken, Token},
    parser::TokenSpan,
};

#[derive(Debug)]
pub struct SyntaxError {
    pub span: TokenSpan,
    pub kind: SyntaxErrorType,
}

impl SyntaxError {
    pub fn print(&self, tokens: &Vec<LocatedToken>) {
        match self.kind {
            SyntaxErrorType::ExpectedIdentifier | SyntaxErrorType::ExpectedMethods => {
                print!("{}", self.kind);
                self.print_found_token(tokens);
            }
            SyntaxErrorType::Unimplemented => unimplemented!("Unimplemented syntax error type"),
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
            Token::Keyword(keyword) => print!("keyword: '{}'", keyword.to_string()),
            Token::Ignored(_) => panic!("Ignored token in stream"),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxErrorType {
    ExpectedIdentifier,
    ExpectedMethods,
    Unimplemented,
}

impl Display for SyntaxErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedIdentifier => write!(f, "expected identifier"),
            Self::ExpectedMethods => write!(
                f,
                "expected methods block or `{}`",
                OperatorToken::EndStatement,
            ),
            Self::Unimplemented => unimplemented!("Unimplemented syntax error type"),
        }
    }
}
