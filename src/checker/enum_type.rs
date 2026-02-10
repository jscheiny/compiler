use std::collections::HashMap;

use crate::checker::{FunctionType, Type};

pub struct EnumType {
    pub members: HashMap<String, EnumMember>,
}

pub enum EnumMember {
    Variant(Option<Type>),
    Method(FunctionType),
}
