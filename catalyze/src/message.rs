use std::sync::{Arc, Weak};

use slotmap::new_key_type;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inner {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message(Arc<Inner>);

pub struct WeakMessage(Weak<Inner>);

new_key_type! {
    pub(crate) struct MessageKey;
}

pub(crate) struct HydrateMessage {
    // message fields
    name: Option<::std::string::String>,
    // field: Vec<FieldDescriptorProto>,
    // extension: Vec<FieldDescriptorProto>,
    // nested_type: Vec<DescriptorProto>,
    // enum_type: Vec<EnumDescriptorProto>,
    // extension_range: Vec<descriptor_proto::ExtensionRange>,
    // oneof_decl: Vec<OneofDescriptorProto>,
    // options: protobuf::MessageField<MessageOptions>,
    // reserved_range: Vec<descriptor_proto::ReservedRange>,
    ///  Reserved field names, which may not be used by fields in the same message.
    ///  A given name may only be reserved once.
    reserved_name: Vec<::std::string::String>,
    pub special_fields: protobuf::SpecialFields,
}
