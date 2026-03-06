use std::rc::Rc;

use crate::checker::Type;

#[derive(Debug)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
}

impl FunctionType {
    pub fn new(input_type: Type, output_type: Type) -> Rc<FunctionType> {
        Rc::new(FunctionType {
            parameters: vec![input_type],
            return_type: Box::new(output_type),
        })
    }

    pub fn as_static_method(self: Rc<Self>, self_type: Type) -> Type {
        let mut parameters = self.parameters.clone();
        parameters.insert(0, self_type.clone());
        Type::Function(Rc::new(FunctionType {
            parameters: parameters,
            return_type: Box::new(self_type),
        }))
    }
}
