use crate::{
    checker::{FunctionType, TypeResolver, ResolveType, Type},
    parser::{ParseNode, ParseNodeVec, TokenSpan, Traverse, TypeParseNode},
};

pub struct FunctionTypeParseNode {
    pub parameters: ParseNodeVec<TypeParseNode>,
    pub return_type: Box<ParseNode<TypeParseNode>>,
}

impl Traverse for FunctionTypeParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("FunctionType.parameters", self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse("FunctionType.parameter", visit);
        }
        self.return_type.traverse("FunctionType.return_type", visit);
    }
}

impl ResolveType for FunctionTypeParseNode {
    fn resolve_types(&self, types: &TypeResolver) -> Type {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|p| p.value.resolve_types(types))
            .collect();

        let return_type = Box::new(self.return_type.value.resolve_types(types));

        Type::Function(FunctionType {
            parameters,
            return_type,
        })
    }
}
