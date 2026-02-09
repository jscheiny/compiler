use crate::parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode};

pub struct EnumVariantParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
}

impl Traverse for EnumVariantParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("EnumVariant.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("EnumVariant.type", visit);
        }
    }
}
