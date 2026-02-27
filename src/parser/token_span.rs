use crate::parser::{Node, TokenStream};

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

    pub fn previous(&self) -> TokenSpan {
        let index = self.start_index - 1;
        TokenSpan {
            start_index: index,
            end_index: index,
        }
    }

    pub fn expand_to(&self, tokens: &TokenStream) -> TokenSpan {
        TokenSpan {
            start_index: self.start_index,
            end_index: tokens.index(),
        }
    }

    pub fn wrap<T>(self, value: T) -> Node<T> {
        Node { value, span: self }
    }
}
