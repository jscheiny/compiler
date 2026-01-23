use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseNode<T: Debug> {
    pub value: T,
    pub token_start_index: usize,
    pub token_end_index: usize,
}

pub type ParseNodeVec<T> = ParseNode<Vec<ParseNode<T>>>;
