use std::{collections::HashMap, rc::Rc};

pub type TypeParameters = HashMap<String, Rc<TypeParameter>>;

pub struct TypeParameter {
    pub name: String,
}
