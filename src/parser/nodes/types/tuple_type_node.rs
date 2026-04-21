use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{Scope, Type, TypeParameterMap},
    parser::{TypeListNode, VisitedTypes},
};

pub struct TupleTypeNode {
    fields: TypeListNode,
    resolved_type: OnceCell<Type>,
}

impl TupleTypeNode {
    pub fn new(fields: TypeListNode) -> Self {
        Self {
            fields,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Type {
        self.resolved_type
            .get_or_init(|| self.init_type(scope, type_params, visited))
            .clone()
    }

    fn init_type(
        &self,
        scope: &Scope,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Type {
        let fields = self.fields.get_type(scope, type_params, visited);
        Type::Tuple(Rc::new(fields))
    }
}
