use std::{fmt, slice};

use crate::{
    ast::{impl_traits_and_methods, uninterpreted::UninterpretedOption, FullyQualifiedName},
    error::{self, HydrationFailed},
};
use ahash::HashMap;
use protobuf::{
    descriptor::{
        self,
        field_descriptor_proto::{self, Type as ProtoType},
        field_options::CType as ProtoCType,
        FieldOptions as ProtoFieldOpts,
    },
    EnumOrUnknown, SpecialFields,
};
use snafu::Backtrace;

use super::{
    access::{AccessComments, AccessFile, AccessFqn, AccessKey, AccessName, AccessNodeKeys},
    collection::Collection,
    enum_::EnumKey,
    file::FileKey,
    location::{Comments, Location},
    message::MessageKey,
    node,
    package::PackageKey,
    reference::{ReferenceInner, References},
    resolve::Resolver,
    uninterpreted::into_uninterpreted_options,
    Ast, Enum, File, Message, Name, Span,
};

slotmap::new_key_type! {
    pub(super) struct FieldKey;
}

pub struct Field<'ast>(pub(super) Resolver<'ast, FieldKey, FieldInner>);
impl_traits_and_methods!(Field, FieldKey, FieldInner);

impl<'ast> Field<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
    }
}

impl<'ast> AccessFqn for Field<'ast> {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
impl<'ast> AccessName for Field<'ast> {
    fn name(&self) -> &str {
        &self.0.name
    }
}
impl<'ast> AccessFile<'ast> for Field<'ast> {
    fn file(&self) -> File<'ast> {
        (self.0.file, self.ast()).into()
    }
}
impl<'ast> AccessComments for Field<'ast> {
    fn comments(&self) -> Option<&Comments> {
        self.0.comments.as_ref()
    }
}
pub struct Fields<'ast> {
    collection: &'ast Collection<FieldKey>,
    ast: &'ast Ast,
}
impl<'ast> Fields<'ast> {
    pub fn get_by_name(&self, name: &str) -> Option<Field> {
        self.collection
            .get_by_name(name)
            .map(|k| (k, self.ast).into())
    }
}

impl<'ast> IntoIterator for Fields<'ast> {
    type Item = Field<'ast>;
    type IntoIter = Iter<'ast>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.collection.iter(),
            ast: self.ast,
        }
    }
}
impl<'ast> IntoIterator for &Fields<'ast> {
    type Item = Field<'ast>;
    type IntoIter = Iter<'ast>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.collection.iter(),
            ast: self.ast,
        }
    }
}

pub struct Iter<'ast> {
    iter: slice::Iter<'ast, FieldKey>,
    ast: &'ast Ast,
}
impl<'ast> Iterator for Iter<'ast> {
    type Item = Field<'ast>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied().map(|key| (key, self.ast).into())
    }
}
impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub(super) type FieldIdent = node::Ident<FieldKey>;
pub(super) type FieldTable =
    super::table::Table<FieldKey, FieldInner, HashMap<FullyQualifiedName, FieldKey>>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FieldOptions {
    pub ctype: Option<CType>,
    pub jstype: Option<JsType>,
    pub lazy: Option<bool>,
    pub packed: Option<bool>,
    pub weak: Option<bool>,
    pub deprecated: Option<bool>,
    pub uninterpreted_options: Vec<UninterpretedOption>,
}
impl FieldOptions {
    pub(super) fn hydrate(&mut self, proto_opts: &mut descriptor::FieldOptions) {
        self.ctype = proto_opts.ctype.take().map(Into::into);
        self.jstype = proto_opts.jstype.take().map(Into::into);
        self.packed = proto_opts.packed.take();
        self.lazy = proto_opts.lazy.take();
        self.weak = proto_opts.weak.take();
        self.deprecated = proto_opts.deprecated.take();
        self.uninterpreted_options = into_uninterpreted_options(&proto_opts.uninterpreted_option);
    }
    pub fn ctype(&self) -> CType {
        self.ctype.unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
pub(super) struct FieldInner {
    pub(super) key: FieldKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) proto_path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
    pub(super) number: i32,
    pub(super) label: Option<Label>,
    pub(super) field_type: FieldTypeInner,
    pub(super) message: MessageKey,
    pub(super) default_value: Option<String>,
    pub(super) oneof_index: Option<i32>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) package: Option<PackageKey>,
    pub(super) reference: Option<ReferenceInner>,
    pub(super) file: FileKey,
    pub(super) special_fields: SpecialFields,
    pub(super) options: FieldOptions,
    pub(super) proto_opts: descriptor::FieldOptions,
}

