use std::collections::HashMap;

use crate::checker::FunctionType;

#[derive(Default, Clone, Debug)]
pub struct InterfaceType {
    pub identifier: String,
    pub methods: HashMap<String, FunctionType>,
}
