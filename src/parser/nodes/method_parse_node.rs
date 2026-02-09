use crate::parser::{FunctionParseNode, ParseNode, TokenSpan, Traverse};

pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionParseNode>,
}

impl Traverse for MethodParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.function.traverse("Method.function", visit);
    }
}