impl FieldInner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<FieldIdent, HydrationFailed> {
        let Hydrate {
            location,
            file,
            number,
            label,
            mut options,
            name,
            message,
            package,
            type_,
            default_value,
            json_name,
            proto3_optional,
            oneof_index,
            special_fields,
            reference,
        } = hydrate;

        self.name = name;
        self.file = file;
        self.reference = reference;
        self.package = package;
        self.message = message;
        self.default_value = default_value;
        self.oneof_index = oneof_index;
        self.json_name = json_name;
        self.proto3_optional = proto3_optional;
        self.number = number;
        self.label = label;
        self.special_fields = special_fields;
        self.field_type = type_;
        self.hydrate_location(location);
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        Ok(self.into())
    }
}

impl AccessKey for FieldInner {
    type Key = FieldKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}

impl AccessFqn for FieldInner {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.fqn
    }
}
impl AccessNodeKeys for FieldInner {
    fn keys(&self) -> impl Iterator<Item = node::NodeKey> {
        std::iter::empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
}
impl Default for Label {
    fn default() -> Self {
        Self::Optional
    }
}

impl Label {
    /// Returns `true` if the label is [`Required`].
    ///
    /// [`Required`]: Label::Required
    #[must_use]
    pub fn is_required(&self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns `true` if the label is [`Optional`].
    ///
    /// [`Optional`]: Label::Optional
    #[must_use]
    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional)
    }

    /// Returns `true` if the label is [`Repeated`].
    ///
    /// [`Repeated`]: Label::Repeated
    #[must_use]
    pub fn is_repeated(&self) -> bool {
        matches!(self, Self::Repeated)
    }
}
impl From<field_descriptor_proto::Label> for Label {
    fn from(value: field_descriptor_proto::Label) -> Self {
        use field_descriptor_proto::Label as ProtoLabel;
        match value {
            ProtoLabel::LABEL_OPTIONAL => Self::Optional,
            ProtoLabel::LABEL_REQUIRED => Self::Required,
            ProtoLabel::LABEL_REPEATED => Self::Repeated,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    #[default]
    String = 0,
    Cord = 1,
    StringPiece = 2,
    Unknown(i32),
}
impl From<EnumOrUnknown<ProtoCType>> for CType {
    fn from(value: EnumOrUnknown<ProtoCType>) -> Self {
        match value.enum_value() {
            Ok(v) => v.into(),
            Err(v) => Self::Unknown(v),
        }
    }
}
impl From<&ProtoCType> for CType {
    fn from(value: &ProtoCType) -> Self {
        match value {
            ProtoCType::STRING => Self::String,
            ProtoCType::CORD => Self::Cord,
            ProtoCType::STRING_PIECE => Self::StringPiece,
        }
    }
}

impl From<ProtoCType> for CType {
    fn from(value: ProtoCType) -> Self {
        Self::from(&value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum JsType {
    /// Use the default type.
    #[default]
    Normal = 0,
    /// Use JavaScript strings.
    String = 1,
    /// Use JavaScript numbers.
    Number = 2,
    Unknown(i32),
}
impl From<EnumOrUnknown<protobuf::descriptor::field_options::JSType>> for JsType {
    fn from(value: EnumOrUnknown<protobuf::descriptor::field_options::JSType>) -> Self {
        match value.enum_value() {
            Ok(v) => v.into(),
            Err(v) => Self::Unknown(v),
        }
    }
}
impl From<protobuf::descriptor::field_options::JSType> for JsType {
    fn from(value: protobuf::descriptor::field_options::JSType) -> Self {
        use protobuf::descriptor::field_options::JSType::*;
        match value {
            JS_NORMAL => Self::Normal,
            JS_STRING => Self::String,
            JS_NUMBER => Self::Number,
        }
    }
}

impl<'ast> Field<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::AccessReferences::references(self)
    }
}

impl<'ast> super::access::AccessReferences<'ast> for Field<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_option(self.0.reference.as_ref(), self.ast())
    }
}
impl super::access::AccessReferencesMut for FieldInner {
    fn references_mut(
        &mut self,
    ) -> impl '_ + Iterator<Item = &'_ mut super::reference::ReferenceInner> {
        self.reference.iter_mut()
    }
}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) message: MessageKey,
    pub(super) location: Location,
    pub(super) number: i32,
    pub(super) type_: FieldTypeInner,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) oneof_index: Option<i32>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<Label>,
    pub(super) options: ProtoFieldOpts,
    pub(super) reference: Option<ReferenceInner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ValueInner {
    Scalar(Scalar),
    Enum(EnumKey),
    Message(MessageKey),
}
impl Default for ValueInner {
    fn default() -> Self {
        Self::Scalar(Scalar::String)
    }
}

