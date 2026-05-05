use std::{cell::OnceCell, rc::Rc};

use crate::{
    checker::{Type, TypeParameterMap, Types},
    parser::TypeListNode,
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

    pub fn get_type(&self, types: &impl Types, type_params: Option<&TypeParameterMap>) -> Type {
        self.resolved_type
            .get_or_init(|| self.init_type(types, type_params))
            .clone()
    }

    fn init_type(&self, types: &impl Types, type_params: Option<&TypeParameterMap>) -> Type {
        let fields = self.fields.get_type(types, type_params);
        Type::Tuple(Rc::new(fields))
    }
}
