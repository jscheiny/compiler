use crate::{
    checker::{FunctionType, ResolveType, Type, TypeResolver},
    parser::{
        FunctionBodyParseNode, IdentifierParseNode, ParameterParseNode, ParseNode, ParseNodeVec,
        TokenSpan, Traverse, TypeParseNode,
    },
};

pub struct FunctionParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: Option<ParseNode<TypeParseNode>>,
    pub body: ParseNode<FunctionBodyParseNode>,
}

impl Traverse for FunctionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("FunctionDefinition.identifier", self.identifier.span);
        visit("FunctionDefinition.parameters", self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse("FunctionDefintion.parameter", visit);
        }
        if let Some(return_type) = self.return_type.as_ref() {
            return_type.traverse("FunctionDefinition.return", visit);
        }
        self.body.traverse("FunctionDefinition.body", visit);
    }
}

impl ResolveType for FunctionParseNode {
    fn resolve_type(&self, types: &TypeResolver) -> Type {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| match parameter.value.type_def.as_ref() {
                Some(type_def) => type_def.value.resolve_type(types),
                None => Type::Error,
            })
            .collect();

        let return_type = self
            .return_type
            .as_ref()
            .map(|rt| Box::new(rt.value.resolve_type(types)));

        Type::Function(FunctionType {
            parameters,
            return_type,
        })
    }
}
