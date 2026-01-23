use std::fmt::Debug;

#[derive(Debug)]
pub struct LocatedNode<ParseNode: Debug> {
    pub value: ParseNode,
    pub token_start_index: usize,
    pub token_end_index: usize,
}

pub type LocatedNodeVec<ParseNode> = LocatedNode<Vec<LocatedNode<ParseNode>>>;
