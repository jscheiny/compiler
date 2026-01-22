use std::fmt::Debug;

use crate::parser::{LocatedNode, ParseResult, TokenTraverser};

pub struct NodeSpanTracker {
    token_start_index: usize,
}

impl NodeSpanTracker {
    pub fn new(token_start_index: usize) -> Self {
        Self { token_start_index }
    }

    pub fn create_node<ParseNode: Debug>(
        &self,
        tokens: &TokenTraverser,
        node: ParseNode,
    ) -> ParseResult<ParseNode> {
        Ok(LocatedNode {
            node,
            token_start_index: self.token_start_index,
            token_end_index: tokens.index(),
        })
    }
}
