use ahash::HashMap;
use protobuf::{
    descriptor::{self, enum_descriptor_proto},
    SpecialFields,
};

use crate::error::HydrationFailed;

use std::{fmt, str::FromStr};

use super::{
    access::{
        AccessComments, AccessFile, AccessFqn, AccessKey, AccessName, AccessNodeKeys, AccessPackage,
    },
    collection::Collection,
    container::ContainerKey,
    enum_value::EnumValueKey,
    file::FileKey,
    impl_traits_and_methods,
    location::{Comments, Location, Span},
    node::Ident,
    package::PackageKey,
    reference::ReferrerKey,
    reserved::Reserved,
    resolve::Resolver,
    table::Table,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct EnumKey;
}

pub struct Enum<'ast>(pub(super) Resolver<'ast, EnumKey, EnumInner>);
impl AccessName for Enum<'_> {
    fn name(&self) -> &str {
        &self.0.name
    }
}
impl AccessKey for Enum<'_> {
    type Key = EnumKey;

    fn key(&self) -> Self::Key {
        self.0.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.0.key
    }
}
impl AccessComments for Enum<'_> {
    fn comments(&self) -> Option<&Comments> {
        self.0.comments.as_ref()
    }
}
impl<'ast> AccessPackage<'ast> for Enum<'ast> {
    fn package(&self) -> Option<super::package::Package<'ast>> {
        self.0.package.map(|key| (key, self.ast()).into())
    }
}
impl<'ast> AccessFile<'ast> for Enum<'ast> {
    fn file(&self) -> super::file::File<'ast> {
        (self.0.file, self.ast()).into()
    }
}
impl AccessFqn for Enum<'_> {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}

impl_traits_and_methods!(Enum, EnumKey, EnumInner);

impl<'ast> Enum<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
    }
}

pub(super) type EnumTable = Table<EnumKey, EnumInner, HashMap<FullyQualifiedName, EnumKey>>;
pub(super) type EnumIdent = Ident<EnumKey>;

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct EnumInner {
    pub(super) key: EnumKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) proto_path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
    pub(super) reserved: Reserved,
    pub(super) package: Option<PackageKey>,
    pub(super) file: FileKey,
    pub(super) container: ContainerKey,
    pub(super) referenced_by: Vec<ReferrerKey>,
    pub(super) values: Collection<EnumValueKey>,
    pub(super) well_known: Option<WellKnownEnum>,
    pub(super) option_special_fields: SpecialFields,
    pub(super) uninterpreted_options: Vec<UninterpretedOption>,
    pub(super) special_fields: SpecialFields,
    pub(super) options: EnumOptions,
    pub(super) proto_opts: descriptor::EnumOptions,
}

impl EnumInner {
    pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Result<EnumIdent, HydrationFailed> {
        let Hydrate {
            name,
            package,
            file,
            values,
            location,
            mut options,
            special_fields,
            reserved_names,
            reserved_ranges,
            container,
            well_known,
        } = hydrate;

        self.values = values.into();
        self.name = name;
        self.file = file;
        self.package = package;

        self.set_reserved(reserved_names, reserved_ranges);
        self.container = container;
        self.well_known = well_known;
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        Ok(self.into())
    }
}
impl AccessKey for EnumInner {
    type Key = EnumKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}
impl AccessNodeKeys for EnumInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        self.values
            .iter()
            .copied()
            .map(super::node::NodeKey::EnumValue)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumOptions {
    pub allow_alias: Option<bool>,
    pub deprecated: Option<bool>,
}
impl EnumOptions {
    pub fn allow_alias(&self) -> bool {
        self.allow_alias.unwrap_or(false)
    }
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }
}
impl EnumOptions {
    fn hydrate(&mut self, options: &mut descriptor::EnumOptions) {
        self.allow_alias = options.allow_alias.take();
        self.deprecated = options.deprecated.take();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownEnum {
    /// Whether a field is optional, required, or repeated.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality>
    FieldCardinality,
    /// Basic field types.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind>
    FieldKind,

    /// NullValue is a singleton enumeration to represent the null value for the
    /// Value type union.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#nullvalue>
    NullValue,
    /// The syntax in which a protocol buffer element is defined.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#syntax>
    Syntax,
}
impl WellKnownEnum {
    const FIELD_CARDINALITY: &'static str = "FieldCardinality";
    const FIELD_KIND: &'static str = "FieldKind";
    const NULL_VALUE: &'static str = "NullValue";
    const SYNTAX: &'static str = "Syntax";
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldCardinality => Self::FIELD_CARDINALITY,
            Self::FieldKind => Self::FIELD_KIND,
            Self::NullValue => Self::NULL_VALUE,
            Self::Syntax => Self::SYNTAX,
        }
    }
}
impl FromStr for WellKnownEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::FIELD_CARDINALITY => Ok(Self::FieldCardinality),
            Self::FIELD_KIND => Ok(Self::FieldKind),
            Self::NULL_VALUE => Ok(Self::NullValue),
            Self::SYNTAX => Ok(Self::Syntax),
            _ => Err(()),
        }
    }
}

impl fmt::Display for WellKnownEnum {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) package: Option<PackageKey>,
    pub(super) file: FileKey,
    pub(super) values: Vec<Ident<EnumValueKey>>,
    pub(super) location: Location,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) reserved_names: Vec<String>,
    pub(super) reserved_ranges: Vec<enum_descriptor_proto::EnumReservedRange>,
    pub(super) container: ContainerKey,
    pub(super) well_known: Option<WellKnownEnum>,
    pub(super) options: descriptor::EnumOptions,
}
