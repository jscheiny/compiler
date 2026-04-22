use crate::{
    checker::{Scope, Type, TypeParameterMap},
    parser::{Node, TypeNode, VisitedTypes},
};

pub struct TypeListElementNode {
    pub is_spread: bool,
    pub inner_type: Node<TypeNode>,
}

impl TypeListElementNode {
    pub fn get_types(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Vec<Type> {
        let resolved_type = self.inner_type.get_type(scope, type_params, visited);
        if !self.is_spread {
            return vec![resolved_type];
        }

        if let Type::Tuple(types) = resolved_type {
            return types.to_vec();
        }

        scope.source.print_error(
            self.inner_type.span,
            "Spread type should be a tuple",
            &format!("found type `{}`", resolved_type.format(scope)),
        );

        vec![resolved_type]
    }
}
