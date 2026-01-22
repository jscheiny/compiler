use std::fmt::Debug;

use crate::parser::{LocatedNode, TokenTraverser};

pub struct NodeSpanTracker {
    token_start_index: usize,
}

impl NodeSpanTracker {
    pub fn new(token_start_index: usize) -> Self {
        Self { token_start_index }
    }

    pub fn close<ParseNode: Debug>(
        &self,
        tokens: &TokenTraverser,
        node: ParseNode,
    ) -> LocatedNode<ParseNode> {
        LocatedNode {
            value: node,
            token_start_index: self.token_start_index,
            token_end_index: tokens.index(),
        }
    }

    pub fn singleton<ParseNode: Debug>(&self, node: ParseNode) -> LocatedNode<ParseNode> {
        LocatedNode {
            value: node,
            token_start_index: self.token_start_index,
            token_end_index: self.token_start_index + 1,
        }
    }
}
