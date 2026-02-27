use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{ExpressionNode, Node, NodeVec},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let (scope, function_type) = self.function.check_expected(scope, expected_type);
        let function_type = function_type.as_function(&scope.types);
        check_function_call(scope, function_type.as_ref(), &self.arguments)
    }
}

pub fn check_function_call(
    mut scope: Box<Scope>,
    function_type: Option<&FunctionType>,
    argument_expressions: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    let mut arguments = vec![];
    for (index, argument) in argument_expressions.iter().enumerate() {
        let parameter_type = function_type.and_then(|ft| ft.parameters.get(index));
        let (new_scope, resolved_type) = argument.check_expected(scope, parameter_type);
        arguments.push(resolved_type);
        scope = new_scope;
    }

    if function_type.is_none() {
        return (scope, Type::Error);
    }

    let function_type = function_type.unwrap();
    if arguments.len() > function_type.parameters.len() {
        scope.source.print_error(
            argument_expressions.span,
            "Too many arguments",
            &format!(
                "expected at most {} argument{} but received {}",
                function_type.parameters.len(),
                if function_type.parameters.len() == 1 {
                    ""
                } else {
                    "s"
                },
                arguments.len()
            ),
        );
    }

    let parameters_and_arguments = function_type.parameters.iter().zip(&arguments);
    for (index, (parameter, argument)) in parameters_and_arguments.enumerate() {
        if !argument.is_assignable_to(parameter, &scope.types) {
            scope.source.print_error(
                argument_expressions[index].span,
                "Argument not assignable to parameter type",
                &format!(
                    "expected type `{}`, found type `{}`",
                    parameter.format(&scope.types),
                    argument.format(&scope.types),
                ),
            );
        }
    }

    if arguments.len() < function_type.parameters.len() {
        let remaining_parameters = &function_type.parameters[arguments.len()..];
        let result_type = Type::Function(FunctionType {
            parameters: remaining_parameters.to_vec(),
            return_type: function_type.return_type.clone(),
        });
        (scope, result_type)
    } else {
        (scope, *function_type.return_type.clone())
    }
}
