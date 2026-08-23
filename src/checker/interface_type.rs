use std::collections::HashMap;

use crate::checker::MethodType;

#[derive(Default)]
pub struct InterfaceType {
    pub name: String,
    pub methods: HashMap<String, MethodType>,
}
