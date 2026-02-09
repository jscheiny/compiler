use crate::parser::{ParseNode, ParseNodeVec, TokenSpan, Traverse, TypeParseNode};

pub struct FunctionTypeParseNode {
    pub parameters: ParseNodeVec<TypeParseNode>,
    pub return_type: Box<ParseNode<TypeParseNode>>,
}

impl Traverse for FunctionTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("FunctionType.parameters", self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse("FunctionType.parameter", visit);
        }
        self.return_type.traverse("FunctionType.return_type", visit);
    }
}
