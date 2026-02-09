use crate::parser::{ExpressionParseNode, ParseNode, PostfixOperator, TokenSpan, Traverse};

pub struct PostfixOpExpressionParseNode {
    pub expression: Box<ParseNode<ExpressionParseNode>>,
    pub operator: ParseNode<PostfixOperator>,
}

impl Traverse for PostfixOpExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.expression
            .traverse("PostfixOpExpression.expression", visit);
        visit("PostfixOpExpression.operator", self.operator.span);
    }
}
