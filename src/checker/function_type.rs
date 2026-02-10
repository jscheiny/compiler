use crate::checker::Type;

pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Option<Box<Type>>,
}
