use protobuf::descriptor::{field_descriptor_proto, FieldOptions};

use crate::{ast::impl_traits_and_methods, error::HydrationFailed};

use super::{
    access::NodeKeys,
    container, extension_decl, file, location, message, node, package,
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
    pub(super) package: Option<package::Key>,
    pub(super) container: container::Key,
    pub(super) extension_decl: extension_decl::Key,
    pub(super) extendee: message::Key,
    pub(super) location: location::Detail,
    pub(super) type_: value::TypeInner,
    pub(super) number: i32,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<Label>,
    pub(super) options: protobuf::MessageField<FieldOptions>,
    pub(super) reference: Option<reference::Inner>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    name: Name,
    block: extension_decl::Key,
    fqn: FullyQualifiedName,
    node_path: Vec<i32>,
    span: location::Span,
    comments: Option<location::Comments>,
    number: i32,
    label: Option<Label>,
    type_: value::TypeInner,
    extendee: message::Key,
    extension_decl: extension_decl::Key,
    default_value: Option<String>,
    json_name: Option<String>,
    ctype: Option<CType>,
    is_packed: bool,
    jstype: Option<JsType>,
    is_lazy: bool,
    is_deprecated: bool,
    is_weak: bool,
    uninterpreted_options: Vec<UninterpretedOption>,
    proto3_optional: Option<bool>,
    package: Option<package::Key>,
    reference: Option<reference::Inner>,
    container: container::Key,
    file: file::Key,

    special_fields: protobuf::SpecialFields,
    options_special_fields: protobuf::SpecialFields,
}

impl Inner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            file,
            package,
            type_,
            container,
            extendee,
            extension_decl,
            location,
            number,
            default_value,
            json_name,
            proto3_optional,
            special_fields,
            label,
            options,
            reference,
        } = hydrate;
        self.name = name;
        self.file = file;
        self.package = package;
        self.type_ = type_;
        self.container = container;
        self.proto3_optional = proto3_optional;
        self.extendee = extendee;
        self.number = number;
        self.default_value = default_value;
        self.json_name = json_name;
        self.special_fields = special_fields;
        self.label = label;
        self.reference = reference;
        self.extension_decl = extension_decl;
        self.hydrate_location(location);
        self.hydrate_options(options.unwrap_or_default())?;
        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: FieldOptions) -> Result<(), HydrationFailed> {
        let FieldOptions {
            ctype,
            jstype,
            packed,
            lazy,
            deprecated,
            weak,
            uninterpreted_option,
            special_fields,
        } = opts;
        self.options_special_fields = special_fields;
        self.ctype = ctype.map(Into::into);
        self.jstype = jstype.map(Into::into);
        self.is_packed = packed.unwrap_or_default();
        self.is_lazy = lazy.unwrap_or_default();
        self.is_deprecated = deprecated.unwrap_or_default();
        self.is_weak = weak.unwrap_or_default();
        self.uninterpreted_options = uninterpreted_option.into_iter().map(Into::into).collect();
        Ok(())
    }
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
        References::from_option(self.0.reference.as_ref(), self.ast())
    }
}
