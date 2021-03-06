mod enums;
mod iter;
mod path;
pub use enums::*;
pub use iter::*;
pub use path::*;

use std::{
    fmt::{self},
    ops::Deref,
    slice, vec,
};

lazy_static::lazy_static! {
    static ref DEFAULT_SOURCE_CODE_INFO: protobuf::descriptor::SourceCodeInfo =
        protobuf::descriptor::SourceCodeInfo::default();
    static ref DEFAULT_LOCATION: protobuf::descriptor::source_code_info::Location =
        protobuf::descriptor::source_code_info::Location::default();
    static ref DEFAULT_FILE_OPTIONS: protobuf::descriptor::FileOptions = protobuf::descriptor::FileOptions::default();
    static ref DEFAULT_MESSAGE_OPTIONS: protobuf::descriptor::MessageOptions =
        protobuf::descriptor::MessageOptions::default();
    static ref DEFAULT_ONEOF_OPTIONS: protobuf::descriptor::OneofOptions =
        protobuf::descriptor::OneofOptions::default();
    static ref DEFAULT_FIELD_OPTIONS: protobuf::descriptor::FieldOptions =
        protobuf::descriptor::FieldOptions::default();
    static ref DEFAULT_SERVICE_OPTIONS: protobuf::descriptor::ServiceOptions =
        protobuf::descriptor::ServiceOptions::default();
    static ref DEFAULT_METHOD_OPTIONS: protobuf::descriptor::MethodOptions =
        protobuf::descriptor::MethodOptions::default();
    static ref DEFAULT_ENUM_OPTIONS: protobuf::descriptor::EnumOptions = protobuf::descriptor::EnumOptions::default();
    static ref DEFAULT_ENUM_VALUE_OPTIONS: protobuf::descriptor::EnumValueOptions =
        protobuf::descriptor::EnumValueOptions::default();
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FileDescriptor<'a> {
    desc: &'a protobuf::descriptor::FileDescriptorProto,
}

impl<'a> FileDescriptor<'a> {
    /// file name, relative to root of source tree
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }

    /// e.g. "foo", "foo.bar", etc.
    pub fn package(&self) -> &'a str {
        self.desc.package()
    }
    /// Names of files imported by this file.
    pub fn dependencies(&self) -> slice::Iter<String> {
        self.desc.dependency.iter()
    }
    /// Indexes of the public imported files in the dependency list.
    pub fn public_dependencies(&self) -> slice::Iter<i32> {
        self.desc.public_dependency.iter()
    }
    /// All top-level `Message` definitions in this file.
    pub fn messages(&self) -> MessageDescriptorIter<'a> {
        (&self.desc.message_type).into()
    }

    // All top-level `Enum` definitions in this file.
    pub fn enums(&self) -> EnumDescriptorIter<'a> {
        (&self.desc.enum_type).into()
    }

    /// All top-level `Service` definitions in this file.
    pub fn services(&self) -> ServiceDescriptorIter<'a> {
        (&self.desc.service).into()
    }
    pub fn extensions(&self) -> FieldDescriptorIter<'a> {
        (&self.desc.extension).into()
    }

    pub fn options(&self) -> FileOptions<'a> {
        self.desc.options.as_ref().into()
    }
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    pub fn source_code_info(&self) -> SourceCodeInfo<'a> {
        self.desc.source_code_info.as_ref().into()
    }
    /// The syntax of the proto file.
    /// The supported values are "proto2" and "proto3".
    pub fn syntax(&self) -> Syntax {
        self.desc.syntax().into()
    }
}
impl<'a> From<&'a protobuf::descriptor::FileDescriptorProto> for FileDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::FileDescriptorProto) -> Self {
        Self { desc }
    }
}
#[cfg(test)]
impl<'a> Default for FileDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_FILE_DESCRIPTOR_PROTO,
        }
    }
}

/// Describes a message type.
#[derive(Debug, Clone, Copy)]
pub struct MessageDescriptor<'a> {
    desc: &'a protobuf::descriptor::DescriptorProto,
}
impl<'a> MessageDescriptor<'a> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn fields(&self) -> FieldDescriptorIter<'a> {
        let fields = &self.desc.field;
        fields.into()
    }
    pub fn is_map_entry(&self) -> bool {
        self.options().is_map_entry()
    }
    pub fn extensions(&self) -> FieldDescriptorIter<'a> {
        (&self.desc.extension).into()
    }
    pub fn nested_messages(&self) -> MessageDescriptorIter<'a> {
        (&self.desc.nested_type).into()
    }

    pub fn enums(&self) -> EnumDescriptorIter<'a> {
        (&self.desc.enum_type).into()
    }

    pub fn extension_ranges(&self) -> ExtensionRanges<'a> {
        (&self.desc.extension_range).into()
    }

    pub fn oneofs(&self) -> OneofDescriptorIter<'a> {
        (&self.desc.oneof_decl).into()
    }
    pub fn options(&self) -> MessageOptions<'a> {
        self.desc.options.as_ref().into()
    }
    pub fn reserved_ranges(&self) -> ReservedRanges<'a> {
        (&self.desc.reserved_range).into()
    }
    /// Reserved field names, which may not be used by fields in the same message.
    /// A given name may only be reserved once.
    pub fn reserved_names(&self) -> slice::Iter<String> {
        self.desc.reserved_name.iter()
    }
}
impl<'a> From<&'a protobuf::descriptor::DescriptorProto> for MessageDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::DescriptorProto) -> Self {
        Self { desc }
    }
}
#[cfg(test)]
impl<'a> Default for MessageDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_DESCRIPTOR_PROTO,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EnumDescriptor<'a> {
    desc: &'a protobuf::descriptor::EnumDescriptorProto,
}
impl<'a> EnumDescriptor<'a> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn values(&self) -> EnumValueDescriptorIter<'a> {
        (&self.desc.value).into()
    }
    pub fn options(&self) -> EnumOptions<'a> {
        self.desc.options.as_ref().into()
    }
    /// Range of reserved numeric values. Reserved numeric values may not be used
    /// by enum values in the same enum declaration. Reserved ranges may not
    /// overlap.
    pub fn reserved_ranges(&self) -> EnumReservedRanges<'a> {
        (&self.desc.reserved_range).into()
    }
    pub fn reserved_names(&self) -> slice::Iter<String> {
        self.desc.reserved_name.iter()
    }
}
impl<'a> From<&'a protobuf::descriptor::EnumDescriptorProto> for EnumDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::EnumDescriptorProto) -> Self {
        EnumDescriptor { desc }
    }
}
#[cfg(test)]
impl<'a> Default for EnumDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_ENUM_DESCRIPTOR_PROTO,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumValueDescriptor<'a> {
    desc: &'a protobuf::descriptor::EnumValueDescriptorProto,
}
impl<'a> EnumValueDescriptor<'a> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn number(&self) -> i32 {
        self.desc.number()
    }
    pub fn options(&self) -> EnumValueOptions<'a> {
        self.desc.options.as_ref().into()
    }
}
impl<'a> From<&'a protobuf::descriptor::EnumValueDescriptorProto> for EnumValueDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::EnumValueDescriptorProto) -> Self {
        Self { desc }
    }
}
#[cfg(test)]
impl<'a> Default for EnumValueDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_ENUM_VALUE_DESCRIPTOR_PROTO,
        }
    }
}

