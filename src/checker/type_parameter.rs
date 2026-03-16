use std::{collections::HashMap, rc::Rc};

use crate::checker::Type;

pub type TypeBindings = Vec<(Rc<TypeParameter>, Type)>;

// TODO rename TypeParametersMap (maybe unbound?)
pub type TypeParameters = HashMap<String, Rc<TypeParameter>>;

pub struct TypeParameter {
    pub name: String,
}

impl TypeParameter {
    pub fn bind(self: &Rc<Self>, bindings: &TypeBindings) -> Type {
        for (type_parameter, bound_type) in bindings {
            if Rc::ptr_eq(self, type_parameter) {
                return bound_type.clone();
            }
        }

        Type::TypeParameter(self.clone())
    }
}
