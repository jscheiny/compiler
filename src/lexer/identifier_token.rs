use crate::{
    lexer::{Token, TokenParse},
    parser::ParserPredicate,
};

#[derive(Debug, Clone)]
pub struct IdentifierToken(pub String);

impl ParserPredicate for IdentifierToken {
    fn is_match(&self, token: &Token) -> bool {
        match token {
            Token::Identifier(_) => true,
            _ => false,
        }
    }
}

impl TokenParse for IdentifierToken {
    fn try_tokenize(text: &str) -> Option<(Token, usize)> {
        let mut identifier = String::from("");
        for character in text.chars() {
            if identifier.is_empty() && !character.is_alphabetic() {
                return None;
            }

            if !character.is_alphanumeric() && character != '_' {
                break;
            }

            identifier.push(character);
        }

        let len = identifier.len();
        if len == 0 {
            None
        } else {
            Some((Token::Identifier(IdentifierToken(identifier)), len))
        }
    }
}
