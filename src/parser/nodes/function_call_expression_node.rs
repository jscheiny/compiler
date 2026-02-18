use crate::{
    checker::{FunctionType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, NodeVec},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, function_type) = self.function.check(types, scope);
        let function_type = function_type.as_function(types);
        let (scope, arguments) = self.get_args(types, scope, function_type.as_ref());

        if let Some(function_type) = function_type {
            (scope, self.check_args(function_type, &arguments, types))
        } else {
            (scope, Type::Error)
        }
    }

    fn get_args(
        &self,
        types: &TypeResolver,
        mut scope: Box<Scope>,
        function_type: Option<&FunctionType>,
    ) -> (Box<Scope>, Vec<Type>) {
        let mut result = vec![];
        for (index, argument) in self.arguments.iter().enumerate() {
            let parameter_type = function_type.and_then(|ft| ft.parameters.get(index));
            let (new_scope, resolved_type) = argument.check_expected(types, scope, parameter_type);
            result.push(resolved_type);
            scope = new_scope;
        }
        (scope, result)
    }

    fn check_args(
        &self,
        function_type: FunctionType,
        arguments: &[Type],
        types: &TypeResolver,
    ) -> Type {
        if arguments.len() > function_type.parameters.len() {
            println!(
                "Type error: Call expected at most {} arguments but recieved {}",
                function_type.parameters.len(),
                arguments.len()
            );
        }

        for (index, (parameter, argument)) in
            function_type.parameters.iter().zip(arguments).enumerate()
        {
            if !argument.is_assignable_to(parameter, types) {
                println!(
                    "Type error: Arg {} of type `{}` cannot be assigned to parameter of type `{}`",
                    index,
                    argument.format(types),
                    parameter.format(types),
                );
            }
        }

        if arguments.len() < function_type.parameters.len() {
            let remaining_parameters = &function_type.parameters[arguments.len()..];
            Type::Function(FunctionType {
                parameters: remaining_parameters.to_vec(),
                return_type: function_type.return_type,
            })
        } else {
            *function_type.return_type
        }
    }
}
