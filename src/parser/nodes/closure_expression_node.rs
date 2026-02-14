use crate::{
    checker::{FunctionType, Scope, ScopeType, Type, TypeResolver},
    parser::{ClosureParameterExpressionNode, ExpressionNode, Identified, Node, get_function_type},
};

pub struct ClosureExpressionNode {
    pub parameters: Vec<Option<Node<ClosureParameterExpressionNode>>>,
    pub body: Box<Node<ExpressionNode>>,
}

impl ClosureExpressionNode {
    pub fn check(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        let function_type =
            expected_type.and_then(|expected_type| get_function_type(expected_type, types));
        if let Some(function_type) = function_type {
            self.check_with_expected(types, scope, &function_type)
        } else {
            self.check_raw(types, scope)
        }
    }

    fn check_with_expected(
        &self,
        _types: &TypeResolver,
        _scope: Box<Scope>,
        _expected_type: &FunctionType,
    ) -> (Box<Scope>, Type) {
        todo!("Implement type checking with inference");
        // (scope, Type::Error)
    }

    fn check_raw(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let mut scope = scope.derive(ScopeType::Closure);
        for parameter in self.parameters.iter().flatten() {
            let parameter_type = if let Some(parameter_type) = parameter.parameter_type.as_ref() {
                parameter_type.get_type(types)
            } else {
                println!(
                    "Type error: Could not infer type of closure parameter `{}`",
                    parameter.id()
                );
                Type::Error
            };
            scope.add(parameter.id(), parameter_type);
        }

        let (scope, return_type) = self.body.check(types, scope, None);
        (scope.parent(), return_type)
    }
}