/// Describes a field within a message.
#[derive(Debug, Clone, Copy)]
pub struct FieldDescriptor<'a> {
    desc: &'a protobuf::descriptor::FieldDescriptorProto,
}
impl<'a> FieldDescriptor<'a> {
    pub fn name(&self) -> &str {
        self.desc.name()
    }
    pub fn number(&self) -> i32 {
        self.desc.number()
    }
    pub fn label(&self) -> Label {
        Label::from(self.desc.label())
    }
    pub fn is_lazy(&self) -> bool {
        self.options().is_lazy()
    }
    pub fn is_deprecated(&self) -> bool {
        self.options().is_deprecated()
    }
    /// alias for `r#type`
    ///
    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    pub fn proto_type(&self) -> Type<'a> {
        self.r#type()
    }

    /// If type_name is set, this need not be set.  If both this and type_name
    /// are set, this must be one of Enum, Message or Group.
    pub fn r#type(&self) -> Type<'a> {
        Type::from(self.desc)
    }

    pub fn is_embed(&self) -> bool {
        matches!(self.r#type(), Type::Message(_))
    }
    pub fn is_message(&self) -> bool {
        matches!(self.r#type(), Type::Message(_))
    }

    pub fn is_enum(&self) -> bool {
        matches!(self.r#type(), Type::Enum(_))
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Double))
    }
    pub fn is_float(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Float))
    }

    pub fn is_int64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int64))
    }

    pub fn is_uint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint64))
    }

    pub fn is_int32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Int32))
    }

    pub fn is_fixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Fixed64))
    }

    pub fn is_fixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Fixed32))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Bool))
    }

    pub fn is_string(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::String))
    }

    pub fn is_bytes(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Bytes))
    }

    pub fn is_uint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Uint32))
    }

    pub fn is_sfixed32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed32))
    }

    pub fn is_sfixed64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sfixed64))
    }

    pub fn is_sint32(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint32))
    }

    pub fn is_sint64(&self) -> bool {
        matches!(self.r#type(), Type::Scalar(Scalar::Sint64))
    }

    pub fn is_repeated(&self) -> bool {
        self.label() == Label::Repeated
    }

    /// For message and enum types, this is the name of the type.  If the name
    /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
    /// rules are used to find the type (i.e. first the nested types within this
    /// message are searched, then within the parent, on up to the root
    /// namespace).
    pub fn type_name(&self) -> &str {
        self.desc.type_name()
    }

    /// For extensions, this is the name of the type being extended.  It is
    /// resolved in the same manner as `proto_type_name`.
    pub fn extendee(&self) -> &str {
        self.desc.extendee()
    }
    /// For numeric types, contains the original text representation of the value.
    /// For booleans, "true" or "false".
    /// For strings, contains the default text contents (not escaped in any way).
    /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
    pub fn default_value(&self) -> &str {
        self.desc.default_value()
    }
    /// If set, gives the index of a oneof in the containing type's oneof_decl
    /// list.
    ///
    /// This field is a member of that oneof.
    pub fn oneof_index(&self) -> Option<i32> {
        if self.desc.has_oneof_index() {
            Some(self.desc.oneof_index())
        } else {
            None
        }
    }

    /// JSON name of this field. The value is set by protocol compiler. If the
    /// user has set a "json_name" option on this field, that option's value
    /// will be used. Otherwise, it's deduced from the field's name by converting
    /// it to camelCase.
    pub fn json_name(&self) -> &str {
        self.desc.json_name()
    }
    pub fn options(&self) -> FieldOptions<'a> {
        self.desc.options.as_ref().into()
    }
    /// If true, this is a proto3 "optional". When a proto3 field is optional, it
    /// tracks presence regardless of field type.
    ///
    /// When proto3_optional is true, this field must be belong to a oneof to
    /// signal to old proto3 clients that presence is tracked for this field. This
    /// oneof is known as a "synthetic" oneof, and this field must be its sole
    /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
    /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
    /// oneofs must be ordered after all "real" oneofs.
    ///
    /// For message fields, proto3_optional doesn't create any semantic change,
    /// since non-repeated message fields always track presence. However it still
    /// indicates the semantic detail of whether the user wrote "optional" or not.
    /// This can be useful for round-tripping the .proto file. For consistency we
    /// give message fields a synthetic oneof also, even though it is not required
    /// to track presence. This is especially important because the parser can't
    /// tell if a field is a message or an enum, so it must always create a
    /// synthetic oneof.
    ///
    /// Proto2 optional fields do not set this flag, because they already indicate
    /// optional with `LABEL_OPTIONAL`.
    pub fn proto3_optional(&self) -> bool {
        self.desc.proto3_optional()
    }
    /// returns `true` if:
    ///
    /// - `syntax` is `Syntax::Proto3` and `proto3_optional` is `true`
    /// - `syntax` is `Syntax::Proto2` and `label` is `Label::Optional`.
    pub fn is_marked_optional(&self, syntax: Syntax) -> bool {
        match syntax {
            Syntax::Proto2 => self.label() == Label::Optional,
            Syntax::Proto3 => self.proto3_optional(),
        }
    }
    pub fn is_marked_required(&self, syntax: Syntax) -> bool {
        syntax.supports_required_prefix() && self.label() == Label::Required
    }
}
impl<'a> From<&'a protobuf::descriptor::FieldDescriptorProto> for FieldDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::FieldDescriptorProto) -> Self {
        Self { desc }
    }
}

