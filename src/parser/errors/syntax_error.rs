use std::fmt::Display;

use crate::{
    lexer::{LocatedToken, Token},
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
            SyntaxErrorType::ExpectedIdentifier => {
                let LocatedToken { token, .. } = &tokens[self.span.start_index];
                match token {
                    Token::Identifier(_) => panic!("Identifier expected"),
                    Token::Ignored(_) => panic!("Ignored token in stream"),
                    Token::IntegerLiteral(_) => print!("{}, found integer literal", self.kind),
                    Token::StringLiteral(_) => print!("{}, found string literal", self.kind),
                    Token::Operator(operator) => {
                        print!("{}, found operator: '{}'", self.kind, operator.to_string())
                    }
                    Token::Keyword(keyword) => {
                        print!("{}, found keyword: '{}'", self.kind, keyword.to_string())
                    }
                }
            }
            SyntaxErrorType::Unimplemented => unimplemented!("Unimplemented syntax error type"),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxErrorType {
    ExpectedIdentifier,
    Unimplemented,
}

impl Display for SyntaxErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxErrorType::ExpectedIdentifier => write!(f, "expected identifier"),
            SyntaxErrorType::Unimplemented => unimplemented!("Unimplemented syntax error type"),
        }
    }
}
