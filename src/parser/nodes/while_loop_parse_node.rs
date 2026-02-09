use crate::parser::{BlockParseNode, ExpressionParseNode, ParseNode, TokenSpan, Traverse};

pub struct WhileLoopParseNode {
    pub predicate: ParseNode<ExpressionParseNode>,
    pub body: ParseNode<BlockParseNode>,
}

impl Traverse for WhileLoopParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.predicate.traverse("WhileLoop.predicate", visit);
        self.body.traverse("WhileLoop.body", visit);
    }
}
