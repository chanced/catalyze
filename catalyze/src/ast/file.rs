use crate::error::Error;
use protobuf::descriptor::{file_options::OptimizeMode as ProtoOptimizeMode, FileOptions};
use std::{
    fmt,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

use super::{
    access::NodeKeys, r#enum, extension, impl_traits_and_methods, message, package, service,
    Comments, FullyQualifiedName, Resolver, State, UninterpretedOption,
};

slotmap::new_key_type! {
    #[doc(hidden)]
    pub struct Key;
}

pub struct File<'ast>(Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(File, Key, Inner);

impl<'ast> File<'ast> {
    #[must_use]
    pub fn path(&self) -> &Path {
        self.0.path.as_ref()
    }

    // #[must_use]
    // pub fn used_imports(&self) -> &HashSet<String> {
    //     &self.0.used_imports
    // }

    #[must_use]
    pub fn is_build_target(&self) -> bool {
        self.0.is_build_target
    }

    #[must_use]
    pub fn syntax(&self) -> Syntax {
        self.0.syntax
    }

    #[must_use]
    pub fn java_multiple_files(&self) -> bool {
        self.0.java_multiple_files
    }

    #[must_use]
    pub fn java_package(&self) -> Option<&str> {
        self.0.java_package.as_deref()
    }

    #[must_use]
    pub fn java_outer_classname(&self) -> Option<&str> {
        self.0.java_outer_classname.as_deref()
    }

    #[must_use]
    pub fn java_generate_equals_and_hash(&self) -> bool {
        self.0.java_generate_equals_and_hash
    }

    #[must_use]
    pub fn java_string_check_utf8(&self) -> bool {
        self.0.java_string_check_utf8
    }

    #[must_use]
    pub fn optimize_for(&self) -> Option<OptimizeMode> {
        self.0.optimize_for
    }

    #[must_use]
    pub fn go_package(&self) -> Option<&str> {
        self.0.go_package.as_deref()
    }

    #[must_use]
    pub fn cc_generic_services(&self) -> bool {
        self.0.cc_generic_services
    }

    #[must_use]
    pub fn java_generic_services(&self) -> bool {
        self.0.java_generic_services
    }

    #[must_use]
    pub fn py_generic_services(&self) -> bool {
        self.0.py_generic_services
    }

    #[must_use]
    pub fn php_generic_services(&self) -> bool {
        self.0.php_generic_services
    }

    ///  Is this file deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for everything in the file, or it will be completely ignored; in the
    /// very  least, this is a formalization for deprecating files.
    #[must_use]
    pub fn deprecated(&self) -> bool {
        self.0.deprecated
    }

    ///  Enables the use of arenas for the proto messages in this file. This
    /// applies  only to generated classes for C++.
    #[must_use]
    pub fn cc_enable_arenas(&self) -> bool {
        self.0.cc_enable_arenas
    }

    ///  Sets the objective c class prefix which is prepended to all objective c
    ///  generated classes from this .proto. There is no default.
    #[must_use]
    pub fn objc_class_prefix(&self) -> Option<&str> {
        self.0.objc_class_prefix.as_deref()
    }

    ///  Namespace for generated classes; defaults to the package.
    #[must_use]
    pub fn csharp_namespace(&self) -> Option<&str> {
        self.0.csharp_namespace.as_deref()
    }

    ///  By default Swift generators will take the proto package and CamelCase
    /// it  replacing '.' with underscore and use that to prefix the
    /// types/symbols  defined. When this options is provided, they will use
    /// this value instead  to prefix the types/symbols defined.
    #[must_use]
    pub fn swift_prefix(&self) -> Option<&str> {
        self.0.swift_prefix.as_deref()
    }

    ///  Sets the php class prefix which is prepended to all php generated
    /// classes  from this .proto. Default is empty.
    #[must_use]
    pub fn php_class_prefix(&self) -> Option<&str> {
        self.0.php_class_prefix.as_deref()
    }

    ///  Use this option to change the namespace of php generated classes.
    /// Default  is empty. When this option is empty, the package name will
    /// be used for  determining the namespace.
    #[must_use]
    pub fn php_namespace(&self) -> Option<&str> {
        self.0.php_namespace.as_deref()
    }

    ///  Use this option to change the namespace of php generated metadata
    /// classes.  Default is empty. When this option is empty, the proto
    /// file name will be  used for determining the namespace.
    #[must_use]
    pub fn php_metadata_namespace(&self) -> Option<&str> {
        self.0.php_metadata_namespace.as_deref()
    }

    ///  Use this option to change the package of ruby generated classes.
    /// Default  is empty. When this option is not set, the package name
    /// will be used for  determining the ruby package.
    #[must_use]
    pub fn ruby_package(&self) -> Option<&str> {
        self.0.ruby_package.as_deref()
    }

    ///  The parser stores options it doesn't recognize here.
    ///  See the documentation for the "Options" section above.
    #[must_use]
    pub fn uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.0.uninterpreted_options
    }
}

