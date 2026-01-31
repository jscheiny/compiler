use crate::parser::{
    BlockParseNode, ExpressionParseNode, IdentifierParseNode, ParseNode, ParseNodeVec, TokenSpan,
    Traverse, TypeParseNode,
};

pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionDefintionParseNode>,
}

impl Traverse for MethodParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.function.traverse("Method.function", visit);
    }
}

pub struct FunctionDefintionParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: Option<ParseNode<TypeParseNode>>,
    pub body: ParseNode<FunctionBodyParseNode>,
}

impl Traverse for FunctionDefintionParseNode {
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

pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}

impl Traverse for FunctionBodyParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Expression(node) => node.traverse(visit),
            Self::Block(node) => node.traverse(visit),
        }
    }
}

pub struct ParameterParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl Traverse for ParameterParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Parameter.identifier", self.identifier.span);
        self.type_def.traverse("Parameter.type", visit);
    }
}
