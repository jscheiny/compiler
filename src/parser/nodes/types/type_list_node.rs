use crate::{
    checker::{Scope, Type, TypeParameterMap},
    parser::{Node, TypeListElementNode},
};

pub struct TypeListNode {
    pub elements: Vec<Node<TypeListElementNode>>,
}

impl TypeListNode {
    pub fn get_type(&self, scope: &Scope, type_params: Option<&TypeParameterMap>) -> Vec<Type> {
        self.elements
            .iter()
            .flat_map(|element| element.get_types(scope, type_params))
            .collect()
    }
}
