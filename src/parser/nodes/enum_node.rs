use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{EnumType, Scope, ScopeType, Type},
    parser::{EnumVariantNode, Identified, IdentifierNode, ImplementationNode, Node, NodeVec},
};

pub struct EnumNode {
    pub identifier: Node<IdentifierNode>,
    pub variants: NodeVec<EnumVariantNode>,
    pub implementation: Option<Node<ImplementationNode>>,
    resolved_type: OnceCell<Rc<EnumType>>,
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

    pub fn check(self: &Rc<Self>, scope: Box<Scope>) -> Box<Scope> {
        let index = scope.get_type_index(self.id()).unwrap();
        scope.nest(ScopeType::Struct(index), |scope| self.check_nested(scope))
    }

    fn check_nested(self: &Rc<Self>, scope: Box<Scope>) -> Box<Scope> {
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
            let self_type = Type::Enum(self.get_type(&scope));
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

impl Identified for EnumNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
