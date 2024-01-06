use crate::{
    ast::{impl_traits, Access, Accessor, Ast, FullyQualifiedName, Nodes},
    field::{self, Field},
    file, message,
    oneof::{self, Oneof},
};

slotmap::new_key_type! {
    pub(crate) struct Key;
}

#[derive(Debug, PartialEq)]
pub(crate) struct Inner {
    fqn: FullyQualifiedName,
    fields: Nodes<field::Key>,
    messages: Nodes<message::Key>,
    oneofs: Nodes<oneof::Key>,
    real_oneofs: Nodes<oneof::Key>,
    synthetic_oneofs: Nodes<oneof::Key>,
    dependents: Nodes<file::Key>,
    applied_extensions: Nodes<oneof::Key>,
}

pub struct Message<'ast, A = Ast>(Accessor<'ast, Key, Inner, A>);
impl_traits!(Message, Key, Inner);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WellKnownMessage {
    /// Any contains an arbitrary serialized message along with a URL that
    /// describes the type of the serialized message.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any>
    Any,
    /// Api is a light-weight descriptor for a protocol buffer service.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Api>
    Api,
    /// Wrapper message for bool.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.BoolValue>
    BoolValue,
    /// Wrapper message for bytes.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#bytesvalue>
    BytesValue,
    /// Wrapper message for double.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#doublevalue>
    DoubleValue,
    /// A Duration represents a signed, fixed-length span of time represented as
    /// a count of seconds and fractions of seconds at nanosecond resolution. It
    /// is independent of any calendar and concepts like "day" or "month". It is
    /// related to Timestamp in that the difference between two Timestamp values
    /// is a Duration and it can be added or subtracted from a Timestamp. Range
    /// is approximately +-10,000 years.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration>
    Duration,
    /// A generic empty message that you can re-use to avoid defining duplicated
    /// empty messages in your APIs. A typical example is to use it as the
    /// request or the response type of an API method. For Instance:
    ///
    /// ```protobuf
    /// service Foo {
    ///     rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
    /// }
    /// ```
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty>
    Empty,
    /// Enum type definition.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enum>
    Enum,
    /// Enum value definition.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enumvalue>
    EnumValue,
    /// A single field of a message type.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#field>
    Field,
    FieldKind,
    /// FieldMask represents a set of symbolic field paths, for example:
    /// ```protobuf
    /// paths: "f.a"
    /// paths: "f.b.d"
    /// ```
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    FieldMask,
    /// Wrapper message for float.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#floatvalue>
    FloatValue,
    /// Wrapper message for int32.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int32value>
    Int32Value,
    /// Wrapper message for int64.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int64value>
    Int64Value,
    /// ListValue is a wrapper around a repeated field of values.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#listvalue>
    ListValue,
    /// Method represents a method of an api.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#method>
    Method,
    /// Declares an API to be included in this API. The including API must
    /// redeclare all the methods from the included API, but documentation and
    /// options are inherited as follows:
    ///
    /// If after comment and whitespace stripping, the documentation string of
    /// the redeclared method is empty, it will be inherited from the original
    /// method.
    ///
    /// Each annotation belonging to the service config (http, visibility) which
    /// is not set in the redeclared method will be inherited.
    ///
    /// If an http annotation is inherited, the path pattern will be modified as
    /// follows. Any version prefix will be replaced by the version of the
    /// including API plus the root path if specified.
    ///
    /// Example of a simple mixin:
    /// ```protobuf
    //     package google.acl.v1;
    /// service AccessControl {
    ///   // Get the underlying ACL object.
    ///   rpc GetAcl(GetAclRequest) returns (Acl) {
    ///     option (google.api.http).get = "/v1/{resource=**}:getAcl";
    ///   }
    /// }
    ///
    /// package google.storage.v2;
    /// service Storage {
    ///   //       rpc GetAcl(GetAclRequest) returns (Acl);
    ///
    ///   // Get a data record.
    ///   rpc GetData(GetDataRequest) returns (Data) {
    ///     option (google.api.http).get = "/v2/{resource=**}";
    ///   }
    /// }
    /// ```
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Mixin>
    Mixin,
    /// A protocol buffer option, which can be attached to a message, field,
    /// enumeration, etc.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#option>
    Option,
    /// SourceContext represents information about the source of a protobuf
    /// element, like the file in which it is defined.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#sourcecontext>
    SourceContext,
    /// Wrapper message for string.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#stringvalue>
    StringValue,
    /// Struct represents a structured data value, consisting of fields which
    /// map to dynamically typed values. In some languages, Struct might be
    /// supported by a native representation. For example, in scripting
    /// languages like JS a struct is represented as an object. The details of
    /// that representation are described together with the proto support for
    /// the language.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct>
    Struct,
    /// A Timestamp represents a point in time independent of any time zone or
    /// calendar, represented as seconds and fractions of seconds at nanosecond
    /// resolution in UTC Epoch time. It is encoded using the Proleptic
    /// Gregorian Calendar which extends the Gregorian calendar backwards to
    /// year one. It is encoded assuming all minutes are 60 seconds long, i.e.
    /// leap seconds are "smeared" so that no leap second table is needed for
    /// interpretation. Range is from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59.999999999Z. By restricting to that range, we ensure
    /// that we can convert to and from RFC 3339 date strings. See
    /// <https://www.ietf.org/rfc/rfc3339.txt.>
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#timestamp>
    Timestamp,
    /// A protocol buffer message type.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#type>
    Type,
    /// Wrapper message for uint32.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint32value>
    UInt32Value,
    /// Wrapper message for uint64.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint64value>
    UInt64Value,
    /// Value represents a dynamically typed value which can be either null, a
    /// number, a string, a boolean, a recursive struct value, or a list of
    /// values. A producer of value is expected to set one of that variants,
    /// absence of any variant indicates an error.
    ///
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#value>
    Value,
}
impl WellKnownMessage {
    pub(crate) const ANY: &'static str = "Any";
    pub(crate) const API: &'static str = "Api";
    pub(crate) const BOOL_VALUE: &'static str = "BoolValue";
    pub(crate) const BYTES_VALUE: &'static str = "BytesValue";
    pub(crate) const DOUBLE_VALUE: &'static str = "DoubleValue";
    pub(crate) const DURATION: &'static str = "Duration";
    pub(crate) const EMPTY: &'static str = "Empty";
    pub(crate) const ENUM: &'static str = "Enum";
    pub(crate) const ENUM_VALUE: &'static str = "EnumValue";
    pub(crate) const FIELD: &'static str = "Field";
    pub(crate) const FIELD_KIND: &'static str = "FieldKind";
    pub(crate) const FIELD_MASK: &'static str = "FieldMask";
    pub(crate) const FLOAT_VALUE: &'static str = "FloatValue";
    pub(crate) const INT32_VALUE: &'static str = "Int32Value";
    pub(crate) const INT64_VALUE: &'static str = "Int64Value";
    pub(crate) const LIST_VALUE: &'static str = "ListValue";
    pub(crate) const METHOD: &'static str = "Method";
    pub(crate) const MIXIN: &'static str = "Mixin";
    pub(crate) const OPTION: &'static str = "Option";
    pub(crate) const SOURCE_CONTEXT: &'static str = "SourceContext";
    pub(crate) const STRING_VALUE: &'static str = "StringValue";
    pub(crate) const STRUCT: &'static str = "Struct";
    pub(crate) const TIMESTAMP: &'static str = "Timestamp";
    pub(crate) const TYPE: &'static str = "Type";
    pub(crate) const UINT32_VALUE: &'static str = "UInt32Value";
    pub(crate) const UINT64_VALUE: &'static str = "UInt64Value";
    pub(crate) const VALUE: &'static str = "Value";

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Any => Self::ANY,
            Self::Api => Self::API,
            Self::BoolValue => Self::BOOL_VALUE,
            Self::BytesValue => Self::BYTES_VALUE,
            Self::DoubleValue => Self::DOUBLE_VALUE,
            Self::Duration => Self::DURATION,
            Self::Empty => Self::EMPTY,
            Self::Enum => Self::ENUM,
            Self::EnumValue => Self::ENUM_VALUE,
            Self::Field => Self::FIELD,
            Self::FieldKind => Self::FIELD_KIND,
            Self::FieldMask => Self::FIELD_MASK,
            Self::FloatValue => Self::FLOAT_VALUE,
            Self::Int32Value => Self::INT32_VALUE,
            Self::Int64Value => Self::INT64_VALUE,
            Self::ListValue => Self::LIST_VALUE,
            Self::Method => Self::METHOD,
            Self::Mixin => Self::MIXIN,
            Self::Option => Self::OPTION,
            Self::SourceContext => Self::SOURCE_CONTEXT,
            Self::StringValue => Self::STRING_VALUE,
            Self::Struct => Self::STRUCT,
            Self::Timestamp => Self::TIMESTAMP,
            Self::Type => Self::TYPE,
            Self::UInt32Value => Self::UINT32_VALUE,
            Self::UInt64Value => Self::UINT64_VALUE,
            Self::Value => Self::VALUE,
        }
    }
}
impl std::fmt::Display for WellKnownMessage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl std::str::FromStr for WellKnownMessage {
    type Err = ();
    fn from_str(s: &str) -> ::std::result::Result<WellKnownMessage, Self::Err> {
        match s {
            Self::ANY => Ok(WellKnownMessage::Any),
            Self::API => Ok(WellKnownMessage::Api),
            Self::BOOL_VALUE => Ok(WellKnownMessage::BoolValue),
            Self::BYTES_VALUE => Ok(WellKnownMessage::BytesValue),
            Self::DOUBLE_VALUE => Ok(WellKnownMessage::DoubleValue),
            Self::DURATION => Ok(WellKnownMessage::Duration),
            Self::EMPTY => Ok(WellKnownMessage::Empty),
            Self::ENUM => Ok(WellKnownMessage::Enum),
            Self::ENUM_VALUE => Ok(WellKnownMessage::EnumValue),
            Self::FIELD => Ok(WellKnownMessage::Field),
            Self::FIELD_KIND => Ok(WellKnownMessage::FieldKind),
            Self::FIELD_MASK => Ok(WellKnownMessage::FieldMask),
            Self::FLOAT_VALUE => Ok(WellKnownMessage::FloatValue),
            Self::INT32_VALUE => Ok(WellKnownMessage::Int32Value),
            Self::INT64_VALUE => Ok(WellKnownMessage::Int64Value),
            Self::LIST_VALUE => Ok(WellKnownMessage::ListValue),
            Self::METHOD => Ok(WellKnownMessage::Method),
            Self::MIXIN => Ok(WellKnownMessage::Mixin),
            Self::OPTION => Ok(WellKnownMessage::Option),
            Self::SOURCE_CONTEXT => Ok(WellKnownMessage::SourceContext),
            Self::STRING_VALUE => Ok(WellKnownMessage::StringValue),
            Self::STRUCT => Ok(WellKnownMessage::Struct),
            Self::TIMESTAMP => Ok(WellKnownMessage::Timestamp),
            Self::TYPE => Ok(WellKnownMessage::Type),
            Self::UINT32_VALUE => Ok(WellKnownMessage::UInt32Value),
            Self::UINT64_VALUE => Ok(WellKnownMessage::UInt64Value),
            Self::VALUE => Ok(WellKnownMessage::Value),
            _ => Err(()),
        }
    }
}
