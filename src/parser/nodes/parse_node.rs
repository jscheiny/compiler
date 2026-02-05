use crate::parser::TokenStream;

pub struct ParseNode<T> {
    pub value: T,
    pub span: TokenSpan,
}

impl<T: Traverse> ParseNode<T> {
    pub fn traverse(&self, field: &str, visit: &impl Fn(&str, TokenSpan)) {
        visit(field, self.span);
        self.value.traverse(visit);
    }
}

pub trait Traverse {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan));
}

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

pub type ParseNodeVec<T> = ParseNode<Vec<ParseNode<T>>>;
