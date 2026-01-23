use std::rc::Rc;

use crate::{
    lexer::{LocatedToken, tokenize},
    parser::TokenSpan,
};

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

    pub fn get_span_string(&self, span: TokenSpan) -> String {
        let start_byte = self.tokens[span.start_index].span.start.byte;
        let end_byte = self.tokens[span.end_index].span.end.byte;
        let source_slice = self.source.as_bytes()[start_byte..end_byte].to_owned();
        String::from_utf8(source_slice).unwrap()
    }
}
