use std::{cmp::min, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{ExpressionNode, Node, NodeVec, SpreadNode, TokenSpan},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let (scope, function_type) = self.function.check_expected(scope, expected_type);
        check_function_call(scope, self.function.span, &function_type, &self.arguments)
    }
}

pub fn check_function_call(
    scope: Box<Scope>,
    function_span: TokenSpan,
    left_type: &Type,
    arguments: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    let function_type = left_type.to_function(&scope);
    match function_type {
        Some(function_type) => check_valid_function_call(scope, &function_type, arguments),
        None => check_invalid_function_call(scope, function_span, left_type, arguments),
    }
}

fn check_invalid_function_call(
    mut scope: Box<Scope>,
    function_span: TokenSpan,
    left_type: &Type,
    arguments: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    if !left_type.is_error() {
        scope.source.print_error(
            function_span,
            "Cannot use value as a function",
            &format!("type `{}` is not usable as a function", left_type),
        );
    }

    // Check parameter types without expected argument types
    for argument in arguments.iter() {
        if let ExpressionNode::Spread(spread_argument) = &argument.value {
            scope = spread_argument.check_valid(scope, None).0;
        } else {
            scope = argument.check(scope).0;
        }
    }

    (scope, Type::Error)
}

fn check_valid_function_call(
    mut scope: Box<Scope>,
    function_type: &Rc<FunctionType>,
    arguments: &NodeVec<ExpressionNode>,
) -> (Box<Scope>, Type) {
    let mut argument_count = 0;
    for argument in arguments.iter() {
        if let ExpressionNode::Spread(spread_node) = &argument.value {
            (scope, argument_count) = check_spread_arg(
                scope,
                spread_node,
                argument.span,
                function_type,
                argument_count,
            );
        } else {
            let parameter_type = function_type.parameters.get(argument_count);
            let (new_scope, argument_type) = argument.check_expected(scope, parameter_type);
            scope = new_scope;
            argument_count += 1;

            let Some(parameter_type) = parameter_type else {
                continue;
            };

            if !argument_type.is_assignable_to(parameter_type, &scope) {
                scope.source.print_error(
                    argument.span,
                    "Argument not assignable to parameter type",
                    &format!(
                        "expected type `{}`, found type `{}`",
                        parameter_type, argument_type,
                    ),
                );
            }
        }
    }

    if argument_count > function_type.parameters.len() {
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
                argument_count
            ),
        );
    }

    if argument_count < function_type.parameters.len() {
        let remaining_parameters = &function_type.parameters[argument_count..];
        let result_type = Type::Function(FunctionType::new(
            remaining_parameters.to_vec(),
            *function_type.return_type.clone(),
        ));
        (scope, result_type)
    } else {
        (scope, *function_type.return_type.clone())
    }
}

fn check_spread_arg(
    scope: Box<Scope>,
    node: &SpreadNode,
    span: TokenSpan,
    function_type: &Rc<FunctionType>,
    parameter_index: usize,
) -> (Box<Scope>, usize) {
    let expected_types = function_type
        .parameters
        .iter()
        .skip(parameter_index)
        .cloned()
        .collect::<Vec<_>>();
    let expected_type = Some(Type::Tuple(Rc::new(expected_types)));

    let (scope, spread_type) = node.check_valid(scope, expected_type.as_ref());
    for (offset, argument_type) in spread_type.iter().enumerate() {
        let parameter_type = function_type.parameters.get(parameter_index + offset);

        let Some(parameter_type) = parameter_type else {
            break;
        };

        if !argument_type.is_assignable_to(parameter_type, &scope) {
            let end_index = min(
                parameter_index + spread_type.len(),
                function_type.parameters.len(),
            );
            let expected_type = Type::Tuple(Rc::new(
                function_type.parameters[parameter_index..end_index].to_vec(),
            ));

            scope.source.print_error(
                span,
                "Spread argument not assignable to parameter types",
                &format!(
                    "expected type `{}`, found type `{}`",
                    expected_type,
                    Type::Tuple(spread_type.clone()),
                ),
            );
            break;
        }
    }

    (scope, parameter_index + spread_type.len())
}
