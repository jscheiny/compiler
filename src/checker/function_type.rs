use crate::checker::Type;

#[derive(Clone, Debug)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
}

impl FunctionType {
    pub fn new(input_type: Type, output_type: Type) -> FunctionType {
        FunctionType {
            parameters: vec![input_type],
            return_type: Box::new(output_type),
        }
    }

    pub fn as_static_method(mut self, self_type: Type) -> Type {
        self.parameters.insert(0, self_type.clone());
        Type::Function(FunctionType {
            parameters: self.parameters,
            return_type: Box::new(self_type),
        })
    }
}
