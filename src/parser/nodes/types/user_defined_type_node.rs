use std::cell::OnceCell;

use crate::{
    checker::{Type, TypeParameterMap, Types},
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

    pub fn get_type(&self, types: &impl Types, type_params: Option<&TypeParameterMap>) -> Type {
        self.resolved_type
            .get_or_init(|| self.init_type(types, type_params))
            .clone()
    }

    fn init_type(&self, types: &impl Types, type_params: Option<&TypeParameterMap>) -> Type {
        let base_type = self.get_base_type(types, type_params);
        if let Some(bound_type_params) = self.bound_type_parameters.as_ref() {
            bind_type(types, &base_type, bound_type_params, type_params)
        } else {
            self.unbound_type(types, base_type)
        }
    }

    fn unbound_type(&self, types: &impl Types, base_type: Type) -> Type {
        match &base_type {
            Type::Generic(generic_type) => {
                types.print_error(
                    self.name.span,
                    "Type parameters required",
                    &format!("type `{base_type}` is generic"),
                );
                let error_bindings = generic_type.type_parameters.get_bindings(&[]);
                generic_type.base_type.bind(types, &error_bindings)
            }
            // TODO check for unbound generic interfaces/structs/enums
            _ => base_type,
        }
    }

    fn get_base_type(&self, types: &impl Types, type_params: Option<&TypeParameterMap>) -> Type {
        let type_parameter = type_params.and_then(|t| t.get(&self.name.value));
        if let Some(type_parameter) = type_parameter {
            return Type::TypeParameter(type_parameter.clone());
        }

        let base_type = types.get_type(&self.name);
        let type_id = types.get_type_id(&self.name);
        if type_id.is_none() {
            types.print_error(
                self.name.span,
                &format!("Unknown type `{}`", self.name),
                "could not find a type with this name",
            );
            return Type::Error;
        }

        // If type_id is some (the type exists), but base_type is none, then the type is still being resolved which indicates
        // a recursive type alias.
        if base_type.is_none() {
            types.print_error(
                self.name.span,
                &format!("Type alias `{}` used recursively", self.name),
                "use of this type creates a circular type alias",
            );
            return Type::Error;
        }

        base_type.unwrap_or(Type::Error)
    }
}

pub fn bind_type(
    types: &impl Types,
    base_type: &Type,
    bound_type_params: &NodeVec<TypeNode>,
    type_params: Option<&TypeParameterMap>,
) -> Type {
    let bound_types = bound_type_params
        .iter()
        .map(|p| p.get_type(types, type_params))
        .collect::<Vec<_>>();

    match base_type {
        Type::Generic(generic_type) => generic_type.bind(types, bound_type_params, &bound_types),
        Type::Enum(_) => todo!("Implement generic binding for enums"),
        Type::Interface(_) => todo!("Implement generic binding for interfaces"),
        Type::Struct(_) => todo!("Implement generic binding for structs"),
        Type::Error => Type::Error,
        _ => panic!("Type encountered that should not be possible? {base_type}"),
    }
}
