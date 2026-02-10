use crate::{
    checker::TypeResolver,
    parser::{
        EnumVariantParseNode, IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec,
        TokenSpan, Traverse,
    },
};

pub struct EnumParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub variants: ParseNodeVec<EnumVariantParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for EnumParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Enum.identifier", self.identifier.span);
        visit("Enum.variants", self.variants.span);
        for variant in self.variants.value.iter() {
            variant.traverse("Enum.variant", visit);
        }
        if let Some(methods) = self.methods.as_ref() {
            visit("Enum.methods", methods.span);
            for method in methods.value.iter() {
                method.traverse("Enum.method", visit);
            }
        }
    }
}

impl EnumParseNode {
    pub fn register_type(&self, types: &mut TypeResolver) {
        todo!("Enum type registration not implemented")
    }
}
