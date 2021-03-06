use std::{any::Any, fmt};

use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type<'a> {
    Scalar(Scalar),
    Enum(&'a str),    //= 14,
    Message(&'a str), //= 11,
    /// not supported
    Group, //  = 10,
}
impl<'a> Type<'a> {
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group)
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
    pub fn is_message(&self) -> bool {
        matches!(self, Self::Message(_))
    }
    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }
}
// impl<'a> fmt::Display for Type<'a> {
//     fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Type::Scalar(_s) => todo!(),
//             Type::Enum(_) => todo!(),
//             Type::Message(_) => todo!(),
//             Type::Group => todo!(),
//         }
//     }
// }

impl<'a> From<&'a protobuf::descriptor::FieldDescriptorProto> for Type<'a> {
    fn from(fd: &'a protobuf::descriptor::FieldDescriptorProto) -> Self {
        match fd.field_type() {
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_DOUBLE => {
                Type::Scalar(Scalar::Double)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT => {
                Type::Scalar(Scalar::Float)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT64 => {
                Type::Scalar(Scalar::Int64)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT64 => {
                Type::Scalar(Scalar::Uint64)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32 => {
                Type::Scalar(Scalar::Int32)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED64 => {
                Type::Scalar(Scalar::Fixed64)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED32 => {
                Type::Scalar(Scalar::Fixed32)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL => {
                Type::Scalar(Scalar::Bool)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => {
                Type::Scalar(Scalar::String)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => {
                Type::Scalar(Scalar::Bytes)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32 => {
                Type::Scalar(Scalar::Uint32)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED32 => {
                Type::Scalar(Scalar::Sfixed32)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED64 => {
                Type::Scalar(Scalar::Sfixed64)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT32 => {
                Type::Scalar(Scalar::Sint32)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT64 => {
                Type::Scalar(Scalar::Sint64)
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM => {
                Type::Enum(fd.type_name())
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE => {
                Type::Message(fd.type_name())
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_GROUP => Type::Group,
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
            Scalar::Double => "double",
            Scalar::Float => "float",
            Scalar::Int64 => "int64",
            Scalar::Uint64 => "uint64",
            Scalar::Int32 => "int32",
            Scalar::Fixed64 => "fixed64",
            Scalar::Fixed32 => "fixed32",
            Scalar::Bool => "bool",
            Scalar::String => "string",
            Scalar::Bytes => "bytes",
            Scalar::Uint32 => "uint32",
            Scalar::Enum => "enum",
            Scalar::Sfixed32 => "sfixed32",
            Scalar::Sfixed64 => "sfixed64",
            Scalar::Sint32 => "sint32",
            Scalar::Sint64 => "sint64",
        };
        write!(f, "{}", s)
    }
}
impl TryFrom<i32> for Scalar {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Scalar::Double),
            2 => Ok(Scalar::Float),
            3 => Ok(Scalar::Int64),
            4 => Ok(Scalar::Uint64),
            5 => Ok(Scalar::Int32),
            6 => Ok(Scalar::Fixed64),
            7 => Ok(Scalar::Fixed32),
            8 => Ok(Scalar::Bool),
            9 => Ok(Scalar::String),
            12 => Ok(Scalar::Bytes),
            13 => Ok(Scalar::Uint32),
            14 => Ok(Scalar::Enum),
            15 => Ok(Scalar::Sfixed32),
            16 => Ok(Scalar::Sfixed64),
            17 => Ok(Scalar::Sint32),
            18 => Ok(Scalar::Sint64),
            v => bail!("Unknown Scalar: {}", v),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
}

impl From<protobuf::descriptor::field_options::CType> for CType {
    fn from(t: protobuf::descriptor::field_options::CType) -> Self {
        match t {
            protobuf::descriptor::field_options::CType::STRING => CType::String,
            protobuf::descriptor::field_options::CType::CORD => CType::Cord,
            protobuf::descriptor::field_options::CType::STRING_PIECE => CType::StringPiece,
        }
    }
}

impl TryFrom<Option<i32>> for CType {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => CType::try_from(v),
            None => bail!("CType is None"),
        }
    }
}
impl TryFrom<i32> for CType {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CType::String),
            1 => Ok(CType::Cord),
            2 => Ok(CType::StringPiece),
            _ => bail!("invalid CType: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsType {
    /// Use the default type.
    Normal = 0,
    /// Use JavaScript strings.
    String = 1,
    /// Use JavaScript numbers.
    Number = 2,
}

impl From<protobuf::descriptor::field_options::JSType> for JsType {
    fn from(value: protobuf::descriptor::field_options::JSType) -> Self {
        match value {
            protobuf::descriptor::field_options::JSType::JS_NORMAL => JsType::Normal,
            protobuf::descriptor::field_options::JSType::JS_STRING => JsType::String,
            protobuf::descriptor::field_options::JSType::JS_NUMBER => JsType::Number,
        }
    }
}

impl TryFrom<Option<i32>> for JsType {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => JsType::try_from(v),
            None => bail!("JsType is None"),
        }
    }
}
impl TryFrom<i32> for JsType {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JsType::Normal),
            1 => Ok(JsType::String),
            2 => Ok(JsType::Number),
            _ => bail!("invalid JsType {}", value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
}

impl From<protobuf::descriptor::field_descriptor_proto::Label> for Label {
    fn from(value: protobuf::descriptor::field_descriptor_proto::Label) -> Self {
        match value {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REQUIRED => Label::Required,
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL => Label::Optional,
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED => Label::Repeated,
        }
    }
}

impl TryFrom<Option<i32>> for Label {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => Label::try_from(v),
            None => bail!("Label is None"),
        }
    }
}
impl TryFrom<i32> for Label {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Label::Required),
            2 => Ok(Label::Optional),
            3 => Ok(Label::Repeated),
            _ => bail!("invalid Label {}", value),
        }
    }
}

/// Generated classes can be optimized for speed or code size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum OptimizeMode {
    /// Generate complete code for parsing, serialization,
    Speed = 1,
    /// etc.
    ///
    /// Use ReflectionOps to implement these methods.
    CodeSize = 2,
    /// Generate code using MessageLite and the lite runtime.
    LiteRuntime = 3,
}

impl From<protobuf::descriptor::file_options::OptimizeMode> for OptimizeMode {
    fn from(value: protobuf::descriptor::file_options::OptimizeMode) -> Self {
        match value {
            protobuf::descriptor::file_options::OptimizeMode::SPEED => OptimizeMode::Speed,
            protobuf::descriptor::file_options::OptimizeMode::CODE_SIZE => OptimizeMode::CodeSize,
            protobuf::descriptor::file_options::OptimizeMode::LITE_RUNTIME => {
                OptimizeMode::LiteRuntime
            }
        }
    }
}

impl TryFrom<Option<i32>> for OptimizeMode {
    type Error = anyhow::Error;
    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => OptimizeMode::try_from(v),
            None => bail!("OptimizeMode cannot be None"),
        }
    }
}
impl TryFrom<i32> for OptimizeMode {
    type Error = anyhow::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OptimizeMode::Speed),
            2 => Ok(OptimizeMode::CodeSize),
            3 => Ok(OptimizeMode::LiteRuntime),
            _ => bail!("OptimizeMode cannot be {}", value),
        }
    }
}

