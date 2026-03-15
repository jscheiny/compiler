use std::cell::OnceCell;

use crate::{
    checker::{Scope, ScopeType, Type},
    parser::{NameNode, Node, TypeNode, TypeParameterListNode},
};

pub struct TypeAliasNode {
    pub name: NameNode,
    pub type_parameters: Option<Node<TypeParameterListNode>>,
    pub type_def: Node<TypeNode>,
    resolved_type: OnceCell<Type>,
}

impl TypeAliasNode {
    pub fn new(
        name: NameNode,
        type_parameters: Option<Node<TypeParameterListNode>>,
        type_def: Node<TypeNode>,
    ) -> Self {
        Self {
            name,
            type_parameters,
            type_def,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        scope.nest_with(ScopeType::Type, |mut scope| {
            if let Some(type_parameters) = self.type_parameters.as_ref() {
                scope = type_parameters.check(scope);
            }
            let resolved_type = self.get_type(&scope).clone();
            (scope, resolved_type)
        })
    }

    pub fn check_statement(&self, scope: Box<Scope>) -> Box<Scope> {
        let (mut scope, resolved_type) = self.check(scope);
        scope.add_type(&self.name, resolved_type);
        scope
        // TODO check for recursion
    }

    pub fn get_type(&self, scope: &Scope) -> &Type {
        self.resolved_type.get_or_init(|| {
            let type_parameters = self.type_parameters.as_ref().map(|t| t.get_types());
            self.type_def.get_type(scope, type_parameters)
        })
    }
}
