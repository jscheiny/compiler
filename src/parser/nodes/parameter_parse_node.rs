use crate::parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode};

pub struct ParameterParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl Traverse for ParameterParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Parameter.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("Parameter.type", visit);
        }
    }
}
