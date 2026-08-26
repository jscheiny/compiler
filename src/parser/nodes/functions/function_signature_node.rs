use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type, Types},
    parser::{NameNode, Node, NodeVec, ParameterNode, TypeNode, TypeParameterListNode},
};

pub struct FunctionSignatureNode {
    pub name: NameNode,
    pub type_parameters: Option<Node<TypeParameterListNode>>,
    pub body_parameters: NodeVec<ParameterNode>,
    pub return_type: Option<Node<TypeNode>>,
    data: OnceCell<FunctionSignatureNodeData>,
}

struct FunctionSignatureNodeData {
    resolved_type: Rc<FunctionType>,
    parameter_index_map: HashMap<usize, usize>,
}

impl FunctionSignatureNode {
    pub fn new(
        name: NameNode,
        type_parameters: Option<Node<TypeParameterListNode>>,
        body_parameters: NodeVec<ParameterNode>,
        return_type: Option<Node<TypeNode>>,
    ) -> Self {
        Self {
            name,
            type_parameters,
            body_parameters,
            return_type,
            data: OnceCell::new(),
        }
    }

    pub fn get_body_parameter_index(&self, scope: &Scope, call_parameter_index: usize) -> usize {
        self.get_data(scope).parameter_index_map[&call_parameter_index]
    }

    pub fn get_type(&self, types: &impl Types) -> Rc<FunctionType> {
        self.get_data(types).resolved_type.clone()
    }

    fn get_data(&self, types: &impl Types) -> &FunctionSignatureNodeData {
        self.data.get_or_init(|| self.init_data(types))
    }

    fn init_data(&self, types: &impl Types) -> FunctionSignatureNodeData {
        let type_params = self
            .type_parameters
            .as_ref()
            .map(|type_parameters| type_parameters.get_types_list().clone());
        let type_params_map = self
            .type_parameters
            .as_ref()
            .map(|type_parameters| type_parameters.get_types_map());

        let mut parameters = Vec::new();
        let mut parameter_index_map = HashMap::new();
        for (raw_index, raw_parameter) in self.body_parameters.iter().enumerate() {
            let types = raw_parameter.get_call_type(types, type_params_map);
            for offset in 0..types.len() {
                parameter_index_map.insert(raw_index + offset, raw_index);
            }
            parameters.extend_from_slice(&types);
        }

        let return_type = self.return_type.as_ref().map_or(Type::Void, |return_type| {
            return_type.get_type(types, type_params_map)
        });

        FunctionSignatureNodeData {
            resolved_type: FunctionType::generic(type_params, parameters, return_type),
            parameter_index_map,
        }
    }
}
