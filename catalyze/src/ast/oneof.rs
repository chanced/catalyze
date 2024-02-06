use protobuf::{descriptor::OneofOptions as ProtoOneofOpts, SpecialFields};

use crate::error::HydrationFailed;

use super::{
    access::{AccessName, AccessNodeKeys},
    collection::Collection,
    field::{self, FieldKey},
    file, impl_traits_and_methods,
    location::{self, Comments, Span},
    message::{self, MessageKey},
    node,
    package::{self, PackageKey},
    resolve::Resolver,
    uninterpreted::{into_uninterpreted_options, UninterpretedOption},
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct OneofKey;
}

pub struct Oneof<'ast>(pub(super) Resolver<'ast, OneofKey, OneofInner>);
impl<'ast> Oneof<'ast> {
    pub fn name(&self) -> &str {
        self.0.name.as_ref()
    }
}
impl<'ast> AccessName for Oneof<'ast> {
    fn name(&self) -> &str {
        self.0.name.as_ref()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct OneofOptions {
    pub uninterpreted_options: Vec<UninterpretedOption>,
}
impl OneofOptions {
    pub fn hydrate(&mut self, proto_opts: &mut ProtoOneofOpts) {
        self.uninterpreted_options = into_uninterpreted_options(&proto_opts.uninterpreted_option);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct OneofInner {
    pub(super) key: OneofKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) message: MessageKey,
    pub(super) package: Option<PackageKey>,
    pub(super) node_path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
    pub(super) file: file::FileKey,
    pub(super) special_fields: SpecialFields,
    pub(super) fields: Collection<FieldKey>,
    pub(super) proto_opts: ProtoOneofOpts,
    pub(super) options: OneofOptions,
}
impl OneofInner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            message,
            file,
            package,
            location,
            fields,
            mut options,
            special_fields,
        } = hydrate;
        self.name = name;
        self.message = message;
        self.package = package;
        self.file = file;
        self.fields = fields.into();
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        Ok(self.into())
    }
    pub(crate) fn add_field(&mut self, field: node::Ident<FieldKey>) {
        self.fields.push(field);
    }
}
impl AccessNodeKeys for OneofInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        self.fields.iter().copied().map(super::node::NodeKey::Field)
    }
}
impl_traits_and_methods!(Oneof, OneofKey, OneofInner);

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) message: MessageKey,
    pub(super) file: file::FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) location: location::Location,
    pub(super) fields: Vec<field::FieldIdent>,
    pub(super) options: ProtoOneofOpts,
    pub(super) special_fields: protobuf::SpecialFields,
}

pub(super) type Ident = node::Ident<OneofKey>;
pub(super) type Table = super::table::Table<OneofKey, OneofInner>;
