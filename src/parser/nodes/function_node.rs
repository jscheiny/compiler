use std::{cell::OnceCell, collections::HashSet};

use crate::{
    checker::{FunctionType, Scope, Type, TypeResolver},
    lexer::SourceCode,
    parser::{
        FunctionBodyNode, Identified, IdentifierNode, Node, NodeVec, ParameterNode, TypeNode,
    },
};

pub struct FunctionNode {
    pub identifier: Node<IdentifierNode>,
    pub parameters: NodeVec<ParameterNode>,
    pub return_type: Option<Node<TypeNode>>,
    pub body: Node<FunctionBodyNode>,
    resolved_type: OnceCell<FunctionType>,
}

impl FunctionNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        parameters: NodeVec<ParameterNode>,
        return_type: Option<Node<TypeNode>>,
        body: Node<FunctionBodyNode>,
    ) -> Self {
        Self {
            identifier,
            parameters,
            return_type,
            body,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let return_type = &self.get_type(&scope.types, &scope.source).return_type;
        scope.nest_fn(return_type, |scope| {
            let scope = self.check_params(scope);
            match &self.body.value {
                FunctionBodyNode::Expression(expression) => {
                    let (scope, resolved_type) =
                        expression.check_expected(scope, Some(return_type));
                    if !resolved_type.is_assignable_to(return_type, &scope.types) {
                        scope.source.print_type_error(
                            self.body.span,
                            &format!(
                                "Function must return value of type `{}`",
                                return_type.format(&scope.types)
                            ),
                            &format!("found type: `{}`", resolved_type.format(&scope.types)),
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
        for param in self.parameters.iter() {
            if param_names.contains(param.id()) {
                scope.source.print_type_error(
                    param.identifier.span,
                    &format!("Duplicate parameter name `{}`", param.id()),
                    "function already contains a parameter with this name",
                );
            } else {
                param_names.insert(param.id().clone());
                scope.add(
                    param.id(),
                    param.get_type(&scope.types, &scope.source).clone(),
                );
            }
        }
        scope
    }

    pub fn get_type(&self, types: &TypeResolver, source: &SourceCode) -> &FunctionType {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(types, source))
    }

    fn get_type_impl(&self, types: &TypeResolver, source: &SourceCode) -> FunctionType {
        let parameters = self
            .parameters
            .value
            .iter()
            .map(|parameter| parameter.get_type(types, source))
            .cloned()
            .collect();

        let return_type = self.return_type.as_ref().map_or(Type::Void, |return_type| {
            return_type.get_type(types, source)
        });

        FunctionType {
            parameters,
            return_type: Box::new(return_type),
        }
    }
}

impl Identified for FunctionNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
