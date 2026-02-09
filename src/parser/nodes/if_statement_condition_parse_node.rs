use crate::parser::{BlockParseNode, ExpressionParseNode, ParseNode, TokenSpan, Traverse};

pub struct IfStatementConditionParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

impl Traverse for IfStatementConditionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.predicate
            .traverse("IfStatementCondition.predicate", visit);
        self.body.traverse("IfStatementCondition.body", visit);
    }
}
