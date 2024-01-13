use super::{
    access, r#enum, extension,
    field::{self},
    file, impl_traits_and_methods, message,
    oneof::{self},
    package,
    reference::{ReferenceInner, References},
    ContainerKey, FullyQualifiedName, ReservedRange, Resolver, State, UninterpretedOption,
};
use protobuf::descriptor::MessageOptions;

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Inner {
    state: State,
    fqn: FullyQualifiedName,
    name: String,
    container: ContainerKey,
    fields: Vec<field::Key>,
    enums: Vec<r#enum::Key>,
    messages: Vec<message::Key>,
    oneofs: Vec<oneof::Key>,
    real_oneofs: Vec<oneof::Key>,
    synthetic_oneofs: Vec<oneof::Key>,
    defined_extensions: Vec<extension::Key>,
    applied_extensions: Vec<extension::Key>,
    dependents: Vec<file::Key>,
    referenced_by: Vec<ReferenceInner>,
    references: Vec<ReferenceInner>,

    reserved_ranges: Vec<ReservedRange>,
    ///  Reserved field names, which may not be used by fields in the same
    /// message.  
    ///
    /// A given name may only be reserved once.
    reserved_names: Vec<String>,

    message_set_wire_format: bool,
    ///  Disables the generation of the standard "descriptor()" accessor, which
    /// can  conflict with a field of the same name.  This is meant to make
    /// migration  from proto1 easier; new code should avoid fields named
    /// "descriptor".
    no_standard_descriptor_accessor: bool,
    ///  Is this message deprecated?
    ///
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for the message, or it will be completely ignored; in the very least,
    ///  this is a formalization for deprecating messages.
    deprecated: bool,
    map_entry: bool,
    ///  The parser stores options it doesn't recognize here. See above.
    uninterpreted_options: Vec<UninterpretedOption>,
    unkown_option_fields: protobuf::UnknownFields,

    package: Option<package::Key>,
    file: file::Key,
}

impl super::access::ReferencesMut for Inner {
    fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
        self.references
            .iter_mut()
            .chain(self.referenced_by.iter_mut())
    }
}

impl Inner {
    pub(super) fn set_fields(&mut self, fields: Vec<field::Key>) {
        self.fields = fields;
    }
    pub(super) fn set_enums(&mut self, enums: Vec<r#enum::Key>) {
        self.enums = enums;
    }
    pub(super) fn set_messages(&mut self, messages: Vec<message::Key>) {
        self.messages = messages;
    }
    pub(super) fn set_oneofs(&mut self, oneofs: Vec<oneof::Key>) {
        self.oneofs = oneofs;
    }
    pub(super) fn set_real_oneofs(&mut self, real_oneofs: Vec<oneof::Key>) {
        self.real_oneofs = real_oneofs;
    }
    pub(super) fn set_synthetic_oneofs(&mut self, synthetic_oneofs: Vec<oneof::Key>) {
        self.synthetic_oneofs = synthetic_oneofs;
    }
    pub(super) fn set_defined_extensions(&mut self, defined_extensions: Vec<extension::Key>) {
        self.defined_extensions = defined_extensions;
    }
    pub(super) fn set_applied_extensions(&mut self, applied_extensions: Vec<extension::Key>) {
        self.applied_extensions = applied_extensions;
    }
    pub(super) fn set_dependents(&mut self, dependents: Vec<file::Key>) {
        self.dependents = dependents;
    }
    pub(super) fn set_referenced_by(&mut self, referenced_by: Vec<ReferenceInner>) {
        self.referenced_by = referenced_by;
    }
    pub(super) fn set_container(&mut self, container: impl Into<ContainerKey>) {
        self.container = container.into();
    }
    pub(super) fn hydrate_options(&mut self, opts: MessageOptions) {
        self.message_set_wire_format = opts.message_set_wire_format();
        self.no_standard_descriptor_accessor = opts.no_standard_descriptor_accessor();
        self.deprecated = opts.deprecated();
        self.map_entry = opts.map_entry();
        self.uninterpreted_options = opts
            .uninterpreted_option
            .into_iter()
            .map(Into::into)
            .collect();
        self.unkown_option_fields = opts.special_fields.unknown_fields().clone();
    }
}

