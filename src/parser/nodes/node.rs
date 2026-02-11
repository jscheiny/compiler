use std::ops::{Deref, DerefMut};

use crate::parser::TokenSpan;

pub struct ParseNode<T> {
    pub value: T,
    pub span: TokenSpan,
}

impl<T> Deref for ParseNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ParseNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub type ParseNodeVec<T> = ParseNode<Vec<ParseNode<T>>>;
