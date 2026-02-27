use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{ExpressionNode, Node, NodeVec, TokenSpan},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let (scope, function_type) = self.function.check_expected(scope, expected_type);
        check_function_call(scope, self.function.span, function_type, &self.arguments)
    }
}

pub fn check_function_call(
    mut scope: Box<Scope>,
    function_span: TokenSpan,
    left_type: Type,
    argument_expressions: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    let function_type = left_type.clone().as_function(&scope.types);
    let mut arguments = vec![];
    for (index, argument) in argument_expressions.iter().enumerate() {
        let parameter_type = function_type
            .as_ref()
            .and_then(|ft| ft.parameters.get(index));
        let (new_scope, resolved_type) = argument.check_expected(scope, parameter_type);
        arguments.push(resolved_type);
        scope = new_scope;
    }

    if function_type.is_none() {
        scope.source.print_error(
            function_span,
            "Cannot use value as a function",
            &format!(
                "type `{}` is not usable as a function",
                left_type.format(&scope.types)
            ),
        );
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
