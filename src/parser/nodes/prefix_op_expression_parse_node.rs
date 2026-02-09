use crate::parser::{ExpressionParseNode, ParseNode, PrefixOperator, TokenSpan, Traverse};

pub struct PrefixOpExpressionParseNode {
    pub operator: ParseNode<PrefixOperator>,
    pub expression: Box<ParseNode<ExpressionParseNode>>,
}

impl Traverse for PrefixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("PrefixOpExpression.operator", self.operator.span);
        self.expression
            .traverse("PrefixOpExpression.expression", visit);
    }
}
