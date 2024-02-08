use ahash::HashMap;
use protobuf::{descriptor::OneofOptions as ProtoOneofOpts, SpecialFields};

use crate::error::HydrationFailed;

use super::{
    access::{
        AccessComments, AccessContainer, AccessFile, AccessFqn, AccessKey, AccessName,
        AccessNodeKeys, AccessPackage, AccessProtoPath,
    },
    collection::Collection,
    field::{FieldIdent, FieldKey},
    file::FileKey,
    impl_traits_and_methods,
    location::{Comments, Location, Span},
    message::{Message, MessageKey},
    node::Ident,
    package::PackageKey,
    resolve::Resolver,
    uninterpreted::{into_uninterpreted_options, UninterpretedOption},
    Container, FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct OneofKey;
}

pub struct Oneof<'ast>(pub(super) Resolver<'ast, OneofKey, OneofInner>);

impl<'ast> AccessFqn for Oneof<'ast> {
    fn fqn(&self) -> &super::FullyQualifiedName {
        &self.0.fqn
    }
}
impl<'ast> AccessName for Oneof<'ast> {
    fn name(&self) -> &str {
        &self.0.name
    }
}
impl<'ast> AccessComments for Oneof<'ast> {
    fn comments(&self) -> Option<&super::location::Comments> {
        self.0.comments.as_ref()
    }
}
impl<'ast> AccessFile<'ast> for Oneof<'ast> {
    fn file(&self) -> super::file::File<'ast> {
        (self.0.file, self.ast()).into()
    }
}
impl<'ast> AccessContainer<'ast> for Oneof<'ast> {
    fn container(&self) -> Container<'ast> {
        (self.0.message, self.ast()).into()
    }
}
impl<'ast> AccessPackage<'ast> for Oneof<'ast> {
    fn package(&self) -> Option<super::package::Package<'ast>> {
        self.0.package.map(|key| (key, self.ast()).into())
    }
}
impl<'ast> AccessProtoPath for Oneof<'ast> {
    fn proto_path(&self) -> &[i32] {
        &self.0.proto_path
    }
}

impl<'ast> Oneof<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
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
    pub(super) proto_path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
    pub(super) file: FileKey,
    pub(super) special_fields: SpecialFields,
    pub(super) fields: Collection<FieldKey>,
    pub(super) proto_opts: ProtoOneofOpts,
    pub(super) options: OneofOptions,
}
impl AccessFqn for OneofInner {
    fn fqn(&self) -> &super::FullyQualifiedName {
        &self.fqn
    }
}
impl AccessKey for OneofInner {
    type Key = OneofKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}
impl OneofInner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<OneofIdent, HydrationFailed> {
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
    pub(crate) fn add_field(&mut self, field: FieldIdent) {
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
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) location: Location,
    pub(super) fields: Vec<FieldIdent>,
    pub(super) options: ProtoOneofOpts,
    pub(super) special_fields: protobuf::SpecialFields,
}

pub(super) type OneofIdent = Ident<OneofKey>;
pub(super) type OneofTable =
    super::table::Table<OneofKey, OneofInner, HashMap<FullyQualifiedName, OneofKey>>;
