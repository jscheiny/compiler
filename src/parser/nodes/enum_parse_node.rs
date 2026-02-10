use crate::{
    checker::{EnumType, Type, TypeResolver},
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
    pub fn declare_type(&self, types: &mut TypeResolver) {
        types.declare(self.identifier());
    }

    pub fn resolve_types(&self, types: &mut TypeResolver) {
        let enum_name = self.identifier();
        let mut enum_type = EnumType::new();

        for variant in self.variants.value.iter() {
            let member = variant.value.resolve_type(types);
            let identifier = &variant.value.identifier.value.0;
            enum_type.add_variant(identifier, enum_name, member, types);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.value.iter() {
                let member = method.value.resolve_enum_method(types);
                let identifier = &method.value.function.value.identifier.value.0;
                enum_type.add_method(identifier, enum_name, member, types);
            }
        }

        types.resolve(enum_name, Type::Enum(enum_type))
    }

    pub fn identifier(&self) -> &String {
        &self.identifier.value.0
    }
}
