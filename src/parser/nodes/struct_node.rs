use std::cell::OnceCell;

use crate::{
    checker::{StructType, TypeResolver},
    parser::{Identified, IdentifierNode, MethodNode, ParseNode, ParseNodeVec, StructFieldNode},
};

pub struct StructNode {
    identifier: ParseNode<IdentifierNode>,
    fields: ParseNodeVec<StructFieldNode>,
    methods: Option<ParseNodeVec<MethodNode>>,
    resolved_type: OnceCell<StructType>,
}

impl StructNode {
    pub fn new(
        identifier: ParseNode<IdentifierNode>,
        fields: ParseNodeVec<StructFieldNode>,
        methods: Option<ParseNodeVec<MethodNode>>,
    ) -> Self {
        Self {
            identifier,
            fields,
            methods,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &mut TypeResolver) -> &StructType {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    pub fn get_type_impl(&self, types: &mut TypeResolver) -> StructType {
        let container_name = self.id().clone();
        let mut struct_type = StructType::new();

        for field in self.fields.iter() {
            let member = field.resolve_type(types);
            let identifier = field.id().clone();
            struct_type.add_member(identifier, &container_name, member, types);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                let member = method.resolve_struct_method(types);
                let identifier = method.id().clone();
                struct_type.add_member(identifier, &container_name, member, types);
            }
        }

        struct_type
    }
}

impl Identified for StructNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
