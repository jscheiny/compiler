use std::{cell::OnceCell, collections::HashMap};

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
            let field_type = field.get_type(types).clone();
            scope.add_or(field.id(), field_type, || {
                println!("Type error: Duplicate member of name `{}`", field.id())
            });
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                let method_type = Type::Function(method.function.get_type(types).clone());
                scope.add_or(method.id(), method_type, || {
                    println!("Type error: Duplicate member of name `{}`", method.id())
                });
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
        let mut members = HashMap::new();

        for field in self.fields.iter() {
            let member = field.get_member(types);
            let identifier = field.id().clone();
            members.entry(identifier).or_insert(member);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                let member = method.resolve_struct_method(types);
                let identifier = method.id().clone();
                members.entry(identifier).or_insert(member);
            }
        }

        StructType { members }
    }
}

impl Identified for StructNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
