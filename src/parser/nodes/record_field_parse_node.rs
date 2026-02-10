use crate::parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode};

pub struct RecordFieldParseNode {
    pub public: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl Traverse for RecordFieldParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("RecordField.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("RecordField.type", visit);
        }
    }
}
