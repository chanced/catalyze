use crate::ast::impl_traits_and_methods;

use super::{
    access::NodeKeys,
    extension_decl,
    field::{TypeInner, ValueInner},
    file, location, message, node, package,
    reference::{ReferenceInner, References},
    resolve,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName,
};

pub use super::field::{CType, JsType, Label};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) type Ident = node::Ident<Key>;
pub(super) type Table = super::table::Table<Key, Inner>;

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    name: Box<str>,
    value: ValueInner,
    block: extension_decl::Key,
    fqn: FullyQualifiedName,
    node_path: Vec<i32>,
    span: location::Span,
    comments: Option<location::Comments>,
    number: i32,
    label: Option<Label>,
    ///  If type_name is set, this need not be set.  If both this and type_name
    ///  are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
    field_type: TypeInner,
    ///  For message and enum types, this is the name of the type.  If the name
    ///  starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    ///  rules are used to find the type (i.e. first the nested types within
    /// this  message are searched, then within the parent, on up to the
    /// root  namespace).
    type_name: Option<String>,
    ///  For extensions, this is the name of the type being extended.  It is
    ///  resolved in the same manner as type_name.
    extendee: message::Key,
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
    reference: Option<ReferenceInner>,
    file: file::Key,
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        std::iter::empty()
    }
}

impl Inner {
    pub(super) fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
        self.reference.iter_mut()
    }
}
pub struct Extension<'ast>(resolve::Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(Extension, Key, Inner);
impl<'ast> Extension<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::References::references(self)
    }
}

impl<'ast> super::access::References<'ast> for Extension<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_option(self.0.reference, self.ast())
    }
}