#[cfg(test)]
impl<'a> Default for FieldDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_FIELD_DESCRIPTOR_PROTO,
        }
    }
}

/// Describes a service.
#[derive(Debug, Clone, Copy)]
pub struct ServiceDescriptor<'a> {
    desc: &'a protobuf::descriptor::ServiceDescriptorProto,
}
impl<'a> ServiceDescriptor<'a> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn options(&self) -> ServiceOptions<'a> {
        self.desc.options.as_ref().into()
    }
    pub fn methods(&self) -> MethodDescriptorIter<'a> {
        (&self.desc.method).into()
    }
}
impl<'a> From<&'a protobuf::descriptor::ServiceDescriptorProto> for ServiceDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::ServiceDescriptorProto) -> Self {
        Self { desc }
    }
}
#[cfg(test)]
impl<'a> Default for ServiceDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_SERVICE_DESCRIPTOR_PROTO,
        }
    }
}

/// Describes a method.
#[derive(Debug, Clone, Copy)]
pub struct MethodDescriptor<'a> {
    desc: &'a protobuf::descriptor::MethodDescriptorProto,
}
impl<'a> MethodDescriptor<'a> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    /// Input type name.
    ///
    /// These are resolved in the same way as
    /// `FieldDescriptor.type_name`, but must refer to a message type
    pub fn input_type(&self) -> &'a str {
        self.desc.input_type()
    }
    /// Output type name.
    ///
    /// These are resolved in the same way as
    /// `FieldDescriptor.type_name`, but must refer to a message type
    pub fn output_type(&self) -> &'a str {
        self.desc.output_type()
    }
    /// Identifies if client streams multiple client messages
    pub fn client_streaming(&self) -> bool {
        self.desc.client_streaming()
    }
    /// Identifies if server streams multiple server messages
    pub fn server_streaming(&self) -> bool {
        self.desc.server_streaming()
    }
    pub fn options(&self) -> MethodOptions<'a> {
        self.desc.options.as_ref().into()
    }
}
impl<'a> From<&'a protobuf::descriptor::MethodDescriptorProto> for MethodDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::MethodDescriptorProto) -> Self {
        Self { desc }
    }
}
#[cfg(test)]
impl<'a> Default for MethodDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_METHOD_DESCRIPTOR_PROTO,
        }
    }
}

/// Describes a oneof.
#[derive(Debug, Clone, Copy)]
pub struct OneofDescriptor<'a> {
    desc: &'a protobuf::descriptor::OneofDescriptorProto,
}
impl<'a> OneofDescriptor<'a> {
    pub fn name(&self) -> &'a str {
        self.desc.name()
    }
    pub fn options(&self) -> OneofOptions<'a> {
        self.desc.options.as_ref().into()
    }
}
impl<'a> From<&'a protobuf::descriptor::OneofDescriptorProto> for OneofDescriptor<'a> {
    fn from(desc: &'a protobuf::descriptor::OneofDescriptorProto) -> Self {
        Self { desc }
    }
}
#[cfg(test)]
impl<'a> Default for OneofDescriptor<'a> {
    fn default() -> Self {
        Self {
            desc: &test_data::DEFAULT_ONEOF_DESCRIPTOR,
        }
    }
}
// ===================================================================
// Options