/// Syntax of the proto file. Lorem ipsum dolor sit amet, consectetur adipiscing
/// elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet,
/// adipiscing nec, ultricies sed, dolor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Syntax {
    Proto2,
    Proto3,
}
impl Syntax {
    const PROTO2: &'static str = "proto2";
    const PROTO3: &'static str = "proto3";
    #[must_use]
    pub const fn supports_required_prefix(&self) -> bool {
        self.is_proto2()
    }
    #[must_use]
    pub const fn is_proto2(&self) -> bool {
        matches!(self, Self::Proto2)
    }
    #[must_use]
    pub const fn is_proto3(&self) -> bool {
        matches!(self, Self::Proto3)
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Proto2 => Self::PROTO2,
            Self::Proto3 => Self::PROTO3,
        }
    }
    pub fn parse(s: &str) -> Result<Self, Error> {
        match s {
            Self::PROTO2 => Ok(Self::Proto2),
            Self::PROTO3 => Ok(Self::Proto3),
            _ => Err(Error::unsupported_syntax(s.to_owned())),
        }
    }
}
impl fmt::Display for Syntax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Syntax {
    type Err = ();

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match &*v.to_lowercase() {
            Self::PROTO2 | "" => Ok(Self::Proto2),
            Self::PROTO3 => Ok(Self::Proto3),
            _ => Err(()),
        }
    }
}
impl Default for Syntax {
    fn default() -> Self {
        Self::Proto2
    }
}

impl TryFrom<&str> for Syntax {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        Self::from_str(v)
    }
}
impl TryFrom<String> for Syntax {
    type Error = ();
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Self::from_str(&v)
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

    /// Unknown optimize mode
    Unknown(i32),
}

impl OptimizeMode {
    /// Returns `true` if the optimize mode is [`Speed`].
    ///
    /// [`Speed`]: OptimizeMode::Speed
    #[must_use]
    pub const fn is_speed(&self) -> bool {
        matches!(self, Self::Speed)
    }

    /// Returns `true` if the optimize mode is [`CodeSize`].
    ///
    /// [`CodeSize`]: OptimizeMode::CodeSize
    #[must_use]
    pub const fn is_code_size(&self) -> bool {
        matches!(self, Self::CodeSize)
    }

    /// Returns `true` if the optimize mode is [`LiteRuntime`].
    ///
    /// [`LiteRuntime`]: OptimizeMode::LiteRuntime
    #[must_use]
    pub const fn is_lite_runtime(&self) -> bool {
        matches!(self, Self::LiteRuntime)
    }

