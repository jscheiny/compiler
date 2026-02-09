use crate::parser::{ExpressionParseNode, ParseNode, TokenSpan, Traverse};

pub struct IfExpressionParseNode {
    pub predicate: Box<ParseNode<ExpressionParseNode>>,
    pub if_true: Box<ParseNode<ExpressionParseNode>>,
    pub if_false: Box<ParseNode<ExpressionParseNode>>,
}

impl Traverse for IfExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.predicate.traverse("IfExpression.predicate", visit);
        self.if_true.traverse("IfExpression.if_true", visit);
        self.if_false.traverse("IfExpression.if_false", visit);
    }
}