// Each of the definitions above may have "options" attached.  These are
// just annotations which may cause code to be generated slightly differently
// or may contain hints for code that manipulates protocol messages.
//
// Clients may define custom options as extensions of the *Options messages.
// These extensions may not yet be known at parsing time, so the parser cannot
// store the values in them.  Instead it stores them in a field in the *Options
// message called uninterpreted_option. This field must have the same name
// across all *Options messages. We then use this field to populate the
// extensions when we build a descriptor, at which point all protos have been
// parsed and so all extensions are known.
//
// Extension numbers for custom options may be chosen as follows:
// * For options which will only be used within a single application or
//   organization, or for experimental options, use field numbers 50000
//   through 99999.  It is up to you to ensure that you do not use the
//   same number for multiple options.
// * For options which will be published and used publicly by multiple
//   independent entities, e-mail protobuf-global-extension-registry@google.com
//   to reserve extension numbers. Simply provide your project name (e.g.
//   Objective-C plugin) and your project website (if available) -- there's no
//   need to explain how you intend to use them. Usually you only need one
//   extension number. You can declare multiple options with only one extension
//   number by putting them in a sub-message. See the Custom Options section of
//   the docs for examples:
//   <https://developers.google.com/protocol-buffers/docs/proto#options>
//   If this turns out to be popular, a web service will be set up
//   to automatically assign option numbers.
#[derive(Debug, Clone, Copy)]
pub struct FileOptions<'a> {
    opts: Option<&'a protobuf::descriptor::FileOptions>,
}
impl<'a> From<Option<&'a protobuf::descriptor::FileOptions>> for FileOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::FileOptions>) -> Self {
        Self { opts }
    }
}
impl<'a> FileOptions<'a> {
    /// Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    pub fn java_package(&self) -> &str {
        self.opts().java_package()
    }
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    pub fn java_outer_classname(&self) -> &str {
        self.opts().java_outer_classname()
    }

    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    pub fn java_multiple_files(&self) -> bool {
        self.opts().java_multiple_files()
    }

    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    /// This option has no effect on when used with the lite runtime.
    pub fn java_string_check_utf8(&self) -> bool {
        self.opts().java_string_check_utf8()
    }
    /// Generated classes can be optimized for speed or code size.
    pub fn optimize_for(&self) -> OptimizeMode {
        self.opts().optimize_for().into()
    }
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    pub fn go_package(&self) -> &str {
        self.opts().go_package()
    }
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    pub fn cc_generic_services(&self) -> bool {
        self.opts().cc_generic_services()
    }

    pub fn java_generic_services(&self) -> bool {
        self.opts().java_generic_services()
    }

    pub fn py_generic_services(&self) -> bool {
        self.opts().py_generic_services()
    }

    pub fn php_generic_services(&self) -> bool {
        self.opts().php_generic_services()
    }

    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    pub fn cc_enable_arenas(&self) -> bool {
        self.opts().cc_enable_arenas()
    }
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    pub fn objc_class_prefix(&self) -> &str {
        self.opts().objc_class_prefix()
    }

    /// Namespace for generated classes; defaults to the package.
    pub fn csharp_namespace(&self) -> &str {
        self.opts().csharp_namespace()
    }
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    pub fn swift_prefix(&self) -> &str {
        self.opts().swift_prefix()
    }

    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    pub fn php_class_prefix(&self) -> &str {
        self.opts().php_class_prefix()
    }

    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    pub fn php_namespace(&self) -> &str {
        self.opts().php_namespace()
    }

    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    pub fn php_metadata_namespace(&self) -> &str {
        self.opts().php_metadata_namespace()
    }
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    pub fn ruby_package(&self) -> &str {
        self.opts().ruby_package()
    }
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a protobuf::descriptor::FileOptions {
        self.opts.unwrap_or(&DEFAULT_FILE_OPTIONS)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumValueOptions<'a> {
    opts: Option<&'a protobuf::descriptor::EnumValueOptions>,
}
impl<'a> EnumValueOptions<'a> {
    /// Is this enum value deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum value, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating enum values.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Options not recognized by the parser.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a protobuf::descriptor::EnumValueOptions {
        self.opts.unwrap_or(&DEFAULT_ENUM_VALUE_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::EnumValueOptions>> for EnumValueOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::EnumValueOptions>) -> Self {
        Self { opts }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MessageOptions<'a> {
    opts: Option<&'a protobuf::descriptor::MessageOptions>,
}
impl<'a> MessageOptions<'a> {
    /// Set true to use the old proto1 MessageSet wire format for extensions.
    /// This is provided for backwards-compatibility with the MessageSet wire
    /// format.  You should not use this for any other reason:  It's less
    /// efficient, has fewer features, and is more complicated.
    ///
    /// The message must be defined exactly as follows:
    ///   message Foo {
    ///     option message_set_wire_format = true;
    ///     extensions 4 to max;
    ///   }
    /// Note that the message cannot have any defined fields; MessageSets only
    /// have extensions.
    ///
    /// All extensions of your type must be singular messages; e.g. they cannot
    /// be int32s, enums, or repeated messages.
    ///
    /// Because this is an option, the above two restrictions are not enforced by
    /// the protocol compiler.
    pub fn message_set_wire_format(&self) -> bool {
        self.opts().message_set_wire_format()
    }
    /// Whether the message is an automatically generated map entry type for the
    /// maps field.
    ///
    /// For maps fields:
    ///     map<KeyType, ValueType> map_field = 1;
    /// The parsed descriptor looks like:
    ///     message MapFieldEntry {
    ///         option map_entry = true;
    ///         optional KeyType key = 1;
    ///         optional ValueType value = 2;
    ///     }
    ///     repeated MapFieldEntry map_field = 1;
    ///
    /// Implementations may choose not to generate the map_entry=true message, but
    /// use a native map in the target language to hold the keys and values.
    /// The reflection APIs in such implementations still need to work as
    /// if the field is a repeated message field.
    ///
    /// NOTE: Do not set the option in .proto files. Always use the maps syntax
    /// instead. The option should only be implicitly set by the proto compiler
    /// parser.
    pub fn map_entry(&self) -> bool {
        self.opts().map_entry()
    }

