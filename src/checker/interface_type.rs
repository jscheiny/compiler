use std::collections::HashMap;

use crate::checker::FunctionType;

#[derive(Default, Clone, Debug)]
pub struct InterfaceType {
    pub identifier: String,
    pub variants: HashMap<String, FunctionType>,
}
