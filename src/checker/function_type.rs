use crate::checker::Type;

#[derive(Clone, Debug)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Option<Box<Type>>,
}
