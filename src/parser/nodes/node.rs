use std::ops::{Deref, DerefMut};

use crate::parser::TokenSpan;

pub struct Node<T> {
    pub value: T,
    pub span: TokenSpan,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub type NodeVec<T> = Node<Vec<Node<T>>>;
