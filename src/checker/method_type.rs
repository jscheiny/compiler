use std::rc::Rc;

use crate::{checker::FunctionType, parser::MethodInstanceKind};

#[derive(Clone)]
pub struct MethodType {
    pub instance_kind: MethodInstanceKind,
    pub function_type: Rc<FunctionType>,
}