impl ValueInner {
    fn resolve_with<'ast>(&self, ast: &'ast Ast) -> Value<'ast> {
        match *self {
            Self::Scalar(s) => Value::Scalar(s),
            Self::Enum(key) => (key, ast).into(),
            Self::Message(key) => (key, ast).into(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Value<'ast> {
    Scalar(Scalar),
    Enum(Enum<'ast>),       // 14,
    Message(Message<'ast>), // 11,
}

impl<'ast> From<Message<'ast>> for Value<'ast> {
    fn from(v: Message<'ast>) -> Self {
        Self::Message(v)
    }
}

impl<'ast> From<Enum<'ast>> for Value<'ast> {
    fn from(v: Enum<'ast>) -> Self {
        Self::Enum(v)
    }
}

impl<'ast> From<Scalar> for Value<'ast> {
    fn from(v: Scalar) -> Self {
        Self::Scalar(v)
    }
}

impl<'ast> From<(MessageKey, &'ast Ast)> for Value<'ast> {
    fn from((key, ast): (MessageKey, &'ast Ast)) -> Self {
        Self::from(Message::from((key, ast)))
    }
}
impl<'ast> From<(EnumKey, &'ast Ast)> for Value<'ast> {
    fn from((key, ast): (EnumKey, &'ast Ast)) -> Self {
        Self::from(Enum::from((key, ast)))
    }
}

impl<'ast> fmt::Debug for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(s) => fmt::Debug::fmt(s, f),
            Self::Enum(e) => fmt::Debug::fmt(e, f),
            Self::Message(m) => fmt::Debug::fmt(m, f),
        }
    }
}