    pub fn is_map_entry(&self) -> bool {
        self.map_entry()
    }

    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn no_standard_descriptor_accessor(&self) -> bool {
        self.opts().no_standard_descriptor_accessor()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_option(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a protobuf::descriptor::MessageOptions {
        self.opts.unwrap_or(&DEFAULT_MESSAGE_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::MessageOptions>> for MessageOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::MessageOptions>) -> Self {
        MessageOptions { opts }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FieldOptions<'a> {
    opts: Option<&'a protobuf::descriptor::FieldOptions>,
}
impl<'a> FieldOptions<'a> {
    pub fn new(opts: Option<&'a protobuf::descriptor::FieldOptions>) -> Self {
        Self { opts }
    }
    /// The ctype option instructs the C++ code generator to use a different
    /// representation of the field than it normally would.  See the specific
    /// options below.  This option is not yet implemented in the open source
    /// release -- sorry, we'll try to include it in a future version!
    pub fn ctype(&self) -> CType {
        CType::from(self.opts().ctype())
    }
    /// The packed option can be enabled for repeated primitive fields to enable
    /// a more efficient representation on the wire. Rather than repeatedly
    /// writing the tag and type for each element, the entire array is encoded as
    /// a single length-delimited blob. In proto3, only explicit setting it to
    /// false will avoid using packed encoding.
    pub fn packed(&self) -> bool {
        self.opts().packed()
    }
    /// The jstype option determines the JavaScript type used for values of the
    /// field.  The option is permitted only for 64 bit integral and fixed types
    /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
    /// is represented as JavaScript string, which avoids loss of precision that
    /// can happen when a large value is converted to a floating point JavaScript.
    /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
    /// use the JavaScript "number" type.  The behavior of the default option
    /// JS_NORMAL is implementation dependent.
    ///
    /// This option is an enum to permit additional types to be added, e.g.
    /// goog.math.Integer.
    pub fn jstype(&self) -> JsType {
        self.opts().jstype().into()
    }
    /// Should this field be parsed lazily?  Lazy applies only to message-type
    /// fields.  It means that when the outer message is initially parsed, the
    /// inner message's contents will not be parsed but instead stored in encoded
    /// form.  The inner message will actually be parsed when it is first accessed.
    ///
    /// This is only a hint.  Implementations are free to choose whether to use
    /// eager or lazy parsing regardless of the value of this option.  However,
    /// setting this option true suggests that the protocol author believes that
    /// using lazy parsing on this field is worth the additional bookkeeping
    /// overhead typically needed to implement it.
    ///
    /// This option does not affect the public interface of any generated code;
    /// all method signatures remain the same.  Furthermore, thread-safety of the
    /// interface is not affected by this option; const methods remain safe to
    /// call from multiple threads concurrently, while non-const methods continue
    /// to require exclusive access.
    ///
    ///
    /// Note that implementations may choose not to check required fields within
    /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
    /// may return true even if the inner message has missing required fields.
    /// This is necessary because otherwise the inner message would have to be
    /// parsed in order to perform the check, defeating the purpose of lazy
    /// parsing.  An implementation which chooses not to check required fields
    /// must be consistent about it.  That is, for any particular sub-message, the
    /// implementation must either *always* check its required fields, or *never*
    /// check its required fields, regardless of whether or not the message has
    /// been parsed.
    pub fn is_lazy(&self) -> bool {
        self.opts().lazy()
    }
    /// Is this field deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for accessors, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating fields.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// For Google-internal migration only. Do not use.
    pub fn is_weak(&self) -> bool {
        self.opts().weak()
    }

    /// Options the parser does not recognize.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }

    fn opts(&self) -> &'a protobuf::descriptor::FieldOptions {
        self.opts.unwrap_or(&DEFAULT_FIELD_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::FieldOptions>> for FieldOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::FieldOptions>) -> Self {
        Self { opts }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumOptions<'a> {
    opts: Option<&'a protobuf::descriptor::EnumOptions>,
}
impl<'a> EnumOptions<'a> {
    /// Is this enum deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the enum, or it will be completely ignored; in the very least, this
    /// is a formalization for deprecating enums.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Options not recognized by the parser.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }
    /// Allows mapping different tag names to the same value.
    pub fn allow_alias(&self) -> bool {
        self.opts().allow_alias()
    }
    fn opts(&self) -> &'a protobuf::descriptor::EnumOptions {
        self.opts.unwrap_or(&DEFAULT_ENUM_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::EnumOptions>> for EnumOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::EnumOptions>) -> Self {
        Self { opts }
    }
}

/// Options for a Service.
///
/// Note: Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
#[derive(Debug, Clone, Copy)]
pub struct ServiceOptions<'a> {
    opts: Option<&'a protobuf::descriptor::ServiceOptions>,
}
impl<'a> ServiceOptions<'a> {
    /// Is this service deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the service, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating services.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }
    fn opts(&self) -> &'a protobuf::descriptor::ServiceOptions {
        self.opts.unwrap_or(&DEFAULT_SERVICE_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::ServiceOptions>> for ServiceOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::ServiceOptions>) -> Self {
        Self { opts }
    }
}

/// Options for a Method.
///
/// Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
/// framework.
pub struct MethodOptions<'a> {
    opts: Option<&'a protobuf::descriptor::MethodOptions>,
}
impl<'a> MethodOptions<'a> {
    // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
    //   framework.  We apologize for hoarding these numbers to ourselves, but
    //   we were already using them long before we decided to release Protocol
    //   Buffers.

    /// Is this method deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for the method, or it will be completely ignored; in the very least,
    /// this is a formalization for deprecating methods.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }

    /// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
    /// or neither? HTTP based RPC implementation may choose GET verb for safe
    /// methods, and PUT verb for idempotent methods instead of the default POST.
    pub fn idempotency_level(&self) -> IdempotencyLevel {
        self.opts().idempotency_level().into()
    }
    fn opts(&self) -> &'a protobuf::descriptor::MethodOptions {
        self.opts.unwrap_or(&DEFAULT_METHOD_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::MethodOptions>> for MethodOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::MethodOptions>) -> Self {
        Self { opts }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OneofOptions<'a> {
    opts: Option<&'a protobuf::descriptor::OneofOptions>,
}
impl<'a> OneofOptions<'a> {
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> UninterpretedOptions<'a> {
        (&self.opts().uninterpreted_option).into()
    }
    pub fn opts(&self) -> &'a protobuf::descriptor::OneofOptions {
        self.opts.unwrap_or(&DEFAULT_ONEOF_OPTIONS)
    }
}
impl<'a> From<Option<&'a protobuf::descriptor::OneofOptions>> for OneofOptions<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::OneofOptions>) -> Self {
        Self { opts }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UninterpretedOptions<'a> {
    pub(crate) opts: &'a [protobuf::descriptor::UninterpretedOption],
}
impl<'a> UninterpretedOptions<'a> {
    pub fn iter(&self) -> UninterpretedOptionsIter<'a> {
        self.into()
    }
}
impl<'a> IntoIterator for UninterpretedOptions<'a> {
    type Item = UninterpretedOption<'a>;
    type IntoIter = UninterpretedOptionsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::UninterpretedOption>> for UninterpretedOptions<'a> {
    fn from(opts: &'a Vec<protobuf::descriptor::UninterpretedOption>) -> Self {
        UninterpretedOptions { opts }
    }
}

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(Debug, Clone, Copy)]
pub struct UninterpretedOption<'a> {
    opt: &'a protobuf::descriptor::UninterpretedOption,
}
impl<'a> UninterpretedOption<'a> {
    pub fn name_parts(&self) -> NameParts<'a> {
        NameParts::from(&self.opt.name)
    }

    pub fn identifier_value(&self) -> &'a str {
        self.opt.identifier_value()
    }
    pub fn positive_int_value(&self) -> u64 {
        self.opt.positive_int_value()
    }
    pub fn negative_int_value(&self) -> i64 {
        self.opt.negative_int_value()
    }
    pub fn double_value(&self) -> f64 {
        self.opt.double_value()
    }
    pub fn string_value(&self) -> &'a [u8] {
        self.opt.string_value()
    }
    pub fn aggregate_value(&self) -> &'a str {
        self.opt.aggregate_value()
    }
}

