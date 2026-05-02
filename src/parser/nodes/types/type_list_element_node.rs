use crate::{
    checker::{Type, TypeParameterMap, Types},
    parser::{Node, TypeNode, VisitedTypes},
};

pub struct TypeListElementNode {
    pub is_spread: bool,
    pub inner_type: Node<TypeNode>,
}

impl TypeListElementNode {
    pub fn get_types(
        &self,
        types: &impl Types,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Vec<Type> {
        let resolved_type = self.inner_type.get_type(types, type_params, visited);
        if !self.is_spread {
            return vec![resolved_type];
        }

        if let Type::Tuple(types) = resolved_type {
            return types.to_vec();
        }

        types.print_error(
            self.inner_type.span,
            "Spread type should be a tuple",
            &format!("found type `{}`", resolved_type),
        );

        vec![resolved_type]
    }
}
