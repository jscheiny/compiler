use crate::parser::{
    BlockParseNode, IfStatementConditionParseNode, ParseNode, TokenSpan, Traverse,
};

pub struct IfStatementParseNode {
    pub conditions: Vec<ParseNode<IfStatementConditionParseNode>>,
    pub else_branch: Option<ParseNode<BlockParseNode>>,
}

impl Traverse for IfStatementParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for condition in self.conditions.iter() {
            condition.traverse("IfStatement.condition", visit);
        }
        if let Some(else_branch) = self.else_branch.as_ref() {
            else_branch.traverse("IfStatement.else", visit);
        }
    }
}
