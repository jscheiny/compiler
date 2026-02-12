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
        let function_type = get_function_type(function_type, types);
        let (scope, arguments) = self.get_args(types, scope);

        if let Some(function_type) = function_type {
            self.check_args(&function_type, &arguments, types);
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

    fn check_args(&self, function_type: &FunctionType, arguments: &[Type], types: &TypeResolver) {
        if function_type.parameters.len() != arguments.len() {
            println!(
                "Type error: Call expected {} arguments but recieved {}",
                function_type.parameters.len(),
                arguments.len()
            );
        }

        for (index, (left, right)) in function_type.parameters.iter().zip(arguments).enumerate() {
            if !left.is_assignable_to(right, types) {
                println!("Type error: Mismatch in type of argument {}", index);
            }
        }
        // TODO check types of each argument
    }
}

// TODO move this elsewhere
pub fn get_function_type(input_type: Type, types: &TypeResolver) -> Option<FunctionType> {
    match input_type {
        Type::Enum(_) => {
            println!("Type error: No call operator for enum");
            None
        }
        Type::Function(function_type) => Some(function_type),
        Type::Primitive(primitive_type) => {
            println!(
                "Type error: Primitive type `{:?}` is not callable",
                primitive_type
            );
            None
        }
        Type::Reference(index) => {
            let resolved_type = types.get_type(index).unwrap();
            get_function_type(resolved_type, types)
        }
        Type::Struct(_) => {
            println!("Type error: no call operator for struct");
            None
        }
        Type::Tuple(_) => {
            println!("Type error: No call operator for tuple");
            None
        }
        Type::Type(_) => todo!("Implement call operator for types"),
        Type::Void => {
            println!("Type error: no call operator for void");
            None
        }
        Type::Error => None,
    }
}
