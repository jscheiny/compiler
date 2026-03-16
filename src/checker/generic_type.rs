use std::rc::Rc;

use crate::checker::{Type, TypeParameter};

pub struct GenericType {
    pub name: String,
    pub base_type: Type,
    pub type_parameters: Vec<Rc<TypeParameter>>,
}
