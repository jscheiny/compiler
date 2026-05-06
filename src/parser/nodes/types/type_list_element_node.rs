use crate::{
    checker::{Type, TypeParameterMap, Types},
    parser::{Node, TokenSpan, TypeNode},
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
    ) -> Vec<Type> {
        let resolved_type = self.inner_type.get_type(types, type_params);
        get_maybe_spread_type(
            types,
            self.is_spread,
            resolved_type,
            Some(self.inner_type.span),
        )
    }
}

pub fn get_maybe_spread_type(
    types: &impl Types,
    is_spread: bool,
    base_type: Type,
    base_type_span: Option<TokenSpan>,
) -> Vec<Type> {
    if !is_spread {
        return vec![base_type];
    }

    if let Type::Tuple(types) = base_type {
        return types.to_vec();
    }

    if let Some(base_type_span) = base_type_span
        && !base_type.is_error()
    {
        types.print_error(
            base_type_span,
            "Spread type should be a tuple",
            &format!("found type `{}`", base_type),
        );
    }

    vec![base_type]
}