    /// Returns `true` if the optimize mode is [`Unknown`].
    ///
    /// [`Unknown`]: OptimizeMode::Unknown
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(..))
    }
}
impl From<protobuf::EnumOrUnknown<ProtoOptimizeMode>> for OptimizeMode {
    fn from(value: protobuf::EnumOrUnknown<ProtoOptimizeMode>) -> Self {
        match value.enum_value() {
            Ok(o) => Self::from(o),
            Err(i) => Self::Unknown(i),
        }
    }
}
impl From<protobuf::descriptor::file_options::OptimizeMode> for OptimizeMode {
    fn from(value: protobuf::descriptor::file_options::OptimizeMode) -> Self {
        match value {
            ProtoOptimizeMode::SPEED => Self::Speed,
            ProtoOptimizeMode::CODE_SIZE => Self::CodeSize,
            ProtoOptimizeMode::LITE_RUNTIME => Self::LiteRuntime,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(super) struct ImportInner {
    is_used: bool,
    is_public: bool,
    is_weak: bool,
    file: Key,
}

pub struct Import<'ast> {
    pub is_used: bool,
    pub is_public: bool,
    pub is_weak: bool,
    /// The imported `File`
    pub import: File<'ast>,
    /// The [`File`] containing this import.
    pub imported_by: File<'ast>,
}

impl<'ast> Deref for Import<'ast> {
    type Target = File<'ast>;

    fn deref(&self) -> &Self::Target {
        &self.import
    }
}

impl<'ast> Import<'ast> {
    #[must_use]
    pub fn is_used(self) -> bool {
        self.is_used
    }
    #[must_use]
    pub fn is_public(self) -> bool {
        self.is_public
    }
    #[must_use]
    pub fn is_weak(self) -> bool {
        self.is_weak
    }
    #[must_use]
    pub fn import(self) -> File<'ast> {
        self.import
    }
    #[must_use]
    pub fn imported_by(self) -> File<'ast> {
        self.imported_by
    }
    #[must_use]
    pub fn as_file(self) -> File<'ast> {
        self.import
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
#[doc(hidden)]
pub(super) struct Inner {
    key: Key,
    state: State,
    name: String,
    path: PathBuf,
    package: Option<package::Key>,
    messages: Vec<message::Key>,
    enums: Vec<r#enum::Key>,
    services: Vec<service::Key>,
    defined_extensions: Vec<extension::Key>,
    used_imports: Vec<ImportInner>,
    unused_imports: Vec<ImportInner>,
    imports: Vec<ImportInner>,
    public_imports: Vec<ImportInner>,
    weak_imports: Vec<ImportInner>,
    fqn: FullyQualifiedName,
    package_comments: Comments,
    comments: Comments,
    dependents: Vec<Key>,
    transitive_dependencies: Vec<Key>,
    transitive_dependents: Vec<Key>,
    is_build_target: bool,
    syntax: Syntax,

    ///  Sets the Java package where classes generated from this .proto will be
    ///  placed.  By default, the proto package is used, but this is often
    ///  inappropriate because proto packages do not normally start with
    /// backwards  domain names.
    java_package: Option<String>,

    ///  Controls the name of the wrapper Java class generated for the .proto
    /// file.  That class will always contain the .proto file's
    /// getDescriptor() method as  well as any top-level extensions defined
    /// in the .proto file.  If java_multiple_files is disabled, then all
    /// the other classes from the  .proto file will be nested inside the
    /// single wrapper outer class.
    java_outer_classname: Option<String>,

    ///  If enabled, then the Java code generator will generate a separate .java
    ///  file for each top-level message, enum, and service defined in the
    /// .proto  file.  Thus, these types will *not* be nested inside the
    /// wrapper class  named by java_outer_classname.  However, the wrapper
    /// class will still be  generated to contain the file's getDescriptor()
    /// method as well as any  top-level extensions defined in the file.
    java_multiple_files: bool,

    ///  This option does nothing.
    java_generate_equals_and_hash: bool,

    ///  If set true, then the Java2 code generator will generate code that
    ///  throws an exception whenever an attempt is made to assign a non-UTF-8
    ///  byte sequence to a string field.
    ///  Message reflection will do the same.
    ///  However, an extension field still accepts non-UTF-8 byte sequences.
    ///  This option has no effect on when used with the lite runtime.
    java_string_check_utf8: bool,

    java_generic_services: bool,