impl<'ast> Value<'ast> {
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
    #[must_use]
    pub const fn is_message(&self) -> bool {
        matches!(self, Self::Message(_))
    }
    #[must_use]
    pub const fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    #[must_use]
    pub const fn as_enum(&self) -> Option<Enum> {
        if let Self::Enum(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_scalar(&self) -> Option<Scalar> {
        if let Self::Scalar(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_message(&self) -> Option<Message> {
        if let Self::Message(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub const fn try_into_scalar(self) -> Result<Scalar, Self> {
        if let Self::Scalar(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn try_into_enum(self) -> Result<Enum<'ast>, Self> {
        if let Self::Enum(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn try_into_message(self) -> Result<Message<'ast>, Self> {
        if let Self::Message(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
impl ValueInner {
    pub(super) fn new(
        typ: field_descriptor_proto::Type,
        enum_: Option<EnumKey>,
        msg: Option<MessageKey>,
    ) -> Self {
        match typ {
            ProtoType::TYPE_ENUM => Self::Enum(enum_.unwrap()),
            ProtoType::TYPE_MESSAGE => Self::Message(msg.unwrap()),
            ProtoType::TYPE_GROUP => unreachable!(),
            _ => Self::Scalar(typ.try_into().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scalar {
    Double = 1,
    Float = 2,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
    /// negative values are likely.
    Int64 = 3,
    /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
    /// negative values are likely.
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    /// New in version 2.
    Bytes = 12,
    Uint32 = 13,
    Uint64 = 4,
    Sfixed32 = 15,
    Sfixed64 = 16,
    /// Uses ZigZag encoding.
    Sint32 = 17,
    /// Uses ZigZag encoding.
    Sint64 = 18,
}

impl TryFrom<field_descriptor_proto::Type> for Scalar {
    type Error = field_descriptor_proto::Type;

    fn try_from(typ: field_descriptor_proto::Type) -> Result<Self, Self::Error> {
        match typ {
            ProtoType::TYPE_DOUBLE => Ok(Self::Double),
            ProtoType::TYPE_FLOAT => Ok(Self::Float),
            ProtoType::TYPE_INT64 => Ok(Self::Int64),
            ProtoType::TYPE_UINT64 => Ok(Self::Uint64),
            ProtoType::TYPE_INT32 => Ok(Self::Int32),
            ProtoType::TYPE_FIXED64 => Ok(Self::Fixed64),
            ProtoType::TYPE_FIXED32 => Ok(Self::Fixed32),
            ProtoType::TYPE_BOOL => Ok(Self::Bool),
            ProtoType::TYPE_STRING => Ok(Self::String),
            ProtoType::TYPE_BYTES => Ok(Self::Bytes),
            ProtoType::TYPE_UINT32 => Ok(Self::Uint32),
            ProtoType::TYPE_SFIXED32 => Ok(Self::Sfixed32),
            ProtoType::TYPE_SFIXED64 => Ok(Self::Sfixed64),
            ProtoType::TYPE_SINT32 => Ok(Self::Sint32),
            ProtoType::TYPE_SINT64 => Ok(Self::Sint64),
            ProtoType::TYPE_ENUM | ProtoType::TYPE_GROUP | ProtoType::TYPE_MESSAGE => Err(typ),
        }
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Double => "double",
            Self::Float => "float",
            Self::Int64 => "int64",
            Self::Uint64 => "uint64",
            Self::Int32 => "int32",
            Self::Fixed64 => "fixed64",
            Self::Fixed32 => "fixed32",
            Self::Bool => "bool",
            Self::String => "string",
            Self::Bytes => "bytes",
            Self::Uint32 => "uint32",
            Self::Sfixed32 => "sfixed32",
            Self::Sfixed64 => "sfixed64",
            Self::Sint32 => "sint32",
            Self::Sint64 => "sint64",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapKey {
    Int64 = 3,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    String = 9,
    Uint32 = 13,
    Uint64 = 4,
    Sfixed32 = 15,
    Sfixed64 = 16,
    Sint32 = 17,
    Sint64 = 18,
}

impl TryFrom<FieldTypeInner> for MapKey {
    type Error = error::InvalidMapKey;

    fn try_from(value: FieldTypeInner) -> Result<Self, Self::Error> {
        match value {
            FieldTypeInner::Single(v) => match v {
                ValueInner::Scalar(scalar) => MapKey::try_from(scalar),
                ValueInner::Enum(_) => Err(error::InvalidMapKey {
                    type_: error::InvalidMapKeyType::Enum,
                    backtrace: Backtrace::capture(),
                }),
                ValueInner::Message(_) => Err(error::InvalidMapKey {
                    type_: error::InvalidMapKeyType::Message,
                    backtrace: Backtrace::capture(),
                }),
            },
            FieldTypeInner::Repeated(_) => Err(error::InvalidMapKey {
                backtrace: Backtrace::capture(),
                type_: error::InvalidMapKeyType::Repeated,
            }),
            FieldTypeInner::Map(_) => Err(error::InvalidMapKey {
                backtrace: Backtrace::capture(),
                type_: error::InvalidMapKeyType::Map,
            }),
        }
    }
}
impl TryFrom<Scalar> for MapKey {
    type Error = error::InvalidMapKey;

    fn try_from(scalar: Scalar) -> Result<Self, Self::Error> {
        match scalar {
            Scalar::Int64 => Ok(MapKey::Int64),
            Scalar::Int32 => Ok(MapKey::Int32),
            Scalar::Fixed64 => Ok(MapKey::Fixed64),
            Scalar::Fixed32 => Ok(MapKey::Fixed32),
            Scalar::String => Ok(MapKey::String),
            Scalar::Uint64 => Ok(MapKey::Uint64),
            Scalar::Uint32 => Ok(MapKey::Uint32),
            Scalar::Sfixed32 => Ok(MapKey::Sfixed32),
            Scalar::Sfixed64 => Ok(MapKey::Sfixed64),
            Scalar::Sint32 => Ok(MapKey::Sint32),
            Scalar::Sint64 => Ok(MapKey::Sint64),

            Scalar::Bytes | Scalar::Bool | Scalar::Float | Scalar::Double => {
                Err(error::InvalidMapKey {
                    backtrace: Backtrace::capture(),
                    type_: error::InvalidMapKeyType::Scalar(scalar),
                })
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Map<'ast> {
    pub key: MapKey,
    pub value: Value<'ast>,
}
impl fmt::Debug for Map<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}

// impl Copy for Map<'_> {}

impl<'ast> Map<'ast> {
    pub const fn new(key: MapKey, value: Value<'ast>) -> Self {
        Self { key, value }
    }
    pub const fn key(&self) -> MapKey {
        self.key
    }
    pub const fn value(&self) -> &Value<'ast> {
        &self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MapInner {
    pub(super) key: MapKey,
    pub(super) value: ValueInner,
}

#[derive(Clone, Debug)]
pub enum FieldType<'ast> {
    Single(Value<'ast>),
    Repeated(Value<'ast>),
    Map(Map<'ast>),
}

// impl Copy for Type<'_> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FieldTypeInner {
    Single(ValueInner),
    Repeated(ValueInner),
    Map(MapInner),
}
impl Default for FieldTypeInner {
    fn default() -> Self {
        Self::Single(ValueInner::default())
    }
}
