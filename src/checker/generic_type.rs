use std::rc::Rc;

use crate::checker::{Type, TypeParameter, TypeParameterMap};

pub struct GenericType {
    pub name: String,
    pub base_type: Type,
    pub type_parameter_list: Vec<Rc<TypeParameter>>,
    pub type_parameter_map: TypeParameterMap,
}