impl<'a> From<&'a protobuf::descriptor::UninterpretedOption> for UninterpretedOption<'a> {
    fn from(opt: &'a protobuf::descriptor::UninterpretedOption) -> Self {
        UninterpretedOption { opt }
    }
}

/// Range of reserved tag numbers. Reserved tag numbers may not be used by
/// fields or extension ranges in the same message. Reserved ranges may
/// not overlap.
#[derive(Debug, Clone, Copy)]
pub struct ReservedRange<'a> {
    range: &'a protobuf::descriptor::descriptor_proto::ReservedRange,
}
impl<'a> ReservedRange<'a> {
    /// Inclusive.
    pub fn start(&self) -> i32 {
        self.range.start()
    }

    /// Exclusive.
    pub fn end(&self) -> i32 {
        self.range.end()
    }

    pub fn in_range(&self, val: i32) -> bool {
        self.start() <= val && val < self.end()
    }
}
impl<'a> PartialEq for ReservedRange<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.start() == other.start() && self.end() == other.end()
    }
}
impl<'a> From<&'a protobuf::descriptor::descriptor_proto::ReservedRange> for ReservedRange<'a> {
    fn from(range: &'a protobuf::descriptor::descriptor_proto::ReservedRange) -> Self {
        ReservedRange { range }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReservedRanges<'a> {
    ranges: &'a [protobuf::descriptor::descriptor_proto::ReservedRange],
}
impl<'a> IntoIterator for ReservedRanges<'a> {
    type Item = ReservedRange<'a>;
    type IntoIter = ReservedRangeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into()
    }
}
impl<'a> ReservedRanges<'a> {
    pub fn iter(&self) -> ReservedRangeIter<'a> {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<ReservedRange<'a>> {
        self.ranges.get(index).map(Into::into)
    }
    pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
        self.iter().any(|r| r.start() <= min && r.end() >= max)
    }
    pub fn is_in_reserved_range(&self, num: i32) -> bool {
        self.iter().any(|r| r.start() <= num && r.end() >= num)
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::descriptor_proto::ReservedRange>>
    for ReservedRanges<'a>
{
    fn from(ranges: &'a Vec<protobuf::descriptor::descriptor_proto::ReservedRange>) -> Self {
        ReservedRanges { ranges }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExtensionRange<'a> {
    range: &'a protobuf::descriptor::descriptor_proto::ExtensionRange,
}
impl<'a> PartialEq for ExtensionRange<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.range.start() == other.start() && self.end() == other.end()
    }
}
impl<'a> ExtensionRange<'a> {
    /// Inclusive.
    pub fn start(&self) -> i32 {
        self.range.start()
    }
    /// Exclusive.
    pub fn end(&self) -> i32 {
        self.range.end()
    }
    pub fn in_range(&self, val: i32) -> bool {
        self.start() <= val && val < self.end()
    }
}
impl<'a> From<&'a protobuf::descriptor::descriptor_proto::ExtensionRange> for ExtensionRange<'a> {
    fn from(range: &'a protobuf::descriptor::descriptor_proto::ExtensionRange) -> Self {
        ExtensionRange { range }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExtensionRanges<'a> {
    ranges: &'a [protobuf::descriptor::descriptor_proto::ExtensionRange],
}
impl<'a> ExtensionRanges<'a> {
    pub fn iter(&self) -> ExtensionRangeIter<'a> {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn first(&self) -> Option<ExtensionRange<'a>> {
        self.ranges.first().map(|r| r.into())
    }
    pub fn last(&self) -> Option<ExtensionRange<'a>> {
        self.ranges.last().map(|r| r.into())
    }
    pub fn get(&self, n: usize) -> Option<ExtensionRange<'a>> {
        self.ranges.get(n).map(|r| r.into())
    }
}
impl<'a> IntoIterator for ExtensionRanges<'a> {
    type Item = ExtensionRange<'a>;
    type IntoIter = ExtensionRangeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::descriptor_proto::ExtensionRange>>
    for ExtensionRanges<'a>
{
    fn from(ranges: &'a Vec<protobuf::descriptor::descriptor_proto::ExtensionRange>) -> Self {
        ExtensionRanges { ranges }
    }
}

// Range of reserved numeric values. Reserved values may not be used by
/// entries in the same enum. Reserved ranges may not overlap.
///
/// Note that this is distinct from DescriptorProto.ReservedRange in that it
/// is inclusive such that it can appropriately represent the entire int32
/// domain.
#[derive(Debug, PartialEq)]
pub struct EnumReservedRange<'a> {
    rr: &'a protobuf::descriptor::enum_descriptor_proto::EnumReservedRange,
}
impl<'a> From<&'a protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>
    for EnumReservedRange<'a>
{
    fn from(r: &'a protobuf::descriptor::enum_descriptor_proto::EnumReservedRange) -> Self {
        Self { rr: r }
    }
}
impl<'a> EnumReservedRange<'a> {
    /// Inclusive
    pub fn start(&self) -> i32 {
        self.rr.start()
    }
    /// Inclusive
    pub fn end(&self) -> i32 {
        self.rr.end()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumReservedRanges<'a> {
    ranges: &'a [protobuf::descriptor::enum_descriptor_proto::EnumReservedRange],
}
impl<'a> IntoIterator for EnumReservedRanges<'a> {
    type Item = EnumReservedRange<'a>;
    type IntoIter = EnumReservedRangeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into()
    }
}
impl<'a> EnumReservedRanges<'a> {
    pub fn iter(&self) -> EnumReservedRangeIter<'a> {
        self.ranges.into()
    }
    pub fn len(&self) -> usize {
        self.ranges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<EnumReservedRange<'a>> {
        self.ranges.get(index).map(|r| r.into())
    }
    pub fn is_range_reserved(&self, min: i32, max: i32) -> bool {
        self.iter().any(|r| r.start() <= min && r.end() >= max)
    }
    pub fn is_in_reserved_range(&self, num: i32) -> bool {
        self.iter().any(|r| r.start() <= num && r.end() >= num)
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>>
    for EnumReservedRanges<'a>
{
    fn from(
        ranges: &'a Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>,
    ) -> Self {
        Self { ranges }
    }
}

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name. `is_extension` is `true` if a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
///
/// E.g.,
/// ```no_run
/// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
/// ```
#[derive(Clone, Copy)]
pub struct NamePart<'a> {
    part: &'a protobuf::descriptor::uninterpreted_option::NamePart,
}
impl<'a> Eq for NamePart<'a> {}
impl<'a> PartialEq<NamePart<'a>> for NamePart<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.part == other.part
    }
}

