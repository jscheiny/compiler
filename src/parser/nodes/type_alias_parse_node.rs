use crate::parser::{IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode};

pub struct TypeAliasParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl Traverse for TypeAliasParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("TypeAlias.identifier", self.identifier.span);
        self.type_def.traverse("TypeAlias.identifer", visit);
    }
}
