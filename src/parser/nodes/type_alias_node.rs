use std::{cell::OnceCell, collections::HashSet, rc::Rc};

use crate::{
    checker::{GenericType, Generics, Scope, ScopeType, Type},
    parser::{NameNode, Node, NodeVec, TypeNode, TypeParameterNode},
};

pub struct TypeAliasNode {
    pub name: NameNode,
    pub type_parameters: Option<NodeVec<TypeParameterNode>>,
    pub type_def: Node<TypeNode>,
    resolved_type: OnceCell<Type>,
}

impl TypeAliasNode {
    pub fn new(
        name: NameNode,
        type_parameters: Option<NodeVec<TypeParameterNode>>,
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
        scope.nest_with(ScopeType::Type, |scope| {
            let scope = self.check_type_parameters(scope);
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

    pub fn check_type_parameters(&self, mut scope: Box<Scope>) -> Box<Scope> {
        let Some(type_parameters) = self.type_parameters.as_ref() else {
            return scope;
        };

        let mut type_parameter_names = HashSet::new();
        for type_param in type_parameters.iter() {
            if !type_parameter_names.insert(&type_param.name.value) {
                scope.source.print_error(
                    type_param.name.span,
                    &format!("Duplicate type parameter name `{}`", type_param.name),
                    "type alias already contains a type parameter with this name",
                );
            } else {
                let generic_type = Rc::new(GenericType {
                    name: type_param.name.clone(),
                });
                scope.add_type(&type_param.name, Type::Generic(generic_type));
            }
            // if type_parameter_names.insert(parameter.na)
        }

        scope
    }

    pub fn get_type(&self, scope: &Scope, generics: Generics<'_>) -> &Type {
        self.resolved_type
            .get_or_init(|| self.type_def.get_type(scope, generics))
    }
}
