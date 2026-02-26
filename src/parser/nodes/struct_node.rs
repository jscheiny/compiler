use std::{cell::OnceCell, collections::HashMap};

use crate::{
    checker::{Scope, ScopeType, StructType, Type, TypeResolver},
    lexer::SourceCode,
    parser::{Identified, IdentifierNode, MethodNode, Node, NodeVec, StructFieldNode},
};

pub struct StructNode {
    pub identifier: Node<IdentifierNode>,
    pub fields: NodeVec<StructFieldNode>,
    pub methods: Option<NodeVec<MethodNode>>,
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

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let index = scope.types.get_ref(self.id()).unwrap();
        scope.nest(ScopeType::Struct(index), |mut scope| {
            for field in self.fields.iter() {
                let field_type = field.get_type(&scope.types, &scope.source).clone();
                scope.add_or(field.id(), field_type, |scope| {
                    scope.source.print_type_error(
                        field.identifier.span,
                        &format!("Duplicate struct member `{}`", field.id()),
                        &format!(
                            "a field of struct `{}` already exists with this name",
                            self.id()
                        ),
                    );
                });
            }

            if let Some(methods) = self.methods.as_ref() {
                for method in methods.iter() {
                    let method_type = Type::Function(
                        method
                            .function
                            .get_type(&scope.types, &scope.source)
                            .clone(),
                    );
                    scope.add_or(method.id(), method_type, |scope| {
                        scope.source.print_type_error(
                            method.function.identifier.span,
                            &format!("Duplicate struct member `{}`", method.id()),
                            &format!(
                                "a field or method of struct `{}` already exists with this name",
                                self.id()
                            ),
                        );
                    });
                }

                for method in methods.iter() {
                    scope = method.check(scope)
                }
            }

            scope
        })
    }

    pub fn get_type(&self, types: &TypeResolver, source: &SourceCode) -> &StructType {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(types, source))
    }

    pub fn get_type_impl(&self, types: &TypeResolver, source: &SourceCode) -> StructType {
        let mut members = HashMap::new();

        for field in self.fields.iter() {
            let member = field.get_member(types, source);
            let identifier = field.id().clone();
            members.entry(identifier).or_insert(member);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                let member = method.resolve_struct_method(types, source);
                let identifier = method.id().clone();
                members.entry(identifier).or_insert(member);
            }
        }

        StructType {
            identifier: self.id().clone(),
            members,
        }
    }
}

impl Identified for StructNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
