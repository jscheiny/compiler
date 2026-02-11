use std::cell::OnceCell;

use crate::{
    checker::{EnumType, TypeResolver},
    parser::{
        EnumVariantParseNode, Identified, IdentifierParseNode, MethodParseNode, ParseNode,
        ParseNodeVec,
    },
};

pub struct EnumParseNode {
    identifier: ParseNode<IdentifierParseNode>,
    variants: ParseNodeVec<EnumVariantParseNode>,
    methods: Option<ParseNodeVec<MethodParseNode>>,
    resolved_type: OnceCell<EnumType>,
}

impl EnumParseNode {
    pub fn new(
        identifier: ParseNode<IdentifierParseNode>,
        variants: ParseNodeVec<EnumVariantParseNode>,
        methods: Option<ParseNodeVec<MethodParseNode>>,
    ) -> Self {
        Self {
            identifier,
            variants,
            methods,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &mut TypeResolver) -> &EnumType {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    pub fn get_type_impl(&self, types: &mut TypeResolver) -> EnumType {
        let enum_name = self.id().clone();
        let mut enum_type = EnumType::new();

        for variant in self.variants.iter() {
            let member = variant.get_type(types).cloned();
            enum_type.add_variant(variant.id(), &enum_name, member, types);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                let member = method.resolve_enum_method(types);
                enum_type.add_method(method.id(), &enum_name, member, types);
            }
        }

        enum_type
    }
}

impl Identified for EnumParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
