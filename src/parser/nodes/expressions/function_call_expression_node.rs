use std::rc::Rc;

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
    scope: Box<Scope>,
    function_span: TokenSpan,
    left_type: Type,
    arguments: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    let function_type = left_type.to_function(&scope);
    match function_type {
        Some(function_type) => check_valid_function_call(scope, function_type, arguments),
        None => check_invalid_function_call(scope, function_span, left_type, arguments),
    }
}

fn check_invalid_function_call(
    mut scope: Box<Scope>,
    function_span: TokenSpan,
    left_type: Type,
    arguments: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    if !left_type.is_error() {
        scope.source.print_error(
            function_span,
            "Cannot use value as a function",
            &format!(
                "type `{}` is not usable as a function",
                left_type.format(&scope)
            ),
        );
    }

    // Check parameter types without expected argument types
    for argument in arguments.iter() {
        scope = argument.check(scope).0;
    }

    (scope, Type::Error)
}

fn check_valid_function_call(
    mut scope: Box<Scope>,
    function_type: Rc<FunctionType>,
    arguments: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    if arguments.len() > function_type.parameters.len() {
        scope.source.print_error(
            arguments.span,
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

    for (index, argument) in arguments.iter().enumerate() {
        let parameter_type = function_type.parameters.get(index);
        let (new_scope, argument_type) = argument.check_expected(scope, parameter_type);
        scope = new_scope;

        let Some(parameter_type) = parameter_type else {
            continue;
        };

        if !argument_type.is_assignable_to(parameter_type, &scope) {
            scope.source.print_error(
                arguments[index].span,
                "Argument not assignable to parameter type",
                &format!(
                    "expected type `{}`, found type `{}`",
                    parameter_type.format(&scope),
                    argument_type.format(&scope),
                ),
            );
        }
    }

    if arguments.len() < function_type.parameters.len() {
        let remaining_parameters = &function_type.parameters[arguments.len()..];
        let result_type = Type::Function(FunctionType::new(
            remaining_parameters.to_vec(),
            *function_type.return_type.clone(),
        ));
        (scope, result_type)
    } else {
        (scope, *function_type.return_type.clone())
    }
}
