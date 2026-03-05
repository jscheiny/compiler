use std::collections::HashSet;

use crate::{
    checker::{FunctionType, Scope},
    parser::{FunctionBodyNode, FunctionSignatureNode, Identified, Node},
};

pub struct FunctionNode {
    pub signature: FunctionSignatureNode,
    body: Node<FunctionBodyNode>,
}

impl FunctionNode {
    pub fn new(signature: FunctionSignatureNode, body: Node<FunctionBodyNode>) -> Self {
        Self { signature, body }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let return_type = &self.get_type(&scope).return_type;
        scope.nest_fn(return_type, |scope| {
            let scope = self.check_params(scope);
            match &self.body.value {
                FunctionBodyNode::Expression(expression) => {
                    let (scope, resolved_type) =
                        expression.check_expected(scope, Some(return_type));
                    if !resolved_type.is_assignable_to(return_type, &scope) {
                        scope.source.print_error(
                            self.body.span,
                            &format!(
                                "Function must return value of type `{}`",
                                return_type.format(&scope)
                            ),
                            &format!("found type: `{}`", resolved_type.format(&scope)),
                        );
                    }
                    scope
                }
                FunctionBodyNode::Block(block) => block.check(scope, Some(return_type)).0,
            }
        })
    }

    fn check_params(&self, mut scope: Box<Scope>) -> Box<Scope> {
        let mut param_names = HashSet::new();
        for param in self.signature.parameters.iter() {
            if param_names.contains(param.id()) {
                scope.source.print_error(
                    param.identifier.span,
                    &format!("Duplicate parameter name `{}`", param.id()),
                    "function already contains a parameter with this name",
                );
            } else {
                param_names.insert(param.id().clone());
                scope.add_value(param.id(), param.get_type(&scope).clone());
            }
        }
        scope
    }

    pub fn get_type(&self, scope: &Scope) -> &FunctionType {
        self.signature.get_type(scope)
    }
}

impl Identified for FunctionNode {
    fn id(&self) -> &String {
        self.signature.identifier.id()
    }
}
