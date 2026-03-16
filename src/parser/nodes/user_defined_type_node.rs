use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{GenericType, Scope, Type, TypeBindings, TypeParameters},
    parser::{NameNode, NodeVec, TypeNode},
};

pub struct UserDefinedTypeNode {
    pub name: NameNode,
    bound_type_parameters: Option<NodeVec<TypeNode>>,
    resolved_type: OnceCell<Type>,
}

impl UserDefinedTypeNode {
    pub fn new(name: NameNode, bound_type_parameters: Option<NodeVec<TypeNode>>) -> Self {
        Self {
            name,
            bound_type_parameters,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, scope: &Scope, type_params: Option<&TypeParameters>) -> Type {
        self.resolved_type
            .get_or_init(|| self.init_type(scope, type_params))
            .clone()
    }

    fn init_type(&self, scope: &Scope, type_params: Option<&TypeParameters>) -> Type {
        let base_type = self.get_base_type(scope, type_params);
        if let Some(bound_type_params) = self.bound_type_parameters.as_ref() {
            self.bind_type(scope, base_type, bound_type_params, type_params)
        } else {
            self.unbound_type(scope, base_type)
        }
    }

    fn bind_type(
        &self,
        scope: &Scope,
        base_type: Type,
        bound_type_params: &NodeVec<TypeNode>,
        type_params: Option<&TypeParameters>,
    ) -> Type {
        let bound_types = bound_type_params
            .iter()
            .map(|p| p.get_type(scope, type_params))
            .collect::<Vec<_>>();

        match base_type.deref(scope) {
            Type::Generic(generic_type) => {
                self.bind_generic_type_alias(scope, generic_type, bound_type_params, &bound_types)
            }
            Type::Enum(_) => todo!("Implement generic binding for enums"),
            Type::Interface(_) => todo!("Implement generic binding for interfaces"),
            Type::Struct(_) => todo!("Implement generic binding for structs"),
            Type::Error => Type::Error,
            _ => {
                panic!(
                    "Type encountered that should not be possible? {}",
                    base_type.format(scope)
                )
            }
        }
    }

    fn bind_generic_type_alias(
        &self,
        scope: &Scope,
        generic_type: Rc<GenericType>,
        bound_type_params: &NodeVec<TypeNode>,
        bound_types: &Vec<Type>,
    ) -> Type {
        // TODO better spans based on less than/greater than diff
        if bound_types.len() != generic_type.parameter_list.len() {
            scope.source.print_error(
                bound_type_params.span,
                "Mismatched type parameters",
                &format!(
                    "expected {} types, found {}",
                    generic_type.parameter_list.len(),
                    bound_type_params.len()
                ),
            );
        }

        let bindings = get_bindings(&generic_type, bound_types);
        generic_type.base_type.bind(scope, &bindings)
    }

    fn unbound_type(&self, scope: &Scope, base_type: Type) -> Type {
        match base_type.deref(scope) {
            Type::Generic(generic_type) => {
                scope.source.print_error(
                    self.name.span,
                    "Type parameters required",
                    &format!("type `{}` is generic", base_type.format(scope)),
                );
                let error_bindings = get_bindings(&generic_type, &vec![]);
                generic_type.base_type.bind(scope, &error_bindings)
            }
            // TODO check for unbound generic interfaces/structs/enums
            _ => base_type,
        }
    }

    fn get_base_type(&self, scope: &Scope, type_params: Option<&TypeParameters>) -> Type {
        let type_parameter = type_params.and_then(|t| t.get(&self.name.value));
        if let Some(type_parameter) = type_parameter {
            return Type::TypeParameter(type_parameter.clone());
        }

        let index = scope.get_type_index(&self.name);
        if let Some(index) = index {
            return Type::Reference(index);
        }

        scope.source.print_error(
            self.name.span,
            &format!("Unknown type `{}`", self.name),
            "could not find a type with this name",
        );
        Type::Error
    }
}

fn get_bindings(generic_type: &GenericType, bound_types: &Vec<Type>) -> TypeBindings {
    let mut bindings: TypeBindings = vec![];
    for (index, type_parameter) in generic_type.parameter_list.iter().enumerate() {
        let bound_type = bound_types.get(index).cloned().unwrap_or(Type::Error);
        bindings.push((type_parameter.clone(), bound_type));
    }
    bindings
}
