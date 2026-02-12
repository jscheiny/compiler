use std::cell::OnceCell;

use crate::{
    checker::{Scope, ScopeType, StructType, Type, TypeResolver},
    parser::{Identified, IdentifierNode, MethodNode, Node, NodeVec, StructFieldNode},
};

pub struct StructNode {
    identifier: Node<IdentifierNode>,
    fields: NodeVec<StructFieldNode>,
    methods: Option<NodeVec<MethodNode>>,
    resolved_type: OnceCell<StructType>,
}

impl StructNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        fields: NodeVec<StructFieldNode>,
        methods: Option<NodeVec<MethodNode>>,
    ) -> Self {
        Self {
            identifier,
            fields,
            methods,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        let mut scope = scope.derive(ScopeType::Struct);
        for field in self.fields.iter() {
            let resolved_type = field.get_type(types).clone();
            scope.add_without_shadow(field.id(), resolved_type);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                scope.add_without_shadow(
                    method.id(),
                    Type::Function(method.function.get_type(types).clone()),
                );
            }

            for method in methods.iter() {
                scope = method.check(types, scope)
            }
        }

        scope.parent()
    }

    pub fn get_type(&self, types: &mut TypeResolver) -> &StructType {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    pub fn get_type_impl(&self, types: &mut TypeResolver) -> StructType {
        let container_name = self.id().clone();
        let mut struct_type = StructType::new();

        for field in self.fields.iter() {
            let member = field.get_member(types);
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
