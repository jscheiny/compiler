use crate::checker::Type;

#[derive(Clone)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Option<Box<Type>>,
}
