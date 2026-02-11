use std::cell::OnceCell;

use crate::{
    checker::{EnumType, TypeResolver},
    parser::{EnumVariantNode, Identified, IdentifierNode, MethodNode, ParseNode, ParseNodeVec},
};

pub struct EnumNode {
    identifier: ParseNode<IdentifierNode>,
    variants: ParseNodeVec<EnumVariantNode>,
    methods: Option<ParseNodeVec<MethodNode>>,
    resolved_type: OnceCell<EnumType>,
}

impl EnumNode {
    pub fn new(
        identifier: ParseNode<IdentifierNode>,
        variants: ParseNodeVec<EnumVariantNode>,
        methods: Option<ParseNodeVec<MethodNode>>,
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

impl Identified for EnumNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
