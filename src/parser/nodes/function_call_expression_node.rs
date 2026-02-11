use crate::{
    checker::{FunctionType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, NodeVec},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Option<Type>) {
        let (scope, function_type) = self.function.check(types, scope);
        let (scope, function_type) = self.get_function_type(function_type, types, scope);
        let (scope, arguments) = self.get_args(types, scope);

        if let Some(function_type) = function_type {
            self.check_arguments(&function_type, &arguments);
            let return_type = function_type.return_type.map(|return_type| *return_type);
            (scope, return_type)
        } else {
            (scope, Some(Type::Error))
        }
    }

    fn get_args(&self, types: &TypeResolver, mut scope: Box<Scope>) -> (Box<Scope>, Vec<Type>) {
        let mut result = vec![];
        for argument in self.arguments.iter() {
            let (new_scope, resolved_type) = argument.check(types, scope);
            result.push(resolved_type);
            scope = new_scope;
        }
        (scope, result)
    }

    fn check_arguments(&self, function_type: &FunctionType, arguments: &Vec<Type>) {
        if function_type.parameters.len() != arguments.len() {
            println!(
                "Call expected {} arguments but recieved {}",
                function_type.parameters.len(),
                arguments.len()
            );
        }

        // TODO check types of each argument
    }

    fn get_function_type(
        &self,
        input_type: Type,
        types: &TypeResolver,
        scope: Box<Scope>,
    ) -> (Box<Scope>, Option<FunctionType>) {
        match input_type {
            Type::Alias(resolved_type) => self.get_function_type(*resolved_type, types, scope),
            Type::Enum(_) => {
                println!("No call operator for enum");
                (scope, None)
            }
            Type::Function(function_type) => (scope, Some(function_type)),
            Type::Primitive(primitive_type) => {
                println!("Primitive type `{:?}` is not callable", primitive_type);
                (scope, None)
            }
            Type::Reference(index) => {
                let resolved_type = types.get_type(index).unwrap();
                self.get_function_type(resolved_type, types, scope)
            }
            Type::Struct(_struct_type) => todo!("Implement call operator for structs"),
            Type::Tuple(_tuple_type) => todo!("Implement call operator for tuple"),
            Type::Error => (scope, None),
        }
    }
}
