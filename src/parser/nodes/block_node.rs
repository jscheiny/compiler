use crate::parser::{Node, StatementNode};

pub struct BlockNode {
    pub statements: Vec<Node<StatementNode>>,
}

impl BlockNode {
    pub fn check(&self) {
        for statement in self.statements.iter() {
            statement.check();
        }
    }
}
