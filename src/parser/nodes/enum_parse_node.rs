use crate::{
    checker::{EnumType, Type, TypeResolver},
    parser::{
        EnumVariantParseNode, Identified, IdentifierParseNode, MethodParseNode, ParseNode,
        ParseNodeVec,
    },
};

pub struct EnumParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub variants: ParseNodeVec<EnumVariantParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl EnumParseNode {
    pub fn resolve_types(&mut self, types: &mut TypeResolver) {
        let enum_name = self.id().clone();
        let mut enum_type = EnumType::new();

        for variant in self.variants.iter() {
            let member = variant.resolve_type(types);
            enum_type.add_variant(variant.id(), &enum_name, member, types);
        }

        if let Some(methods) = self.methods.as_mut() {
            for method in methods.iter_mut() {
                let member = method.resolve_enum_method(types);
                enum_type.add_method(method.id(), &enum_name, member, types);
            }
        }

        types.resolve(&enum_name, Type::Enum(enum_type))
    }
}

impl Identified for EnumParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
