use std::rc::Rc;

use crate::checker::{Type, TypeParameterBindings, Types};

pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
}

impl FunctionType {
    pub fn new(parameters: Vec<Type>, return_type: Type) -> Rc<Self> {
        Rc::new(Self {
            parameters,
            return_type: Box::new(return_type),
        })
    }

    pub fn simple(input_type: Type, output_type: Type) -> Rc<Self> {
        Self::new(vec![input_type], output_type)
    }

    pub fn as_static_method(self: Rc<Self>, self_type: Type) -> Type {
        let mut parameters = self.parameters.clone();
        parameters.insert(0, self_type.clone());
        Type::Function(Self::new(parameters, *self.return_type.clone()))
    }

    pub fn bind(&self, types: &impl Types, bindings: &TypeParameterBindings) -> Rc<Self> {
        Self::new(
            self.parameters
                .iter()
                .map(|param| param.bind(types, bindings))
                .collect(),
            self.return_type.bind(types, bindings),
        )
    }
}
