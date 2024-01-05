use ::std::vec::Vec;
use std::fmt;

use inherent::inherent;

use crate::{
    error::Error,
    fqn::{Fqn, FullyQualifiedName},
    message::{Message, WeakMessage},
    node::Upgrade,
    r#enum::{Enum, WeakEnum},
    uninterpreted_option::UninterpretedOption,
};

use protobuf::descriptor::field_descriptor_proto;
use protobuf::descriptor::field_options::CType as ProtobufCType;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
    Unkown(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
    Unknown(i32),
}
impl From<protobuf::EnumOrUnknown<protobuf::descriptor::field_options::CType>> for CType {
    fn from(value: protobuf::EnumOrUnknown<protobuf::descriptor::field_options::CType>) -> Self {
        match value.enum_value() {
            Ok(v) => v.into(),
            Err(v) => Self::Unknown(v),
        }
    }
}
impl From<&protobuf::descriptor::field_options::CType> for CType {
    fn from(value: &protobuf::descriptor::field_options::CType) -> Self {
        match value {
            ProtobufCType::STRING => Self::String,
            ProtobufCType::CORD => Self::Cord,
            ProtobufCType::STRING_PIECE => Self::StringPiece,
        }
    }
}

impl From<protobuf::descriptor::field_options::CType> for CType {
    fn from(value: protobuf::descriptor::field_options::CType) -> Self {
        Self::from(&value)
    }
}

// crate::EnumOrUnknown<field_options::JSType>

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

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub key: MapKey,
    pub value: FieldValue,
}
struct WeakMap {
    key: MapKey,
    value: WeakFieldValue,
}

impl Map {
    pub fn new(key: MapKey, value: FieldValue) -> Self {
        Self { key, value }
    }
    pub fn key(&self) -> MapKey {
        self.key
    }
    pub fn value(&self) -> &FieldValue {
        &self.value
    }
}

