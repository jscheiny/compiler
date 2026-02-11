use crate::{
    checker::{StructType, Type, TypeResolver},
    parser::{
        Identified, IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec,
        StructFieldParseNode,
    },
};

pub struct StructParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub fields: ParseNodeVec<StructFieldParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl StructParseNode {
    pub fn resolve_types(&mut self, types: &mut TypeResolver) {
        let container_name = self.id().clone();
        let mut struct_type = StructType::new();

        for field in self.fields.value.iter() {
            let member = field.value.resolve_type(types);
            let identifier = field.id().clone();
            struct_type.add_member(identifier, &container_name, member, types);
        }

        if let Some(methods) = self.methods.as_mut() {
            for method in methods.value.iter_mut() {
                let member = method.value.resolve_struct_method(types);
                let identifier = method.id().clone();
                struct_type.add_member(identifier, &container_name, member, types);
            }
        }

        types.resolve(&container_name, Type::Struct(struct_type))
    }
}

impl Identified for StructParseNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
