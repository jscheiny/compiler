use crate::parser::{ExpressionParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse};

pub struct FunctionCallExpressionParseNode {
    pub function: Box<ParseNode<ExpressionParseNode>>,
    pub arguments: ParseNodeVec<ExpressionParseNode>,
}

impl Traverse for FunctionCallExpressionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.function.traverse("FunctionCall.function", visit);
        visit("FunctionCall.arguments", self.arguments.span);
        for argument in self.arguments.value.iter() {
            argument.traverse("FunctionCall.argument", visit);
        }
    }
}
