use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    checker::{Scope, Type, TypeParameter},
    parser::{Node, TypeParameterNode},
};

pub struct TypeParameterListNode {
    pub list: Vec<Node<TypeParameterNode>>,
    types: OnceCell<HashMap<String, Rc<TypeParameter>>>,
}

impl TypeParameterListNode {
    pub fn new(list: Vec<Node<TypeParameterNode>>) -> Self {
        Self {
            list,
            types: OnceCell::new(),
        }
    }

    pub fn check(&self, mut scope: Box<Scope>) -> Box<Scope> {
        let mut names = HashSet::new();
        for type_param in self.list.iter() {
            if !names.insert(&type_param.name.value) {
                scope.source.print_error(
                    type_param.name.span,
                    &format!("Duplicate type parameter name `{}`", type_param.name),
                    "type alias already contains a type parameter with this name",
                );
            } else {
                let type_parameter = Rc::new(TypeParameter {
                    name: type_param.name.clone(),
                });
                scope.add_type(&type_param.name, Type::TypeParameter(type_parameter));
            }
        }

        scope
    }

    pub fn get_types(&self) -> &HashMap<String, Rc<TypeParameter>> {
        self.types.get_or_init(|| self.init_types())
    }

    fn init_types(&self) -> HashMap<String, Rc<TypeParameter>> {
        let mut types = HashMap::new();
        // TODO make sure this is selecting the first item
        for type_param in self.list.iter() {
            types
                .entry(type_param.name.value.clone())
                .or_insert_with(|| {
                    Rc::new(TypeParameter {
                        name: type_param.name.clone(),
                    })
                });
        }

        types
    }
}