pub struct Message<'ast>(Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(Message, Key, Inner);

impl<'ast> Message<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        access::References::references(self)
    }
    pub fn referenced_by(&'ast self) -> References<'ast> {
        access::ReferencedBy::referenced_by(self)
    }
}
impl<'ast> access::References<'ast> for Message<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_slice(&self.0.references, self.ast())
    }
}
impl<'ast> access::ReferencedBy<'ast> for Message<'ast> {
    fn referenced_by(&'ast self) -> super::reference::References<'ast> {
        References::from_slice(&self.0.referenced_by, self.ast())
    }
}

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
    const ANY: &'static str = "Any";
    const API: &'static str = "Api";
    const BOOL_VALUE: &'static str = "BoolValue";
    const BYTES_VALUE: &'static str = "BytesValue";
    const DOUBLE_VALUE: &'static str = "DoubleValue";
    const DURATION: &'static str = "Duration";
    const EMPTY: &'static str = "Empty";
    const ENUM: &'static str = "Enum";
    const ENUM_VALUE: &'static str = "EnumValue";
    const FIELD: &'static str = "Field";
    const FIELD_KIND: &'static str = "FieldKind";
    const FIELD_MASK: &'static str = "FieldMask";
    const FLOAT_VALUE: &'static str = "FloatValue";
    const INT32_VALUE: &'static str = "Int32Value";
    const INT64_VALUE: &'static str = "Int64Value";
    const LIST_VALUE: &'static str = "ListValue";
    const METHOD: &'static str = "Method";
    const MIXIN: &'static str = "Mixin";
    const OPTION: &'static str = "Option";
    const SOURCE_CONTEXT: &'static str = "SourceContext";
    const STRING_VALUE: &'static str = "StringValue";
    const STRUCT: &'static str = "Struct";
    const TIMESTAMP: &'static str = "Timestamp";
    const TYPE: &'static str = "Type";
    const UINT32_VALUE: &'static str = "UInt32Value";
    const UINT64_VALUE: &'static str = "UInt64Value";
    const VALUE: &'static str = "Value";

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
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            Self::ANY => Ok(Self::Any),
            Self::API => Ok(Self::Api),
            Self::BOOL_VALUE => Ok(Self::BoolValue),
            Self::BYTES_VALUE => Ok(Self::BytesValue),
            Self::DOUBLE_VALUE => Ok(Self::DoubleValue),
            Self::DURATION => Ok(Self::Duration),
            Self::EMPTY => Ok(Self::Empty),
            Self::ENUM => Ok(Self::Enum),
            Self::ENUM_VALUE => Ok(Self::EnumValue),
            Self::FIELD => Ok(Self::Field),
            Self::FIELD_KIND => Ok(Self::FieldKind),
            Self::FIELD_MASK => Ok(Self::FieldMask),
            Self::FLOAT_VALUE => Ok(Self::FloatValue),
            Self::INT32_VALUE => Ok(Self::Int32Value),
            Self::INT64_VALUE => Ok(Self::Int64Value),
            Self::LIST_VALUE => Ok(Self::ListValue),
            Self::METHOD => Ok(Self::Method),
            Self::MIXIN => Ok(Self::Mixin),
            Self::OPTION => Ok(Self::Option),
            Self::SOURCE_CONTEXT => Ok(Self::SourceContext),
            Self::STRING_VALUE => Ok(Self::StringValue),
            Self::STRUCT => Ok(Self::Struct),
            Self::TIMESTAMP => Ok(Self::Timestamp),
            Self::TYPE => Ok(Self::Type),
            Self::UINT32_VALUE => Ok(Self::UInt32Value),
            Self::UINT64_VALUE => Ok(Self::UInt64Value),
            Self::VALUE => Ok(Self::Value),
            _ => Err(()),
        }
    }
}
