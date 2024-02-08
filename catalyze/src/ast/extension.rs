use ahash::HashMap;
use protobuf::descriptor::FieldOptions as ProtoFieldOpts;

use crate::{ast::impl_traits_and_methods, error::HydrationFailed};

use super::{
    access::{
        AccessComments, AccessContainer, AccessFqn, AccessKey, AccessName, AccessNodeKeys,
        AccessReferences,
    },
    container::ContainerKey,
    extension_decl::ExtensionDeclKey,
    field::{FieldOptions, FieldTypeInner},
    file::FileKey,
    location::{Comments, Location},
    message::MessageKey,
    node,
    package::PackageKey,
    reference::{ReferenceInner, References},
    resolve::Resolver,
    FullyQualifiedName, Name, Span,
};

pub use super::field::{CType, JsType, Label};

slotmap::new_key_type! {
    pub(super) struct ExtensionKey;
}

pub struct Extension<'ast>(pub(super) Resolver<'ast, ExtensionKey, ExtensionInner>);
impl_traits_and_methods!(Extension, ExtensionKey, ExtensionInner);

impl<'ast> Extension<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        AccessReferences::references(self)
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

impl AccessFqn for Extension<'_> {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
impl AccessComments for Extension<'_> {
    fn comments(&self) -> Option<&Comments> {
        self.0.comments.as_ref()
    }
}
impl<'ast> AccessContainer<'ast> for Extension<'ast> {
    fn container(&self) -> super::container::Container<'ast> {
        (self.0.container, self.ast()).into()
    }
}

impl<'ast> AccessReferences<'ast> for Extension<'ast> {
    fn references(&'ast self) -> References<'ast> {
        References::from_option(self.0.reference.as_ref(), self.ast())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct ExtensionInner {
    pub(super) key: ExtensionKey,
    pub(super) name: Name,
    pub(super) block: ExtensionDeclKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) proto_path: Vec<i32>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
    pub(super) number: i32,
    pub(super) label: Option<Label>,
    pub(super) field_type: FieldTypeInner,
    pub(super) extendee: MessageKey,
    pub(super) extension_decl: ExtensionDeclKey,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) package: Option<PackageKey>,
    pub(super) reference: Option<ReferenceInner>,
    pub(super) container: ContainerKey,
    pub(super) file: FileKey,
    pub(super) options: FieldOptions,
    pub(super) proto_opts: ProtoFieldOpts,
    pub(super) special_fields: protobuf::SpecialFields,
}

impl AccessKey for ExtensionInner {
    type Key = ExtensionKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}
impl AccessName for ExtensionInner {
    fn name(&self) -> &str {
        &self.name
    }
}
impl AccessNodeKeys for ExtensionInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        std::iter::empty()
    }
}
impl ExtensionInner {
    pub(super) fn hydrate(
        &mut self,
        hydrate: HydrateExtension,
    ) -> Result<ExtensionIdent, HydrationFailed> {
        let HydrateExtension {
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
        self.field_type = type_;
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

pub(super) struct HydrateExtension {
    pub(super) name: Name,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) container: ContainerKey,
    pub(super) extension_decl: ExtensionDeclKey,
    pub(super) extendee: MessageKey,
    pub(super) location: Location,
    pub(super) type_: FieldTypeInner,
    pub(super) number: i32,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<Label>,
    pub(super) options: ProtoFieldOpts,
    pub(super) reference: Option<ReferenceInner>,
}

pub(super) type ExtensionIdent = node::Ident<ExtensionKey>;
pub(super) type ExtensionTable =
    super::table::Table<ExtensionKey, ExtensionInner, HashMap<FullyQualifiedName, ExtensionKey>>;
