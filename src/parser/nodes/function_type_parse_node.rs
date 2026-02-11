use std::cell::OnceCell;

use crate::{
    checker::{FunctionType, TypeResolver},
    parser::{ParseNode, ParseNodeVec, TypeParseNode},
};

pub struct FunctionTypeParseNode {
    parameters: ParseNodeVec<TypeParseNode>,
    return_type: Box<ParseNode<TypeParseNode>>,
    resolved_type: OnceCell<FunctionType>,
}

impl FunctionTypeParseNode {
    pub fn new(
        parameters: ParseNodeVec<TypeParseNode>,
        return_type: Box<ParseNode<TypeParseNode>>,
    ) -> Self {
        Self {
            parameters,
            return_type,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn get_type(&self, types: &TypeResolver) -> &FunctionType {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    fn get_type_impl(&self, types: &TypeResolver) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.get_type(types))
            .collect();

        let return_type = Some(Box::new(self.return_type.get_type(types)));

        FunctionType {
            parameters,
            return_type,
        }
    }
}
