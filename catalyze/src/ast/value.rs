use std::fmt;

use protobuf::descriptor::field_descriptor_proto;

use super::{enum_, message, Ast, Enum, Message};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Inner {
    Scalar(Scalar),
    Enum(enum_::Key),
    Message(message::Key),
    Unknown(i32),
}
impl Default for Inner {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl Inner {
    fn resolve_with<'ast>(&self, ast: &'ast Ast) -> Value<'ast> {
        match *self {
            Self::Scalar(s) => Value::Scalar(s),
            Self::Enum(key) => (key, ast).into(),
            Self::Message(key) => (key, ast).into(),
            Self::Unknown(u) => Value::Unknown(u),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Value<'ast> {
    Scalar(Scalar),
    Enum(Enum<'ast>),       // 14,
    Message(Message<'ast>), // 11,
    // Group = 10, not supported
    Unknown(i32),
}

impl<'ast> From<i32> for Value<'ast> {
    fn from(v: i32) -> Self {
        Self::Unknown(v)
    }
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

impl<'ast> From<(message::Key, &'ast Ast)> for Value<'ast> {
    fn from((key, ast): (message::Key, &'ast Ast)) -> Self {
        Self::from(Message::from((key, ast)))
    }
}
impl<'ast> From<(enum_::Key, &'ast Ast)> for Value<'ast> {
    fn from((key, ast): (enum_::Key, &'ast Ast)) -> Self {
        Self::from(Enum::from((key, ast)))
    }
}

impl<'ast> fmt::Debug for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(s) => fmt::Debug::fmt(s, f),
            Self::Enum(e) => fmt::Debug::fmt(e, f),
            Self::Message(m) => fmt::Debug::fmt(m, f),
            Self::Unknown(i) => fmt::Debug::fmt(i, f),
        }
    }
}

impl<'ast> Value<'ast> {
    /// Returns `true` if the type is [`Unknown`].
    ///
    /// [`Unknown`]: Type::Unknown
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(..))
    }

    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
    #[must_use]
    pub const fn is_group(&self) -> bool {
        matches!(self, Self::Unknown(10))
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

    #[must_use]
    pub const fn as_unknown(&self) -> Option<i32> {
        if let Self::Unknown(v) = self {
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

    pub const fn try_into_unknown(self) -> Result<i32, Self> {
        if let Self::Unknown(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
impl Inner {
    pub(super) fn new(
        typ: field_descriptor_proto::Type,
        enum_: Option<enum_::Key>,
        msg: Option<message::Key>,
    ) -> Self {
        use field_descriptor_proto::Type as ProtoType;
        match typ {
            ProtoType::TYPE_ENUM => Self::Enum(enum_.unwrap()),
            ProtoType::TYPE_MESSAGE => Self::Message(msg.unwrap()),

            ProtoType::TYPE_DOUBLE => Self::Scalar(Scalar::Double),
            ProtoType::TYPE_FLOAT => Self::Scalar(Scalar::Float),
            ProtoType::TYPE_INT64 => Self::Scalar(Scalar::Int64),
            ProtoType::TYPE_UINT64 => Self::Scalar(Scalar::Uint64),
            ProtoType::TYPE_INT32 => Self::Scalar(Scalar::Int32),
            ProtoType::TYPE_FIXED64 => Self::Scalar(Scalar::Fixed64),
            ProtoType::TYPE_FIXED32 => Self::Scalar(Scalar::Fixed32),
            ProtoType::TYPE_BOOL => Self::Scalar(Scalar::Bool),
            ProtoType::TYPE_STRING => Self::Scalar(Scalar::String),
            ProtoType::TYPE_BYTES => Self::Scalar(Scalar::Bytes),
            ProtoType::TYPE_UINT32 => Self::Scalar(Scalar::Uint32),
            ProtoType::TYPE_SFIXED32 => Self::Scalar(Scalar::Sfixed32),
            ProtoType::TYPE_SFIXED64 => Self::Scalar(Scalar::Sfixed64),
            ProtoType::TYPE_SINT32 => Self::Scalar(Scalar::Sint32),
            ProtoType::TYPE_SINT64 => Self::Scalar(Scalar::Sint64),
            ProtoType::TYPE_GROUP => unreachable!(),
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
    Uint64 = 4,
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
    Enum = 14,
    Sfixed32 = 15,
    Sfixed64 = 16,
    /// Uses ZigZag encoding.
    Sint32 = 17,
    /// Uses ZigZag encoding.
    Sint64 = 18,
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
            Self::Enum => "enum",
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
    Uint64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    String = 9,
    Uint32 = 13,
    Sfixed32 = 15,
    Sfixed64 = 16,
    Sint32 = 17,
    Sint64 = 18,
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
    key: MapKey,
    value: Inner,
}
