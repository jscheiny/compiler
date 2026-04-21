use std::{
    cell::{OnceCell, RefCell},
    collections::HashSet,
    rc::Rc,
};

use crate::{
    checker::{GenericType, Scope, ScopeType, Type},
    parser::{NameNode, Node, TypeNode, TypeParameterListNode, VisitedTypes},
};

pub struct TypeAliasNode {
    pub name: NameNode,
    type_parameters: Option<Node<TypeParameterListNode>>,
    type_def: Node<TypeNode>,
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
                scope = type_parameters.check(scope, type_parameters.span);
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
            let index = scope
                .get_type_index(&self.name)
                .expect("Type should be registered at this point");
            let type_params = self.type_parameters.as_ref().map(|t| t.get_types_map());
            let base_type = self
                .type_def
                .get_type(scope, type_params, initial_visited(index));
            let Some(type_parameters) = self.type_parameters.as_ref() else {
                return base_type;
            };

            Type::Generic(Rc::new(GenericType {
                name: self.name.clone(),
                base_type,
                type_parameters: type_parameters.get_types_list().clone(),
            }))
        })
    }
}

fn initial_visited(index: usize) -> VisitedTypes {
    let mut visited_set = HashSet::new();
    visited_set.insert(index);
    Some(Rc::new(RefCell::new(visited_set)))
}
