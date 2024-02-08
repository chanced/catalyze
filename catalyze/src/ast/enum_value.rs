use ahash::HashMap;
use protobuf::{descriptor, SpecialFields};

use crate::error::HydrationFailed;

use super::{
    access::{
        AccessComments, AccessFile, AccessFqn, AccessKey, AccessName, AccessNodeKeys, AccessPackage,
    },
    file::FileKey,
    impl_traits_and_methods,
    location::{self, Comments},
    node,
    package::PackageKey,
    resolve::Resolver,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct EnumValueKey;
}

pub struct EnumValue<'ast>(pub(super) Resolver<'ast, EnumValueKey, EnumValueInner>);

impl<'ast> EnumValue<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
    }
}
impl<'ast> AccessName for EnumValue<'ast> {
    fn name(&self) -> &str {
        &self.0.name
    }
}
impl AccessKey for EnumValue<'_> {
    type Key = EnumValueKey;

    fn key(&self) -> Self::Key {
        self.0.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.0.key
    }
}
impl AccessComments for EnumValue<'_> {
    fn comments(&self) -> Option<&Comments> {
        self.0.comments.as_ref()
    }
}
impl<'ast> AccessPackage<'ast> for EnumValue<'ast> {
    fn package(&self) -> Option<super::package::Package<'ast>> {
        self.0.package.map(|key| (key, self.ast()).into())
    }
}
impl<'ast> AccessFile<'ast> for EnumValue<'ast> {
    fn file(&self) -> super::file::File<'ast> {
        (self.0.file, self.ast()).into()
    }
}
impl AccessFqn for EnumValue<'_> {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnumValueOptions {
    deprecated: Option<bool>,
}
impl EnumValueOptions {
    fn hydrate(&mut self, options: &mut descriptor::EnumValueOptions) {
        self.deprecated = options.deprecated.take();
    }
    pub fn deprecated(&self) -> Option<bool> {
        self.deprecated
    }
}

impl_traits_and_methods!(EnumValue, EnumValueKey, EnumValueInner);

/// [`EnumValue`] inner data.
#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct EnumValueInner {
    /// enum_value::Key
    pub(super) key: EnumValueKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) proto_path: Box<[i32]>,
    pub(super) number: i32,
    pub(super) enum_: super::enum_::EnumKey,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) span: location::Span,
    pub(super) comments: Option<Comments>,
    pub(super) uninterpreted_options: Vec<UninterpretedOption>,
    pub(super) special_fields: SpecialFields,
    pub(super) options: EnumValueOptions,
    pub(super) proto_opts: descriptor::EnumValueOptions,
}
impl AccessFqn for EnumValueInner {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.fqn
    }
}
impl AccessKey for EnumValueInner {
    type Key = EnumValueKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}

impl EnumValueInner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<EnumValueIdent, HydrationFailed> {
        let Hydrate {
            name,
            number,
            location,
            mut options,
            special_fields,
            enum_,
            file,
            package,
        } = hydrate;
        self.name = name;
        self.number = number;
        self.file = file;
        self.package = package;
        self.special_fields = special_fields;
        self.enum_ = enum_;
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        self.hydrate_location(location);
        Ok(self.into())
    }
}

impl AccessNodeKeys for EnumValueInner {
    fn keys(&self) -> impl Iterator<Item = node::NodeKey> {
        std::iter::empty()
    }
}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) number: i32,
    pub(super) location: location::Location,
    pub(super) options: descriptor::EnumValueOptions,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) enum_: super::enum_::EnumKey,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
}

pub(super) type EnumValueIdent = node::Ident<EnumValueKey>;
pub(super) type EnumValueTable =
    super::table::Table<EnumValueKey, EnumValueInner, HashMap<FullyQualifiedName, EnumValueKey>>;
