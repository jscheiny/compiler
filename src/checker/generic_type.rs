use crate::{
    checker::{Scope, Type, TypeParameterList},
    parser::{NodeVec, TypeNode},
};

pub struct GenericType {
    pub name: String,
    pub base_type: Type,
    pub type_parameters: TypeParameterList,
}

impl GenericType {
    pub fn bind(
        &self,
        scope: &Scope,
        bound_type_params: &NodeVec<TypeNode>,
        bound_types: &[Type],
    ) -> Type {
        if bound_types.len() != self.type_parameters.len() {
            scope.source.print_error(
                bound_type_params.span,
                "Mismatched type parameters",
                &format!(
                    "expected {} types, found {}",
                    self.type_parameters.len(),
                    bound_type_params.len()
                ),
            );
        }

        let bindings = self.type_parameters.get_bindings(bound_types);
        self.base_type.bind(scope, &bindings)
    }
}
