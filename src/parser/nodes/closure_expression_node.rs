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
        let function_type = expected_type.and_then(|t| get_function_type(t, types));
        let scope = scope.derive(ScopeType::Closure);
        let (scope, parameters) = self.check_parameters(function_type.as_ref(), types, scope);
        let (scope, return_type) = self.body.check(types, scope, None);
        let result_type = Type::Function(FunctionType {
            parameters,
            return_type: Box::new(return_type),
        });
        (scope.parent(), result_type)
    }

    fn check_parameters(
        &self,
        expected_type: Option<&FunctionType>,
        types: &TypeResolver,
        mut scope: Box<Scope>,
    ) -> (Box<Scope>, Vec<Type>) {
        let parameter_types = self
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                if let Some(parameter) = parameter {
                    let parameter_type = get_parameter_type(parameter, index, expected_type, types);
                    scope.add(parameter.id(), parameter_type.clone());
                    parameter_type
                } else {
                    Type::Error
                }
            })
            .collect::<Vec<_>>();

        (scope, parameter_types)
    }
}

fn get_parameter_type(
    parameter: &Node<ClosureParameterExpressionNode>,
    index: usize,
    expected_type: Option<&FunctionType>,
    types: &TypeResolver,
) -> Type {
    let expected_type = expected_type.and_then(|ft| ft.parameters.get(index));
    if let Some(given_type) = parameter.parameter_type.as_ref() {
        given_type.get_type(types)
    } else if let Some(expected_type) = expected_type {
        expected_type.clone()
    } else {
        println!(
            "Type error: Could not infer type of closure parameter `{}`",
            parameter.id()
        );
        Type::Error
    }
}
