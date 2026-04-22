use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{EnumType, Scope, ScopeType},
    parser::{EnumVariantNode, ImplementationNode, ImplementationType, NameNode, Node, NodeVec},
};

pub struct EnumNode {
    pub name: NameNode,
    pub variants: NodeVec<EnumVariantNode>,
    pub implementation: Option<Node<ImplementationNode>>,
    resolved_type: OnceCell<Rc<EnumType>>,
}

impl EnumNode {
    pub fn new(
        name: NameNode,
        variants: NodeVec<EnumVariantNode>,
        implementation: Option<Node<ImplementationNode>>,
    ) -> Self {
        Self {
            name,
            variants,
            implementation,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(self: &Rc<Self>, scope: Box<Scope>) -> Box<Scope> {
        let self_type = self.get_type(&scope);
        scope.nest(ScopeType::Enum(self_type), |scope| self.check_nested(scope))
    }

    fn check_nested(self: &Rc<Self>, scope: Box<Scope>) -> Box<Scope> {
        let mut scope_names = HashSet::new();
        for variant in self.variants.iter() {
            if !scope_names.insert(variant.name.clone()) {
                scope.source.print_error(
                    variant.name.span,
                    &format!("Duplicate enum variant `{}`", variant.name),
                    &format!(
                        "enum `{}` already contains a variant with this name",
                        self.name
                    ),
                );
            }
        }

        if let Some(implementation) = self.implementation.as_ref() {
            let self_type = ImplementationType::Enum(self.get_type(&scope));
            return implementation.check(scope, &self_type, scope_names);
        }

        scope
    }

    pub fn get_type(self: &Rc<Self>, scope: &Scope) -> Rc<EnumType> {
        self.resolved_type
            .get_or_init(|| EnumType::from(self.clone(), scope))
            .clone()
    }
}
