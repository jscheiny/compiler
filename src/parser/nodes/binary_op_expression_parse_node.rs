use crate::parser::{BinaryOperator, ExpressionParseNode, ParseNode, TokenSpan, Traverse};

pub struct BinaryOpExpressionParseNode {
    pub left: Box<ParseNode<ExpressionParseNode>>,
    pub operator: ParseNode<BinaryOperator>,
    pub right: Box<ParseNode<ExpressionParseNode>>,
}

impl Traverse for BinaryOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.left.traverse("BinaryOpExpression.left", visit);
        visit("BinaryOpExpression.operator", self.operator.span);
        self.right.traverse("BinaryOpExpression.right", visit);
    }
}
