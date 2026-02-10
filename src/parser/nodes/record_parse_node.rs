use crate::{
    checker::{StructType, Type, TypeResolver},
    parser::{
        IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec, RecordFieldParseNode,
        TokenSpan, Traverse,
    },
};

pub struct RecordDefinitionParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub fields: ParseNodeVec<RecordFieldParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for RecordDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Record.identifier", self.identifier.span);
        visit("Record.fields", self.fields.span);
        for field in self.fields.value.iter() {
            field.traverse("Record.field", visit);
        }
        if let Some(methods) = self.methods.as_ref() {
            visit("Record.methods", methods.span);
            for method in methods.value.iter() {
                method.traverse("Record.method", visit);
            }
        }
    }
}

impl RecordDefinitionParseNode {
    pub fn register_type(&self, types: &mut TypeResolver) {
        let container_name = &self.identifier.value.0;
        let mut struct_type = StructType::new();

        for field in self.fields.value.iter() {
            let member = field.value.resolve_type(types);
            let identifier = &field.value.identifier.value.0;
            struct_type.add_member(identifier, container_name, member, types);
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.value.iter() {
                let identifier = &method.value.function.value.identifier.value.0;
                let member = method.value.resolve_struct_method(types);
                struct_type.add_member(identifier, container_name, member, types);
            }
        }

        types.insert(&self.identifier.value.0, Type::Struct(struct_type))
    }
}
