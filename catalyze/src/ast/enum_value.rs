use protobuf::{descriptor, SpecialFields};

use crate::error::HydrationFailed;

use super::{
    access::{AccessName, AccessNodeKeys},
    file, impl_traits_and_methods, location, node, package,
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
    pub(super) node_path: Box<[i32]>,
    pub(super) number: i32,
    pub(super) enum_: super::enum_::EnumKey,
    pub(super) file: file::FileKey,
    pub(super) package: Option<package::PackageKey>,
    pub(super) span: location::Span,
    pub(super) comments: Option<location::Comments>,
    pub(super) uninterpreted_options: Vec<UninterpretedOption>,
    pub(super) special_fields: SpecialFields,
    pub(super) options: EnumValueOptions,
    pub(super) proto_opts: descriptor::EnumValueOptions,
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
    pub(super) file: file::FileKey,
    pub(super) package: Option<package::PackageKey>,
}

pub(super) type EnumValueIdent = node::Ident<EnumValueKey>;
pub(super) type EnumValueTable = super::table::Table<EnumValueKey, EnumValueInner>;
