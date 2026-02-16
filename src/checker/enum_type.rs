use std::collections::HashMap;

use crate::checker::{FunctionType, Type};

#[derive(Default, Clone, Debug)]
pub struct EnumType {
    pub identifier: String,
    pub variants: HashMap<String, Option<Type>>,
    pub methods: HashMap<String, EnumMethod>,
}

#[derive(Clone, Debug)]
pub struct EnumMethod {
    pub public: bool,
    pub function_type: FunctionType,
}
