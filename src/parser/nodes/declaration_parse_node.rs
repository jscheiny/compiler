use crate::parser::{
    ExpressionParseNode, IdentifierParseNode, ParseNode, TokenSpan, Traverse, TypeParseNode,
};

pub struct DeclarationParseNode {
    pub mutable: bool,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub type_def: Option<ParseNode<TypeParseNode>>,
    pub initializer: Option<ParseNode<ExpressionParseNode>>,
}

impl Traverse for DeclarationParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Declaration.identifier", self.identifier.span);
        if let Some(type_def) = self.type_def.as_ref() {
            type_def.traverse("Declaration.type", visit);
        }
        if let Some(expression) = self.initializer.as_ref() {
            expression.traverse("Declaration.expression", visit);
        }
    }
}
