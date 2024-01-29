use protobuf::{
    descriptor::{field_descriptor_proto, FieldOptions},
    EnumOrUnknown,
};

use crate::{ast::impl_traits_and_methods, error::HydrationFailed};

use super::{
    access::NodeKeys,
    container, extension_decl,
    field::TypeInner,
    file, location, message, node, package,
    reference::{self, References},
    resolve,
    uninterpreted::UninterpretedOption,
    value, FullyQualifiedName, Name,
};

pub use super::field::{CType, JsType, Label};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) type Ident = node::Ident<Key>;
pub(super) type Table = super::table::Table<Key, Inner>;

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) file: file::Key,
    pub(super) value: value::Inner,
    pub(super) package: Option<package::Key>,
    pub(super) container: container::Key,
    pub(super) extendee: message::Key,
    pub(super) location: location::Detail,
    pub(super) number: Option<i32>,
    pub(super) type_: Option<EnumOrUnknown<field_descriptor_proto::Type>>,
    pub(super) type_name: Option<String>,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<protobuf::EnumOrUnknown<field_descriptor_proto::Label>>,
    pub(super) options: protobuf::MessageField<FieldOptions>,
    pub(super) proto_type: field_descriptor_proto::Type,
    pub(super) reference: Option<reference::Inner>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    name: Box<str>,
    value: value::Inner,
    block: extension_decl::Key,
    fqn: FullyQualifiedName,
    node_path: Vec<i32>,
    span: location::Span,
    comments: Option<location::Comments>,
    number: i32,
    label: Option<Label>,
    field_type: TypeInner,
    type_name: Option<String>,
    extendee: message::Key,
    default_value: Option<String>,
    oneof_index: Option<i32>,
    json_name: Option<String>,
    ctype: Option<CType>,
    packed: bool,
    jstype: Option<JsType>,
    lazy: bool,
    deprecated: bool,
    weak: bool,
    uninterpreted_options: Vec<UninterpretedOption>,
    proto3_optional: Option<bool>,
    package: Option<package::Key>,
    reference: Option<reference::Inner>,
    container: container::Key,
    file: file::Key,
}

impl Inner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            file,
            value,
            package,
            container,
            extendee,
            location,
            number,
            type_,
            type_name,
            default_value,
            json_name,
            proto3_optional,
            special_fields,
            label,
            options,
            proto_type,
            reference,
        } = hydrate;
    }
    fn hydrate_options(&mut self, opts: FieldOptions) -> Result<(), HydrationFailed> {}
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        std::iter::empty()
    }
}

impl Inner {}
pub struct Extension<'ast>(resolve::Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(Extension, Key, Inner);
impl<'ast> Extension<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::References::references(self)
    }
}

impl<'ast> super::access::References<'ast> for Extension<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_option(self.0.reference, self.ast())
    }
}
