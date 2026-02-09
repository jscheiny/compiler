use crate::parser::{ParseNode, StatementParseNode, TokenSpan, Traverse};

pub struct BlockParseNode {
    pub statements: Vec<ParseNode<StatementParseNode>>,
}

impl Traverse for BlockParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for statement in self.statements.iter() {
            statement.traverse("Block.statement", visit);
        }
    }
}
