use std::{ops::Deref, rc::Rc};

use crate::checker::{Type, TypeParameter};

pub type TypeBindings = Vec<(Rc<TypeParameter>, Type)>;

#[derive(Clone)]
pub struct TypeParameterList {
    list: Vec<Rc<TypeParameter>>,
}

impl TypeParameterList {
    pub fn new(list: Vec<Rc<TypeParameter>>) -> Self {
        Self { list }
    }

    pub fn get_bindings(&self, bound_types: &[Type]) -> TypeBindings {
        let mut bindings: TypeBindings = vec![];
        for (index, type_parameter) in self.list.iter().enumerate() {
            let bound_type = bound_types.get(index).cloned().unwrap_or(Type::Error);
            bindings.push((type_parameter.clone(), bound_type));
        }

        bindings
    }
}

impl Deref for TypeParameterList {
    type Target = Vec<Rc<TypeParameter>>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
