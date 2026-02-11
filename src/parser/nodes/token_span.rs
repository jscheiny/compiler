use crate::parser::{ParseNode, TokenStream};

#[derive(Clone, Copy)]
pub struct TokenSpan {
    pub start_index: usize,
    pub end_index: usize,
}

impl TokenSpan {
    pub fn singleton(tokens: &TokenStream) -> TokenSpan {
        TokenSpan {
            start_index: tokens.index(),
            end_index: tokens.index(),
        }
    }

    pub fn expand_to(&self, tokens: &TokenStream) -> TokenSpan {
        TokenSpan {
            start_index: self.start_index,
            end_index: tokens.index(),
        }
    }

    pub fn wrap<T>(self, value: T) -> ParseNode<T> {
        ParseNode { value, span: self }
    }
}
