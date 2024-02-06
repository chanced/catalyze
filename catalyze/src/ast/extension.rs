use protobuf::descriptor::FieldOptions as ProtoFieldOpts;

use crate::{ast::impl_traits_and_methods, error::HydrationFailed};

use super::{
    access::{AccessName, AccessNodeKeys},
    container,
    extension_decl::{self, ExtensionDeclKey},
    field::FieldOptions,
    file,
    location::{self, Comments},
    message::{self, MessageKey},
    node,
    package::{self, PackageKey},
    reference::{self, ReferenceInner, References},
    resolve::Resolver,
    value, FullyQualifiedName, Name,
};

pub use super::field::{CType, JsType, Label};

slotmap::new_key_type! {
    pub(super) struct ExtensionKey;
}

pub struct Extension<'ast>(pub(super) Resolver<'ast, ExtensionKey, ExtensionInner>);
impl_traits_and_methods!(Extension, ExtensionKey, ExtensionInner);

impl<'ast> Extension<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::AccessReferences::references(self)
    }
    pub fn name(&'ast self) -> &'ast str {
        &self.0.name
    }
}
impl<'ast> AccessName for Extension<'ast> {
    fn name(&self) -> &str {
        &self.0.name
    }
}

impl<'ast> super::access::AccessReferences<'ast> for Extension<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_option(self.0.reference.as_ref(), self.ast())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct ExtensionInner {
    pub(super) key: ExtensionKey,
    pub(super) name: Name,
    pub(super) block: ExtensionDeclKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) node_path: Vec<i32>,
    pub(super) span: location::Span,
    pub(super) comments: Option<Comments>,
    pub(super) number: i32,
    pub(super) label: Option<Label>,
    pub(super) type_: value::TypeInner,
    pub(super) extendee: MessageKey,
    pub(super) extension_decl: ExtensionDeclKey,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) package: Option<PackageKey>,
    pub(super) reference: Option<ReferenceInner>,
    pub(super) container: container::Key,
    pub(super) file: file::FileKey,
    pub(super) options: FieldOptions,
    pub(super) proto_opts: ProtoFieldOpts,
    pub(super) special_fields: protobuf::SpecialFields,
}

impl ExtensionInner {
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
            mut options,
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
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        Ok(self.into())
    }
}

impl AccessNodeKeys for ExtensionInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        std::iter::empty()
    }
}

impl ExtensionInner {}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) file: file::FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) container: container::Key,
    pub(super) extension_decl: ExtensionDeclKey,
    pub(super) extendee: MessageKey,
    pub(super) location: location::Location,
    pub(super) type_: value::TypeInner,
    pub(super) number: i32,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<Label>,
    pub(super) options: ProtoFieldOpts,
    pub(super) reference: Option<ReferenceInner>,
}

pub(super) type Ident = node::Ident<ExtensionKey>;
pub(super) type Table = super::table::Table<ExtensionKey, ExtensionInner>;