impl<'a> fmt::Debug for NamePart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NamePart")
            .field("value", &self.part.name_part())
            .field("is_extension", &self.part.is_extension())
            .finish()
    }
}

impl<'a> PartialEq<String> for NamePart<'a> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}
impl<'a> fmt::Display for NamePart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl<'a> PartialEq<&str> for NamePart<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> NamePart<'a> {
    /// alias for value
    /// the value of the part `NamePart`
    pub fn name_part(&self) -> &'a str {
        &self.part.name_part()
    }
    /// the value of the part
    /// E.g. `"foo"`, `"bar.baz"`, or `"qux"` of:
    /// ```no_run
    /// "foo.(bar.baz).qux" => [ ("foo", false), ("bar.baz", true), ("qux", false) ]
    /// ```
    pub fn value(&self) -> &'a str {
        self.name_part()
    }
    /// is_extension is true if the segment represents an extension (denoted
    /// with parentheses in options specs in .proto files).
    pub fn is_extension(&self) -> bool {
        self.part.is_extension()
    }
    pub fn formatted_value(&self) -> String {
        if self.part.is_extension() {
            format!("({})", self.part.name_part())
        } else {
            self.part.name_part().to_string()
        }
    }
}

impl<'a> NamePart<'a> {
    pub fn as_str(&self) -> &str {
        &self.part.name_part()
    }
}

impl<'a> Deref for NamePart<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.part.name_part()
    }
}

impl<'a> From<&'a protobuf::descriptor::uninterpreted_option::NamePart> for NamePart<'a> {
    fn from(part: &'a protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
        Self { part }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NameParts<'a> {
    parts: &'a [protobuf::descriptor::uninterpreted_option::NamePart],
}

impl<'a> ToString for NameParts<'a> {
    fn to_string(&self) -> String {
        self.formatted_value()
    }
}

impl<'a> From<&'a std::vec::Vec<protobuf::descriptor::uninterpreted_option::NamePart>>
    for NameParts<'a>
{
    fn from(
        prost_parts: &'a std::vec::Vec<protobuf::descriptor::uninterpreted_option::NamePart>,
    ) -> Self {
        Self { parts: prost_parts }
    }
}
impl<'a> std::iter::IntoIterator for &NameParts<'a> {
    type Item = NamePart<'a>;
    type IntoIter = vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.parts
            .iter()
            .map(NamePart::from)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<'a> NameParts<'a> {
    pub fn iter(&self) -> NamePartIter<'a> {
        self.into()
    }
    pub fn get(&self, idx: usize) -> Option<NamePart<'a>> {
        self.parts.get(idx).map(NamePart::from)
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
    pub fn contains(&self, part: &str) -> bool {
        self.parts.iter().any(|p| p.name_part() == part)
    }
    pub fn formatted_value(&self) -> String {
        self.iter()
            .map(|part| part.formatted_value())
            .collect::<Vec<_>>()
            .join(".")
    }
}

pub struct NamePartIter<'a> {
    iter: std::slice::Iter<'a, protobuf::descriptor::uninterpreted_option::NamePart>,
}
impl<'a> Iterator for NamePartIter<'a> {
    type Item = NamePart<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(NamePart::from)
    }
}

impl<'a> From<&NameParts<'a>> for NamePartIter<'a> {
    fn from(parts: &NameParts<'a>) -> Self {
        Self {
            iter: parts.parts.iter(),
        }
    }
}
impl<'a> From<&'a [protobuf::descriptor::uninterpreted_option::NamePart]> for NamePartIter<'a> {
    fn from(parts: &'a [protobuf::descriptor::uninterpreted_option::NamePart]) -> Self {
        Self { iter: parts.iter() }
    }
}
impl<'a> From<&'a Vec<protobuf::descriptor::uninterpreted_option::NamePart>> for NamePartIter<'a> {
    fn from(parts: &'a Vec<protobuf::descriptor::uninterpreted_option::NamePart>) -> Self {
        Self { iter: parts.iter() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourceCodeInfo<'a> {
    pub(crate) info: &'a protobuf::descriptor::SourceCodeInfo,
}

impl<'a> SourceCodeInfo<'a> {
    pub fn iter(&self) -> LocationIter<'a> {
        self.into()
    }
    pub fn len(&self) -> usize {
        self.info.location.len()
    }
    pub fn is_empty(&self) -> bool {
        self.info.location.is_empty()
    }
}

impl<'a> From<&'a protobuf::descriptor::SourceCodeInfo> for SourceCodeInfo<'a> {
    fn from(info: &'a protobuf::descriptor::SourceCodeInfo) -> Self {
        SourceCodeInfo { info }
    }
}

