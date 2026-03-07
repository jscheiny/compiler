use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{Scope, ScopeType, StructType},
    parser::{
        Identified, IdentifierNode, ImplementationNode, ImplementationNodeType, Node, NodeVec,
        StructFieldNode,
    },
};

pub struct StructNode {
    pub identifier: Node<IdentifierNode>,
    pub fields: NodeVec<StructFieldNode>,
    pub implementation: Option<Node<ImplementationNode>>,
    resolved_type: OnceCell<Rc<StructType>>,
}

impl StructNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        fields: NodeVec<StructFieldNode>,
        implementation: Option<Node<ImplementationNode>>,
    ) -> Self {
        Self {
            identifier,
            fields,
            implementation,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let index = scope.get_type_index(self.id()).unwrap();
        scope.nest(ScopeType::Struct(index), |scope| self.check_nested(scope))
    }

    fn check_nested(&self, mut scope: Box<Scope>) -> Box<Scope> {
        let mut scope_names = HashSet::new();
        for field in self.fields.iter() {
            if !scope_names.insert(field.id().clone()) {
                scope.source.print_error(
                    field.identifier.span,
                    &format!("Duplicate struct member `{}`", field.id()),
                    &format!(
                        "struct `{}` already contains a field with this name",
                        self.id()
                    ),
                );
            } else {
                let field_type = field.get_type(&scope).clone();
                scope.add_value(field.id(), field_type);
            }
        }

        if let Some(implementation) = self.implementation.as_ref() {
            return implementation.check(
                scope,
                ImplementationNodeType::Struct,
                self.id(),
                scope_names,
            );
        }

        scope
    }

    pub fn get_type(self: &Rc<Self>, scope: &Scope) -> Rc<StructType> {
        self.resolved_type
            .get_or_init(|| StructType::from(self.clone(), scope))
            .clone()
    }
}

impl Identified for StructNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
