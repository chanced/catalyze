use std::slice;

use crate::{
    ast::{impl_traits_and_methods, uninterpreted::UninterpretedOption, FullyQualifiedName},
    error::HydrationFailed,
};
use ahash::HashMap;
use protobuf::{
    descriptor::{
        self, field_descriptor_proto, field_options::CType as ProtobufCType,
        FieldOptions as ProtoFieldOpts,
    },
    EnumOrUnknown, SpecialFields,
};

use super::{
    access::{
        AccessComments, AccessFile, AccessFqn, AccessKey, AccessName, AccessNodeKeys,
        AccessReferences,
    },
    collection::Collection,
    file::FileKey,
    location::{Comments, Location},
    message::MessageKey,
    node,
    package::PackageKey,
    reference::{ReferenceInner, References},
    resolve::Resolver,
    uninterpreted::into_uninterpreted_options,
    value, Ast, File, Name, Span,
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
    pub(super) field_type: value::TypeInner,
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
    pub(super) type_: value::TypeInner,
    pub(super) default_value: Option<String>,
    pub(super) json_name: Option<String>,
    pub(super) proto3_optional: Option<bool>,
    pub(super) oneof_index: Option<i32>,
    pub(super) special_fields: protobuf::SpecialFields,
    pub(super) label: Option<Label>,
    pub(super) options: ProtoFieldOpts,
    pub(super) reference: Option<ReferenceInner>,
}
