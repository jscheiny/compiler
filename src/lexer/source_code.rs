use std::rc::Rc;

use crate::lexer::{LocatedToken, tokenize};

pub struct SourceCode {
    pub tokens: Rc<Vec<LocatedToken>>,
    pub source: String,
}

impl SourceCode {
    pub fn from(text: &str) -> Self {
        let tokens = Rc::new(tokenize(text));
        let source = text.to_owned();
        Self { tokens, source }
    }
}
