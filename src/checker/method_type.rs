use std::rc::Rc;

use crate::{
    checker::{FunctionType, Type},
    parser::MethodInstanceKind,
};

#[derive(Clone)]
pub struct MethodType {
    pub instance_kind: MethodInstanceKind,
    pub function_type: Rc<FunctionType>,
}

impl MethodType {
    pub fn as_static_method(&self, self_type: Type) -> Type {
        match self.instance_kind {
            MethodInstanceKind::Instance => self.function_type.clone().as_static_method(self_type),
            MethodInstanceKind::Static => Type::Function(self.function_type.clone()),
        }
    }
}