    optimize_for: Option<OptimizeMode>,
    ///  Sets the Go package where structs generated from this .proto will be
    ///  placed. If omitted, the Go package will be derived from the following:
    ///    - The basename of the package import path, if provided.
    ///    - Otherwise, the package statement in the .proto file, if present.
    ///    - Otherwise, the basename of the .proto file, without extension.
    go_package: Option<String>,
    ///  Should generic services be generated in each language?  "Generic"
    /// services  are not specific to any particular RPC system.  They are
    /// generated by the  main code generators in each language (without
    /// additional plugins).  Generic services were the only kind of service
    /// generation supported by  early versions of google.protobuf.
    ///
    ///  Generic services are now considered deprecated in favor of using
    /// plugins  that generate code specific to your particular RPC system.
    /// Therefore,  these default to false.  Old code which depends on
    /// generic services should  explicitly set them to true.
    cc_generic_services: bool,
    py_generic_services: bool,
    php_generic_services: bool,
    ///  Is this file deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for everything in the file, or it will be completely ignored; in the
    /// very  least, this is a formalization for deprecating files.
    deprecated: bool,
    ///  Enables the use of arenas for the proto messages in this file. This
    /// applies  only to generated classes for C++.
    cc_enable_arenas: bool,
    ///  Sets the objective c class prefix which is prepended to all objective c
    ///  generated classes from this .proto. There is no default.
    objc_class_prefix: Option<String>,
    ///  Namespace for generated classes; defaults to the package.
    csharp_namespace: Option<String>,
    ///  By default Swift generators will take the proto package and CamelCase
    /// it  replacing '.' with underscore and use that to prefix the
    /// types/symbols  defined. When this options is provided, they will use
    /// this value instead  to prefix the types/symbols defined.
    swift_prefix: Option<String>,
    ///  Sets the php class prefix which is prepended to all php generated
    /// classes  from this .proto. Default is empty.
    php_class_prefix: Option<String>,
    ///  Use this option to change the namespace of php generated classes.
    /// Default  is empty. When this option is empty, the package name will
    /// be used for  determining the namespace.
    php_namespace: Option<String>,
    ///  Use this option to change the namespace of php generated metadata
    /// classes.  Default is empty. When this option is empty, the proto
    /// file name will be  used for determining the namespace.
    php_metadata_namespace: Option<String>,
    ///  Use this option to change the package of ruby generated classes.
    /// Default  is empty. When this option is not set, the package name
    /// will be used for  determining the ruby package.
    ruby_package: Option<String>,
    ///  The parser stores options it doesn't recognize here.
    uninterpreted_options: Vec<UninterpretedOption>,

    pub(super) unknown_option_fields: protobuf::UnknownFields,
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::Key> {
        self.messages
            .iter()
            .copied()
            .map(Into::into)
            .chain(self.enums.iter().copied().map(Into::into))
            .chain(self.services.iter().copied().map(Into::into))
            .chain(self.defined_extensions.iter().copied().map(Into::into))
    }
}
impl Inner {
    pub(super) fn set_name_and_path(&mut self, name: String) {
        self.path = PathBuf::from(&name);
        self.set_name(name);
    }
    pub(super) fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }
    pub(super) fn set_messages(&mut self, messages: Vec<message::Key>) {
        self.messages = messages;
    }
    pub(super) fn set_enums(&mut self, enums: Vec<r#enum::Key>) {
        self.enums = enums;
    }
    pub(super) fn set_services(&mut self, services: Vec<service::Key>) {
        self.services = services;
    }
    pub(super) fn set_defined_extensions(&mut self, defined_extensions: Vec<extension::Key>) {
        self.defined_extensions = defined_extensions;
    }
    pub(super) fn set_used_imports(&mut self, used_imports: Vec<ImportInner>) {
        self.used_imports = used_imports;
    }
    pub(super) fn set_unused_imports(&mut self, unused_imports: Vec<ImportInner>) {
        self.unused_imports = unused_imports;
    }
    pub(super) fn set_imports(&mut self, imports: Vec<ImportInner>) {
        self.imports = imports;
    }
    pub(super) fn set_public_imports(&mut self, public_imports: Vec<ImportInner>) {
        self.public_imports = public_imports;
    }
    pub(super) fn set_weak_imports(&mut self, weak_imports: Vec<ImportInner>) {
        self.weak_imports = weak_imports;
    }
    pub(super) fn set_fqn(&mut self, fqn: FullyQualifiedName) {
        self.fqn = fqn;
    }
    pub(super) fn set_package_comments(&mut self, package_comments: Comments) {
        self.package_comments = package_comments;
    }
    pub(super) fn set_comments(&mut self, comments: Comments) {
        self.comments = comments;
    }
    pub(super) fn set_dependents(&mut self, dependents: Vec<Key>) {
        self.dependents = dependents;
    }
    pub(super) fn set_transitive_dependencies(&mut self, transitive_dependencies: Vec<Key>) {
        self.transitive_dependencies = transitive_dependencies;
    }
    pub(super) fn set_transitive_dependents(&mut self, transitive_dependents: Vec<Key>) {
        self.transitive_dependents = transitive_dependents;
    }
    pub(super) fn set_is_build_target(&mut self, is_build_target: bool) {
        self.is_build_target = is_build_target;
    }
    pub(super) fn set_syntax(&mut self, syntax: Option<String>) -> Result<(), Error> {
        self.syntax = parse_syntax(&syntax.unwrap_or_default())?;
        Ok(())
    }
    /// Hydrates the data within the descriptor.
    ///
    /// Note: References and nested nodes are not hydrated.
    pub(super) fn hydrate_options(&mut self, opts: FileOptions, is_build_target: bool) {
        self.is_build_target = is_build_target;

        self.java_package = opts.java_package;
        self.java_outer_classname = opts.java_outer_classname;
        self.java_multiple_files = opts.java_multiple_files.unwrap_or(false);
        self.java_generate_equals_and_hash = opts.java_generate_equals_and_hash.unwrap_or(false);
        self.java_string_check_utf8 = opts.java_string_check_utf8.unwrap_or(false);
        self.java_generic_services = opts.java_generic_services.unwrap_or(false);
        self.optimize_for = opts.optimize_for.map(Into::into);
        self.go_package = opts.go_package;
        self.cc_generic_services = opts.cc_generic_services.unwrap_or(false);
        self.py_generic_services = opts.py_generic_services.unwrap_or(false);
        self.php_generic_services = opts.php_generic_services.unwrap_or(false);
        self.deprecated = opts.deprecated.unwrap_or(false);
        self.cc_enable_arenas = opts.cc_enable_arenas.unwrap_or(false);
        self.objc_class_prefix = opts.objc_class_prefix;
        self.csharp_namespace = opts.csharp_namespace;
        self.swift_prefix = opts.swift_prefix;
        self.php_class_prefix = opts.php_class_prefix;
        self.php_namespace = opts.php_namespace;
        self.php_metadata_namespace = opts.php_metadata_namespace;
        self.ruby_package = opts.ruby_package;
        self.uninterpreted_options = opts
            .uninterpreted_option
            .into_iter()
            .map(Into::into)
            .collect();
        self.unknown_option_fields = opts.special_fields.unknown_fields().clone();
    }
}