impl<'a> From<Option<&'a protobuf::descriptor::SourceCodeInfo>> for SourceCodeInfo<'a> {
    fn from(info: Option<&'a protobuf::descriptor::SourceCodeInfo>) -> Self {
        SourceCodeInfo {
            info: info.unwrap_or(&DEFAULT_SOURCE_CODE_INFO),
        }
    }
}

impl<'a> IntoIterator for SourceCodeInfo<'a> {
    type Item = Location<'a>;
    type IntoIter = LocationIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LocationIter::from(&self)
    }
}

impl<'a> Default for Location<'a> {
    fn default() -> Self {
        Location {
            loc: &DEFAULT_LOCATION,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_string() {
        let mut p1 = protobuf::descriptor::uninterpreted_option::NamePart::new();
        p1.set_name_part("foo".to_string());
        let mut p2 = protobuf::descriptor::uninterpreted_option::NamePart::new();
        p2.set_name_part("bar".to_string());
        p2.set_is_extension(true);
        let mut p3 = protobuf::descriptor::uninterpreted_option::NamePart::new();
        p3.set_name_part("baz".to_string());
        let parts = vec![p1, p2, p3];
        let name_parts: NameParts<'_> = NameParts::from(&parts);
        assert_eq!(name_parts.to_string(), "foo.(bar).baz");
        assert_eq!(name_parts.get(0).unwrap(), "foo")
    }
}

/// Comments associated to entities in the source code.
#[derive(Debug, Clone, Copy)]
pub struct Location<'a> {
    loc: &'a protobuf::descriptor::source_code_info::Location,
}
impl<'a> From<&'a protobuf::descriptor::source_code_info::Location> for Location<'a> {
    fn from(loc: &'a protobuf::descriptor::source_code_info::Location) -> Self {
        Self { loc }
    }
}

impl<'a> Location<'a> {
    /// Identifies which part of the FileDescriptorProto was defined at this
    /// location.
    ///
    /// Each element is a field number or an index.  They form a path from
    /// the root FileDescriptorProto to the place where the definition.  For
    /// example, this path:
    ///   [ 4, 3, 2, 7, 1 ]
    /// refers to:
    ///   file.message_type(3)  // 4, 3
    ///       .field(7)         // 2, 7
    ///       .name()           // 1
    /// This is because FileDescriptorProto.message_type has field number 4:
    ///   repeated DescriptorProto message_type = 4;
    /// and DescriptorProto.field has field number 2:
    ///   repeated FieldDescriptorProto field = 2;
    /// and FieldDescriptorProto.name has field number 1:
    ///   optional string name = 1;
    ///
    /// Thus, the above path gives the location of a field name.  If we removed
    /// the last element:
    ///   [ 4, 3, 2, 7 ]
    /// this path refers to the whole field declaration (from the beginning
    /// of the label to the terminating semicolon).
    pub fn path(&self) -> &'a [i32] {
        &self.loc.path
    }
    /// Always has exactly three or four elements: start line, start column,
    /// end line (optional, otherwise assumed same as start line), end column.
    /// These are packed into a single field for efficiency.  Note that line
    /// and column numbers are zero-based -- typically you will want to add
    /// 1 to each before displaying to a user
    pub fn span(&self) -> &'a [i32] {
        &self.loc.span
    }

    /// Returns any comment immediately preceding the node, without anyElsewhere
    /// whitespace between it and the comment.
    pub fn leading_comments(&self) -> &'a str {
        self.loc.leading_comments()
    }

    /// Returns each comment block or line above the
    /// entity but separated by whitespace.a
    pub fn leading_detached_comments(&self) -> std::slice::Iter<'a, String> {
        self.loc.leading_detached_comments.iter()
    }
    /// Returns any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing_comments(&self) -> &'a str {
        self.loc.trailing_comments()
    }

    pub fn is_file_syntax_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Syntax == self.path()[0]
    }

    pub fn is_file_package_location(&self) -> bool {
        self.path().len() == 1 && FileDescriptorPath::Package == self.path()[0]
    }

    pub fn file_descriptor_path(&self) -> Result<FileDescriptorPath, anyhow::Error> {
        FileDescriptorPath::try_from(self.path().get(0))
    }

    pub fn has_comments(&self) -> bool {
        !self.leading_comments().is_empty()
            || self.leading_detached_comments().count() > 0
            || !self.trailing_comments().is_empty()
    }
}

#[cfg(test)]

mod test_data {

    lazy_static::lazy_static! {
        pub static ref DEFAULT_FILE_DESCRIPTOR_PROTO:protobuf::descriptor::FileDescriptorProto = protobuf::descriptor::FileDescriptorProto::default();
        pub static ref DEFAULT_DESCRIPTOR_PROTO:protobuf::descriptor::DescriptorProto = protobuf::descriptor::DescriptorProto::default();
        pub static ref DEFAULT_FIELD_DESCRIPTOR_PROTO:protobuf::descriptor::FieldDescriptorProto = protobuf::descriptor::FieldDescriptorProto::default();
        pub static ref DEFAULT_ENUM_DESCRIPTOR_PROTO:protobuf::descriptor::EnumDescriptorProto = protobuf::descriptor::EnumDescriptorProto::default();
        pub static ref DEFAULT_ENUM_VALUE_DESCRIPTOR_PROTO:protobuf::descriptor::EnumValueDescriptorProto = protobuf::descriptor::EnumValueDescriptorProto::default();
        pub static ref DEFAULT_SERVICE_DESCRIPTOR_PROTO:protobuf::descriptor::ServiceDescriptorProto = protobuf::descriptor::ServiceDescriptorProto::default();
        pub static ref DEFAULT_METHOD_DESCRIPTOR_PROTO:protobuf::descriptor::MethodDescriptorProto = protobuf::descriptor::MethodDescriptorProto::default();
        pub static ref DEFAULT_ONEOF_DESCRIPTOR:protobuf::descriptor::OneofDescriptorProto = protobuf::descriptor::OneofDescriptorProto::default();
    }
}
