use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type, Types},
    parser::{NameNode, Node, NodeVec, ParameterNode, TypeNode},
};

pub struct FunctionSignatureNode {
    pub name: NameNode,
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
        body_parameters: NodeVec<ParameterNode>,
        return_type: Option<Node<TypeNode>>,
    ) -> Self {
        Self {
            name,
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
        let mut parameters = Vec::new();
        let mut parameter_index_map = HashMap::new();
        for (raw_index, raw_parameter) in self.body_parameters.iter().enumerate() {
            let types = raw_parameter.get_call_type(types);
            for offset in 0..types.len() {
                parameter_index_map.insert(raw_index + offset, raw_index);
            }
            parameters.extend_from_slice(&types);
        }

        let return_type = self
            .return_type
            .as_ref()
            .map_or(Type::Void, |return_type| return_type.get_type(types, None));

        FunctionSignatureNodeData {
            resolved_type: FunctionType::new(parameters, return_type),
            parameter_index_map,
        }
    }
}