fn parse_syntax(syntax: &str) -> Result<Syntax, Error> {
    match syntax {
        "proto2" => Ok(Syntax::Proto2),
        "proto3" => Ok(Syntax::Proto3),
        _ => Err(Error::unsupported_syntax(syntax)),
    }
}

// #[derive(Debug, Default)]
// pub(super) struct Files {
//     files: Vec<Key>,
//     fqn_lookup: HashMap<FullyQualifiedName, usize>,
//     path_lookup: HashMap<PathBuf, usize>,
// }

// impl Files {
//     pub(super) fn new() -> Self {
//         Self {
//             files: Vec::new(),
//             fqn_lookup: HashMap::default(),
//             path_lookup: HashMap::default(),
//         }
//     }
//     pub(super) fn files(&self) -> &[File] {
//         &self.files
//     }
//     pub(super) fn contains_fqn(&self, fqn: &FullyQualifiedName) -> bool {
//         self.fqn_lookup.contains_key(fqn)
//     }
//     pub(super) fn contains_path(&self, path: &Path) -> bool {
//         self.path_lookup.contains_key(path)
//     }
//     pub(super) fn get_by_fqn(&self, fqn: &FullyQualifiedName) ->
// Option<&File> {         self.fqn_lookup.get(fqn).map(|idx| &self.files[*idx])
//     }
//     pub(super) fn get_by_path(&self, path: impl AsRef<Path>) -> Option<Key> {
//         self.path_lookup
//             .get(path.as_ref())
//             .map(|idx| &self.files[*idx])
//     }

//     pub(super) fn push(&mut self, path: PathBuf, fqn: FullyQualifiedName,
// key: Key) {         if self.fqn_lookup.contains_key(&fqn) {
//             return;
//         }
//         let idx = self.files.len();
//         self.fqn_lookup.insert(fqn, self.files.len());
//         self.path_lookup.insert(path, idx);
//         self.files.push(key);
//     }
// }
