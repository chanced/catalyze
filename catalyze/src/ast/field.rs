use crate::{
    ast::{impl_traits_and_methods, uninterpreted::UninterpretedOption, FullyQualifiedName},
    error::HydrationFailed,
};
use ::std::vec::Vec;
use protobuf::{
    descriptor::{field_descriptor_proto, field_options::CType as ProtobufCType, FieldOptions},
    EnumOrUnknown, SpecialFields,
};

use super::{
    access::NodeKeys,
    file, location, message, node, package,
    reference::{self, References},
    resolve::Resolver,
    value::{self, Value},
    Name,
};

pub struct Field<'ast>(Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(Field, Key, Inner);

pub(super) type Ident = node::Ident<Key>;
pub(super) type Table = super::table::Table<Key, Inner>;

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) file: file::Key,
    pub(super) value: value::Inner,
    pub(super) package: Option<package::Key>,
    pub(super) message: message::Key,
    pub(super) location: location::Detail,
    pub(super) number: Option<i32>,
    pub(super) type_: Option<EnumOrUnknown<field_descriptor_proto::Type>>,
    pub(super) type_name: Option<String>,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) oneof_index: Option<i32>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<protobuf::EnumOrUnknown<field_descriptor_proto::Label>>,
    pub(super) options: protobuf::MessageField<FieldOptions>,
    pub(super) proto_type: field_descriptor_proto::Type,
    pub(super) reference: Option<reference::Inner>,
}

#[derive(Debug, Default, Clone)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Name,
    value: value::Inner,
    node_path: Box<[i32]>,
    span: location::Span,
    comments: Option<location::Comments>,
    number: i32,
    label: Option<Label>,

    proto_type: field_descriptor_proto::Type,
    ///  If type_name is set, this need not be set.  If both this and type_name
    ///  are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    type_: TypeInner,
    ///  For message and enum types, this is the name of the type.  If the name
    ///  starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    ///  rules are used to find the type (i.e. first the nested types within
    /// this  message are searched, then within the parent, on up to the
    /// root  namespace).
    type_name: Option<String>,
    ///  For numeric types, contains the original text representation of the
    /// value.  For booleans, "true" or "false".
    ///  For strings, contains the default text contents (not escaped in any
    /// way).  For bytes, contains the C escaped value.  All bytes >= 128
    /// are escaped.  TODO(kenton):  Base-64 encode?
    default_value: Option<String>,
    ///  If set, gives the index of a oneof in the containing type's oneof_decl
    ///  list.  This field is a member of that oneof.
    oneof_index: Option<i32>,
    ///  JSON name of this field. The value is set by protocol compiler. If the
    ///  user has set a "json_name" option on this field, that option's value
    ///  will be used. Otherwise, it's deduced from the field's name by
    /// converting  it to camelCase.
    json_name: Option<String>,
    ///  The ctype option instructs the C++ code generator to use a different
    ///  representation of the field than it normally would.  See the specific
    ///  options below.  This option is not yet implemented in the open source
    ///  release -- sorry, we'll try to include it in a future version!
    // @@protoc_insertion_point(field:google.protobuf.FieldOptions.ctype)
    ctype: Option<CType>,
    ///  The packed option can be enabled for repeated primitive fields to
    /// enable  a more efficient representation on the wire. Rather than
    /// repeatedly  writing the tag and type for each element, the entire
    /// array is encoded as  a single length-delimited blob. In proto3, only
    /// explicit setting it to  false will avoid using packed encoding.
    packed: bool,
    ///  The jstype option determines the JavaScript type used for values of the
    ///  field.  The option is permitted only for 64 bit integral and fixed
    /// types  (int64, uint64, sint64, fixed64, sfixed64).  A field with
    /// jstype JS_STRING  is represented as JavaScript string, which avoids
    /// loss of precision that  can happen when a large value is converted
    /// to a floating point JavaScript.  Specifying JS_NUMBER for the jstype
    /// causes the generated JavaScript code to  use the JavaScript "number"
    /// type.  The behavior of the default option  JS_NORMAL is
    /// implementation dependent.
    ///
    ///  This option is an enum to permit additional types to be added, e.g.
    ///  goog.math.Integer.
    jstype: Option<JsType>,
    ///  Should this field be parsed lazily?  Lazy applies only to message-type
    ///  fields.  It means that when the outer message is initially parsed, the
    ///  inner message's contents will not be parsed but instead stored in
    /// encoded  form.  The inner message will actually be parsed when it is
    /// first accessed.
    ///
    ///  This is only a hint.  Implementations are free to choose whether to use
    ///  eager or lazy parsing regardless of the value of this option.  However,
    ///  setting this option true suggests that the protocol author believes
    /// that  using lazy parsing on this field is worth the additional
    /// bookkeeping  overhead typically needed to implement it.
    ///
    ///  This option does not affect the public interface of any generated code;
    ///  all method signatures remain the same.  Furthermore, thread-safety of
    /// the  interface is not affected by this option; const methods remain
    /// safe to  call from multiple threads concurrently, while non-const
    /// methods continue  to require exclusive access.
    ///
    ///
    ///  Note that implementations may choose not to check required fields
    /// within  a lazy sub-message.  That is, calling IsInitialized() on the
    /// outer message  may return true even if the inner message has missing
    /// required fields.  This is necessary because otherwise the inner
    /// message would have to be  parsed in order to perform the check,
    /// defeating the purpose of lazy  parsing.  An implementation which
    /// chooses not to check required fields  must be consistent about it.
    /// That is, for any particular sub-message, the  implementation must
    /// either *always* check its required fields, or *never*  check its
    /// required fields, regardless of whether or not the message has
    ///  been parsed.
    lazy: bool,
    ///  Is this field deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for accessors, or it will be completely ignored; in the very least,
    /// this  is a formalization for deprecating fields.
    deprecated: bool,
    ///  For Google-internal migration only. Do not use.
    weak: bool,
    ///  The parser stores options it doesn't recognize here. See above.
    uninterpreted_options: Vec<UninterpretedOption>,
    ///  If true, this is a proto3 "optional". When a proto3 field is optional,
    /// it  tracks presence regardless of field type.
    ///
    ///  When proto3_optional is true, this field must be belong to a oneof to
    ///  signal to old proto3 clients that presence is tracked for this field.
    /// This  oneof is known as a "synthetic" oneof, and this field must be
    /// its sole  member (each proto3 optional field gets its own synthetic
    /// oneof). Synthetic  oneofs exist in the descriptor only, and do not
    /// generate any API. Synthetic  oneofs must be ordered after all "real"
    /// oneofs.
    ///
    ///  For message fields, proto3_optional doesn't create any semantic change,
    ///  since non-repeated message fields always track presence. However it
    /// still  indicates the semantic detail of whether the user wrote
    /// "optional" or not.  This can be useful for round-tripping the .proto
    /// file. For consistency we  give message fields a synthetic oneof
    /// also, even though it is not required  to track presence. This is
    /// especially important because the parser can't  tell if a field is a
    /// message or an enum, so it must always create a  synthetic oneof.
    ///
    ///  Proto2 optional fields do not set this flag, because they already
    /// indicate  optional with `LABEL_OPTIONAL`.
    proto3_optional: Option<bool>,
    package: Option<package::Key>,
    reference: Option<reference::Inner>,
    file: file::Key,

    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}

