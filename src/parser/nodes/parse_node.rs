use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseNode<T: Debug> {
    pub value: T,
    pub span: TokenSpan,
}

impl<T: Debug + Traverse> ParseNode<T> {
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

impl Debug for TokenSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start_index, self.end_index)
    }
}

pub type ParseNodeVec<T> = ParseNode<Vec<ParseNode<T>>>;