/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum IdempotencyLevel {
    IdempotencyUnknown = 0,
    /// implies idempotent
    NoSideEffects = 1,
    /// idempotent, but may have side effects
    Idempotent = 2,
}
impl From<protobuf::descriptor::method_options::IdempotencyLevel> for IdempotencyLevel {
    fn from(value: protobuf::descriptor::method_options::IdempotencyLevel) -> Self {
        match value {
            protobuf::descriptor::method_options::IdempotencyLevel::IDEMPOTENT => {
                IdempotencyLevel::Idempotent
            }
            protobuf::descriptor::method_options::IdempotencyLevel::NO_SIDE_EFFECTS => {
                IdempotencyLevel::NoSideEffects
            }
            protobuf::descriptor::method_options::IdempotencyLevel::IDEMPOTENCY_UNKNOWN => {
                IdempotencyLevel::IdempotencyUnknown
            }
        }
    }
}
impl TryFrom<Option<i32>> for IdempotencyLevel {
    type Error = anyhow::Error;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            Some(v) => IdempotencyLevel::try_from(v),
            None => bail!("IdempotencyLevel can not be None"),
        }
    }
}
impl TryFrom<i32> for IdempotencyLevel {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IdempotencyLevel::IdempotencyUnknown),
            1 => Ok(IdempotencyLevel::NoSideEffects),
            2 => Ok(IdempotencyLevel::Idempotent),
            _ => bail!("IdempotencyLevel cannot be {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Syntax {
    pub fn supports_required_prefix(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto2(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto3(&self) -> bool {
        match self {
            Syntax::Proto2 => false,
            Syntax::Proto3 => true,
        }
    }
}

impl TryFrom<String> for Syntax {
    type Error = anyhow::Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        match v.as_str() {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            "" => Ok(Syntax::Proto2),
            _ => bail!("invalid syntax: {}", v),
        }
    }
}

impl ToString for Syntax {
    fn to_string(&self) -> String {
        match self {
            Syntax::Proto2 => "proto2",
            Syntax::Proto3 => "proto3",
        }
        .to_string()
    }
}
impl From<&str> for Syntax {
    fn from(v: &str) -> Self {
        match v.to_lowercase().as_str() {
            "proto2" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            _ => Syntax::Proto2,
        }
    }
}
