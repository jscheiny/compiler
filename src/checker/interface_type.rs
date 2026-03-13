use std::{collections::HashMap, rc::Rc};

use crate::checker::FunctionType;

#[derive(Default)]
pub struct InterfaceType {
    pub name: String,
    pub methods: HashMap<String, Rc<FunctionType>>,
}
