use std::{collections::HashMap, rc::Rc};

use crate::checker::FunctionType;

#[derive(Default, Clone, Debug)]
pub struct InterfaceType {
    pub identifier: String,
    pub methods: HashMap<String, Rc<FunctionType>>,
}
