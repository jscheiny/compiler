use std::{collections::HashMap, rc::Rc};

pub type Generics<'a> = Option<&'a HashMap<String, Rc<GenericType>>>;

pub struct GenericType {
    pub name: String,
}
