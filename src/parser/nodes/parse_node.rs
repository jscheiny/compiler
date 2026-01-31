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

pub type ParseNodeVec<T> = ParseNode<Vec<ParseNode<T>>>;
