use crate::{
    checker::{Type, TypeParameterMap, Types},
    parser::{Node, TypeListElementNode, VisitedTypes},
};

pub struct TypeListNode {
    pub elements: Vec<Node<TypeListElementNode>>,
}

impl TypeListNode {
    pub fn get_type(
        &self,
        types: &impl Types,
        type_params: Option<&TypeParameterMap>,
        visited: VisitedTypes,
    ) -> Vec<Type> {
        self.elements
            .iter()
            .flat_map(|element| element.get_types(types, type_params, visited.clone()))
            .collect()
    }
}
