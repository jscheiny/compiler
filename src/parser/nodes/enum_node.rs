use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
};

use crate::{
    checker::{EnumMethod, EnumType, Scope, ScopeType},
    parser::{
        EnumVariantNode, Identified, IdentifierNode, ImplementationNode, ImplementationNodeType,
        Node, NodeVec,
    },
};

pub struct EnumNode {
    pub identifier: Node<IdentifierNode>,
    pub variants: NodeVec<EnumVariantNode>,
    pub implementation: Option<Node<ImplementationNode>>,
    resolved_type: OnceCell<EnumType>,
}

impl EnumNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        variants: NodeVec<EnumVariantNode>,
        implementation: Option<Node<ImplementationNode>>,
    ) -> Self {
        Self {
            identifier,
            variants,
            implementation,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let index = scope.get_type_index(self.id()).unwrap();
        scope.nest(ScopeType::Struct(index), |scope| self.check_nested(scope))
    }

    fn check_nested(&self, scope: Box<Scope>) -> Box<Scope> {
        let mut scope_names = HashSet::new();
        for variant in self.variants.iter() {
            if !scope_names.insert(variant.id().clone()) {
                scope.source.print_error(
                    variant.identifier.span,
                    &format!("Duplicate enum variant `{}`", variant.id()),
                    &format!(
                        "enum `{}` already contains a variant with this name",
                        self.id()
                    ),
                );
            }
        }

        if let Some(implementation) = self.implementation.as_ref() {
            return implementation.check(
                scope,
                ImplementationNodeType::Enum,
                self.id(),
                scope_names,
            );
        }

        scope
    }

    pub fn get_type(&self, scope: &Scope) -> &EnumType {
        self.resolved_type.get_or_init(|| self.get_type_impl(scope))
    }

    fn get_type_impl(&self, scope: &Scope) -> EnumType {
        let mut variants = HashMap::new();
        for variant in self.variants.iter() {
            let identifier = variant.id().clone();
            let variant = variant.get_type(scope).cloned();
            variants.entry(identifier).or_insert(variant);
        }

        let mut methods = HashMap::new();
        if let Some(implementation) = self.implementation.as_ref() {
            for (identifier, public, function_type) in implementation.get_methods(scope) {
                methods.entry(identifier).or_insert(EnumMethod {
                    public,
                    function_type,
                });
            }
        }

        EnumType {
            identifier: self.id().clone(),
            variants,
            methods,
        }
    }
}

impl Identified for EnumNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
