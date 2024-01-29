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
    pub(super) package: Option<package::Key>,
    pub(super) message: message::Key,
    pub(super) location: location::Detail,
    pub(super) number: Option<i32>,
    pub(super) type_: TypeInner,
    pub(super) type_name: Option<String>,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) oneof_index: Option<i32>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<protobuf::EnumOrUnknown<field_descriptor_proto::Label>>,
    pub(super) options: protobuf::MessageField<FieldOptions>,
    pub(super) reference: Option<reference::Inner>,
}

#[derive(Debug, Default, Clone)]
pub(super) struct Inner {
    pub(super) key: Key,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) node_path: Box<[i32]>,
    pub(super) span: location::Span,
    pub(super) comments: Option<location::Comments>,
    pub(super) number: i32,
    pub(super) label: Option<Label>,
    pub(super) type_: TypeInner,
    pub(super) type_name: Option<String>,
    pub(super) default_value: Option<String>,
    pub(super) oneof_index: Option<i32>,
    pub(super) json_name: Option<String>,
    pub(super) ctype: Option<CType>,
    pub(super) packed: bool,
    pub(super) jstype: Option<JsType>,
    pub(super) lazy: bool,
    pub(super) deprecated: bool,
    pub(super) weak: bool,
    pub(super) uninterpreted_options: Vec<UninterpretedOption>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) package: Option<package::Key>,
    pub(super) reference: Option<reference::Inner>,
    pub(super) file: file::Key,
    pub(super) special_fields: SpecialFields,
    pub(super) options_special_fields: SpecialFields,
}

impl Inner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            location,
            file,
            number,
            label,
            options,
            name,
            message,
            package,
            type_,
            type_name,
            default_value,
            json_name,
            proto3_optional,
            oneof_index,
            special_fields,
            reference,
        } = hydrate;

        self.name = name;
        self.file = file;

        self.type_name = type_name;
        self.default_value = default_value;
        self.oneof_index = oneof_index;
        self.json_name = json_name;
        self.proto3_optional = proto3_optional;
        self.number = number.unwrap();
        self.label = label.map(Into::into);
        self.special_fields = special_fields;
        self.type_ = type_;
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
}

// impl Copy for Type<'_> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TypeInner {
    Single(value::Inner),
    Repeated(value::Inner),
    Map(value::MapInner),
}
impl Default for TypeInner {
    fn default() -> Self {
        Self::Single(value::Inner::default())
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
