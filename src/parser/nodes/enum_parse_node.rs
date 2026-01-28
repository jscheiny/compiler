use crate::parser::{MethodParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse, TypeParseNode};

#[derive(Debug)]
pub struct EnumParseNode {
    pub identifier: ParseNode<String>,
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

#[derive(Debug)]
pub struct EnumVariantParseNode {
    pub identifier: ParseNode<String>,
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