#[derive(Debug, Clone)]
enum WeakFieldType {
    Single(WeakFieldValue),
    Repeated(WeakFieldValue),
    Map(Map),
    Unknown(i32),
}
impl Upgrade for WeakFieldType {
    type Target = FieldType;
    fn upgrade(&self) -> Self::Target {
        match self {
            Self::Single(v) => Self::Target::Single(v.upgrade()),
            Self::Repeated(v) => Self::Target::Repeated(v.upgrade()),
            Self::Map(v) => Self::Target::Map {
                key: v.key,
                value: v.value.clone(),
            },
            Self::Unknown(v) => Self::Target::Unknown(*v),
        }
    }
}
impl Default for WeakFieldType {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

#[derive(Clone, Debug)]
enum WeakFieldValue {
    Scalar(Scalar),
    Enum(WeakEnum),       // 14,
    Message(WeakMessage), // 11,
    Unknown(i32),
}
impl Upgrade for WeakFieldValue {
    type Target = FieldValue;
    fn upgrade(&self) -> Self::Target {
        match self {
            Self::Scalar(v) => Self::Target::Scalar(*v),
            Self::Enum(v) => Self::Target::Enum(v.upgrade()),
            Self::Message(v) => Self::Target::Message(v.upgrade()),
            Self::Unknown(v) => Self::Target::Unknown(*v),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Single(FieldValue),
    Repeated(FieldValue),
    Map { key: MapKey, value: FieldValue },
    Unknown(i32),
}
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Scalar(Scalar),
    Enum(Enum),       // 14,
    Message(Message), // 11,
    // Group = 10, not supported
    Unknown(i32),
}
impl Default for FieldValue {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl FieldValue {
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
            Some(v.clone())
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
    pub fn as_message(&self) -> Option<Message> {
        if let Self::Message(v) = self {
            Some(v.clone())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_unknown(&self) -> Option<&i32> {
        if let Self::Unknown(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
impl FieldValue {
    pub(crate) fn new(
        typ: field_descriptor_proto::Type,
        enum_: Option<Enum>,
        msg: Option<Message>,
    ) -> Result<Self, Error> {
        use field_descriptor_proto::Type::*;
        match typ {
            TYPE_DOUBLE => Ok(Self::Scalar(Scalar::Double)),
            TYPE_FLOAT => Ok(Self::Scalar(Scalar::Float)),
            TYPE_INT64 => Ok(Self::Scalar(Scalar::Int64)),
            TYPE_UINT64 => Ok(Self::Scalar(Scalar::Uint64)),
            TYPE_INT32 => Ok(Self::Scalar(Scalar::Int32)),
            TYPE_FIXED64 => Ok(Self::Scalar(Scalar::Fixed64)),
            TYPE_FIXED32 => Ok(Self::Scalar(Scalar::Fixed32)),
            TYPE_BOOL => Ok(Self::Scalar(Scalar::Bool)),
            TYPE_STRING => Ok(Self::Scalar(Scalar::String)),
            TYPE_BYTES => Ok(Self::Scalar(Scalar::Bytes)),
            TYPE_UINT32 => Ok(Self::Scalar(Scalar::Uint32)),
            TYPE_SFIXED32 => Ok(Self::Scalar(Scalar::Sfixed32)),
            TYPE_SFIXED64 => Ok(Self::Scalar(Scalar::Sfixed64)),
            TYPE_SINT32 => Ok(Self::Scalar(Scalar::Sint32)),
            TYPE_SINT64 => Ok(Self::Scalar(Scalar::Sint64)),
            TYPE_ENUM => Ok(Self::Enum(enum_.unwrap())),
            TYPE_MESSAGE => Ok(Self::Message(msg.unwrap())),
            TYPE_GROUP => Err(Error::GroupNotSupported),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum JsType {
    /// Use the default type.
    Normal = 0,
    /// Use JavaScript strings.
    String = 1,
    /// Use JavaScript numbers.
    Number = 2,
    Unknown(i32),
}
impl From<protobuf::EnumOrUnknown<protobuf::descriptor::field_options::JSType>> for JsType {
    fn from(value: protobuf::EnumOrUnknown<protobuf::descriptor::field_options::JSType>) -> Self {
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

#[derive(Debug, Clone)]
struct Inner {
    fqn: FullyQualifiedName,
    name: String,
    number: i32,
    label: Option<Label>,
    ///  If type_name is set, this need not be set.  If both this and type_name
    ///  are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    field_type: WeakFieldType,
    ///  For message and enum types, this is the name of the type.  If the name
    ///  starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    ///  rules are used to find the type (i.e. first the nested types within this
    ///  message are searched, then within the parent, on up to the root
    ///  namespace).
    type_name: Option<String>,
    ///  For extensions, this is the name of the type being extended.  It is
    ///  resolved in the same manner as type_name.
    extendee: Option<String>,
    ///  For numeric types, contains the original text representation of the value.
    ///  For booleans, "true" or "false".
    ///  For strings, contains the default text contents (not escaped in any way).
    ///  For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    ///  TODO(kenton):  Base-64 encode?
    default_value: Option<String>,
    ///  If set, gives the index of a oneof in the containing type's oneof_decl
    ///  list.  This field is a member of that oneof.
    oneof_index: Option<i32>,
    ///  JSON name of this field. The value is set by protocol compiler. If the
    ///  user has set a "json_name" option on this field, that option's value
    ///  will be used. Otherwise, it's deduced from the field's name by converting
    ///  it to camelCase.
    json_name: Option<String>,
    ///  The ctype option instructs the C++ code generator to use a different
    ///  representation of the field than it normally would.  See the specific
    ///  options below.  This option is not yet implemented in the open source
    ///  release -- sorry, we'll try to include it in a future version!
    // @@protoc_insertion_point(field:google.protobuf.FieldOptions.ctype)
    ctype: Option<CType>,
    ///  The packed option can be enabled for repeated primitive fields to enable
    ///  a more efficient representation on the wire. Rather than repeatedly
    ///  writing the tag and type for each element, the entire array is encoded as
    ///  a single length-delimited blob. In proto3, only explicit setting it to
    ///  false will avoid using packed encoding.
    packed: bool,
    ///  The jstype option determines the JavaScript type used for values of the
    ///  field.  The option is permitted only for 64 bit integral and fixed types
    ///  (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    ///  is represented as JavaScript string, which avoids loss of precision that
    ///  can happen when a large value is converted to a floating point JavaScript.
    ///  Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    ///  use the JavaScript "number" type.  The behavior of the default option
    ///  JS_NORMAL is implementation dependent.
    ///
    ///  This option is an enum to permit additional types to be added, e.g.
    ///  goog.math.Integer.
    jstype: Option<JsType>,
    ///  Should this field be parsed lazily?  Lazy applies only to message-type
    ///  fields.  It means that when the outer message is initially parsed, the
    ///  inner message's contents will not be parsed but instead stored in encoded
    ///  form.  The inner message will actually be parsed when it is first accessed.
    ///
    ///  This is only a hint.  Implementations are free to choose whether to use
    ///  eager or lazy parsing regardless of the value of this option.  However,
    ///  setting this option true suggests that the protocol author believes that
    ///  using lazy parsing on this field is worth the additional bookkeeping
    ///  overhead typically needed to implement it.
    ///
    ///  This option does not affect the public interface of any generated code;
    ///  all method signatures remain the same.  Furthermore, thread-safety of the
    ///  interface is not affected by this option; const methods remain safe to
    ///  call from multiple threads concurrently, while non-const methods continue
    ///  to require exclusive access.
    ///
    ///
    ///  Note that implementations may choose not to check required fields within
    ///  a lazy sub-message.  That is, calling IsInitialized() on the outer message
    ///  may return true even if the inner message has missing required fields.
    ///  This is necessary because otherwise the inner message would have to be
    ///  parsed in order to perform the check, defeating the purpose of lazy
    ///  parsing.  An implementation which chooses not to check required fields
    ///  must be consistent about it.  That is, for any particular sub-message, the
    ///  implementation must either *always* check its required fields, or *never*
    ///  check its required fields, regardless of whether or not the message has
    ///  been parsed.
    lazy: bool,
    ///  Is this field deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for accessors, or it will be completely ignored; in the very least, this
    ///  is a formalization for deprecating fields.
    deprecated: bool,
    ///  For Google-internal migration only. Do not use.
    weak: bool,
    ///  The parser stores options it doesn't recognize here. See above.
    uninterpreted_option: Vec<UninterpretedOption>,
    ///  If true, this is a proto3 "optional". When a proto3 field is optional, it
    ///  tracks presence regardless of field type.
    ///
    ///  When proto3_optional is true, this field must be belong to a oneof to
    ///  signal to old proto3 clients that presence is tracked for this field. This
    ///  oneof is known as a "synthetic" oneof, and this field must be its sole
    ///  member (each proto3 optional field gets its own synthetic oneof). Synthetic
    ///  oneofs exist in the descriptor only, and do not generate any API. Synthetic
    ///  oneofs must be ordered after all "real" oneofs.
    ///
    ///  For message fields, proto3_optional doesn't create any semantic change,
    ///  since non-repeated message fields always track presence. However it still
    ///  indicates the semantic detail of whether the user wrote "optional" or not.
    ///  This can be useful for round-tripping the .proto file. For consistency we
    ///  give message fields a synthetic oneof also, even though it is not required
    ///  to track presence. This is especially important because the parser can't
    ///  tell if a field is a message or an enum, so it must always create a
    ///  synthetic oneof.
    ///
    ///  Proto2 optional fields do not set this flag, because they already indicate
    ///  optional with `LABEL_OPTIONAL`.
    // @@protoc_insertion_point(field:google.protobuf.FieldDescriptorProto.proto3_optional)
    pub proto3_optional: Option<bool>,
}

impl Inner {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn default_value(&self) -> Option<&String> {
        self.default_value.as_ref()
    }

    fn type_name(&self) -> Option<&String> {
        self.type_name.as_ref()
    }

    fn field_type(&self) -> FieldType {
        self.field_type.upgrade()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field(Inner);

#[inherent]
impl Fqn for Field {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
