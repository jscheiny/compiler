use crate::{
    checker::{FunctionType, Scope, ScopeType, Type, TypeResolver},
    parser::{ClosureParameterExpressionNode, ExpressionNode, Identified, Node},
};

pub struct ClosureExpressionNode {
    pub parameters: Vec<Option<Node<ClosureParameterExpressionNode>>>,
    pub body: Box<Node<ExpressionNode>>,
}

impl ClosureExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let function_type = get_expected_type(expected_type, &scope.types);
        scope.nest_with(ScopeType::Closure, |scope| {
            let (scope, parameters) = self.check_parameters(function_type.as_ref(), scope);
            let expected_return_type = function_type.map(|t| t.return_type);
            let (scope, return_type) = self
                .body
                .check_expected(scope, expected_return_type.as_deref());

            let some_parameter_is_error_type =
                parameters.iter().any(|parameter| parameter.is_error());
            if some_parameter_is_error_type || return_type.is_error() {
                return (scope, Type::Error);
            }

            let result_type = Type::Function(FunctionType {
                parameters,
                return_type: Box::new(return_type),
            });
            (scope, result_type)
        })
    }

    fn check_parameters(
        &self,
        expected_type: Option<&FunctionType>,
        mut scope: Box<Scope>,
    ) -> (Box<Scope>, Vec<Type>) {
        let parameter_types = self
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                if let Some(parameter) = parameter {
                    let parameter_type =
                        get_parameter_type(parameter, index, expected_type, &scope);
                    scope.add_or(parameter.id(), parameter_type.clone(), |scope| {
                        scope.source.print_error(
                            parameter.identifier.span,
                            &format!("Duplicate parameter name `{}`", parameter.id()),
                            "closure already contains a parameter with this name",
                        );
                    });
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
    scope: &Scope,
) -> Type {
    let expected_type = expected_type.and_then(|ft| ft.parameters.get(index));
    if let Some(given_type) = parameter.parameter_type.as_ref() {
        given_type.get_type(&scope.types, &scope.source)
    } else if let Some(expected_type) = expected_type {
        expected_type.clone()
    } else {
        scope.source.print_error(
            parameter.span,
            "Parameter type is ambiguous",
            "could not infer type of parameter",
        );
        Type::Error
    }
}

fn get_expected_type(t: Option<&Type>, types: &TypeResolver) -> Option<FunctionType> {
    match t {
        Some(Type::Function(function_type)) => Some(function_type.clone()),
        Some(Type::Reference(index)) => get_expected_type(types.get_type(*index).as_ref(), types),
        _ => None,
    }
}
