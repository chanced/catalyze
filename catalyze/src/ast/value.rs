use std::fmt;

use crate::error::{self, UnknownFieldType};

use super::{enum_::EnumKey, message::MessageKey, Ast, Enum, Message};
use protobuf::descriptor::field_descriptor_proto::{self, Type as ProtoType};
use snafu::Backtrace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Inner {
    Scalar(Scalar),
    Enum(EnumKey),
    Message(MessageKey),
}
impl Default for Inner {
    fn default() -> Self {
        Self::Scalar(Scalar::String)
    }
}

impl Inner {
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
impl Inner {
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

impl TryFrom<TypeInner> for MapKey {
    type Error = error::InvalidMapKey;

    fn try_from(value: TypeInner) -> Result<Self, Self::Error> {
        match value {
            TypeInner::Single(v) => match v {
                Inner::Scalar(scalar) => MapKey::try_from(scalar),
                Inner::Enum(_) => Err(error::InvalidMapKey {
                    type_: error::InvalidMapKeyType::Enum,
                    backtrace: Backtrace::capture(),
                }),
                Inner::Message(_) => Err(error::InvalidMapKey {
                    type_: error::InvalidMapKeyType::Message,
                    backtrace: Backtrace::capture(),
                }),
            },
            TypeInner::Repeated(_) => Err(error::InvalidMapKey {
                backtrace: Backtrace::capture(),
                type_: error::InvalidMapKeyType::Repeated,
            }),
            TypeInner::Map(_) => Err(error::InvalidMapKey {
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
    pub(super) value: Inner,
}

#[derive(Clone, Debug)]
pub enum Type<'ast> {
    Single(Value<'ast>),
    Repeated(Value<'ast>),
    Map(Map<'ast>),
}

// impl Copy for Type<'_> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TypeInner {
    Single(Inner),
    Repeated(Inner),
    Map(MapInner),
}
impl Default for TypeInner {
    fn default() -> Self {
        Self::Single(Inner::default())
    }
}