impl Inner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            value,
            location,
            file,
            number,
            label,
            options,
            name,
            message: _,
            package: _,
            type_: _,
            type_name,
            default_value,
            json_name,
            proto3_optional,
            oneof_index,
            special_fields: _,
            proto_type,
            reference: _,
        } = hydrate;

        self.name = name;
        self.file = file;
        self.value = value;

        self.type_name = type_name;
        self.default_value = default_value;
        self.oneof_index = oneof_index;
        self.json_name = json_name;
        self.proto3_optional = proto3_optional;
        self.number = number.unwrap();
        self.label = label.map(Into::into);
        self.proto_type = proto_type;

        self.hydrate_location(location);
        self.hydrate_options(options.unwrap_or_default())?;
        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: FieldOptions) -> Result<(), HydrationFailed> {
        let FieldOptions {
            ctype,
            packed,
            jstype,
            lazy,
            deprecated,
            weak,
            uninterpreted_option,
            special_fields,
        } = opts;

        self.ctype = ctype.map(Into::into);
        self.packed = packed.unwrap_or(false);
        self.jstype = jstype.map(Into::into);
        self.lazy = lazy.unwrap_or(false);
        self.deprecated = deprecated.unwrap_or(false);
        self.weak = weak.unwrap_or(false);
        self.uninterpreted_options = uninterpreted_option.into_iter().map(Into::into).collect();
        self.options_special_fields = special_fields;
        Ok(())
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        std::iter::empty()
    }
}

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Label {
    Required = 1,
    Optional = 2,
    Repeated = 3,
    Unkown(i32),
}
impl From<EnumOrUnknown<field_descriptor_proto::Label>> for Label {
    fn from(value: EnumOrUnknown<field_descriptor_proto::Label>) -> Self {
        match value.enum_value() {
            Ok(v) => v.into(),
            Err(v) => Self::Unkown(v),
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CType {
    /// Default mode.
    String = 0,
    Cord = 1,
    StringPiece = 2,
    Unknown(i32),
}
impl From<EnumOrUnknown<ProtobufCType>> for CType {
    fn from(value: EnumOrUnknown<ProtobufCType>) -> Self {
        match value.enum_value() {
            Ok(v) => v.into(),
            Err(v) => Self::Unknown(v),
        }
    }
}
impl From<&ProtobufCType> for CType {
    fn from(value: &ProtobufCType) -> Self {
        match value {
            ProtobufCType::STRING => Self::String,
            ProtobufCType::CORD => Self::Cord,
            ProtobufCType::STRING_PIECE => Self::StringPiece,
        }
    }
}

impl From<ProtobufCType> for CType {
    fn from(value: ProtobufCType) -> Self {
        Self::from(&value)
    }
}

#[derive(Clone, Debug)]
pub enum Type<'ast> {
    Single(Value<'ast>),
    Repeated(Value<'ast>),
    Map(value::Map<'ast>),
    Unknown(i32),
}

// impl Copy for Type<'_> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TypeInner {
    Single(value::Inner),
    Repeated(value::Inner),
    Map(value::MapInner),
    Unknown(i32),
}
impl Default for TypeInner {
    fn default() -> Self {
        Self::Unknown(0)
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
        super::access::References::references(self)
    }
}

impl<'ast> super::access::References<'ast> for Field<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_option(self.0.reference, self.ast())
    }
}
impl super::access::ReferencesMut for Inner {
    fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut super::reference::Inner> {
        self.reference.iter_mut()
    }
}
