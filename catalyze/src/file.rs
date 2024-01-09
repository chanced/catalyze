use crate::{
    ast::{impl_traits, Accessor, Ast, FullyQualifiedName, UninterpretedOption},
    r#enum,
    error::Error,
    extension,
    location::Comments,
    message::{self},
    package::{self, Package},
    service, HashSet,
};
use protobuf::descriptor::{file_options::OptimizeMode as ProtoOptimizeMode, FileDescriptorProto};
use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

slotmap::new_key_type! {
    #[doc(hidden)]
    pub struct Key;
}

pub struct File<'ast>(Accessor<'ast, Key, Inner>);
impl_traits!(File, Key, Inner);

impl<'ast> File<'ast> {
    #[must_use]
    pub fn name(&self) -> &str {
        self.0.name.as_ref()
    }
    #[must_use]
    pub fn path(&self) -> &Path {
        self.0.path.as_ref()
    }

    #[must_use]
    pub fn package(&self) -> Option<Package> {
        self.0.package.map(|k| (k, self.0.ast).into())
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
        &self.0.uninterpreted_option
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
#[derive(Debug, Default, Clone, PartialEq)]
#[doc(hidden)]
pub(crate) struct Inner {
    pub(crate) is_hydrated: bool,
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) package: Option<package::Key>,

    pub(crate) messages: Vec<message::Key>,
    pub(crate) enums: Vec<r#enum::Key>,
    pub(crate) services: Vec<service::Key>,
    pub(crate) defined_extensions: Vec<extension::Key>,

    // file_path: PathBuf,
    pub(crate) fqn: FullyQualifiedName,

    pub(crate) package_comments: Comments,
    pub(crate) comments: Comments,
    pub(crate) dependents: Vec<Key>,
    pub(crate) imports: Vec<Key>,
    pub(crate) transitive_dependencies: Vec<Key>,
    pub(crate) transitive_dependents: Vec<Key>,
    pub(crate) used_imports: HashSet<Key>,
    pub(crate) unused_imports: HashSet<Key>,
    pub(crate) is_build_target: bool,

    pub(crate) syntax: Syntax,
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
    ///  See the documentation for the "Options" section above.
    uninterpreted_option: Vec<UninterpretedOption>,
}

impl Inner {
    /// Hydrates the data within the descriptor.
    ///
    /// Note: References and nested nodes are not hydrated.
    pub(crate) fn hydrate_data(
        &mut self,
        descriptor: &FileDescriptorProto,
        is_build_target: bool,
    ) -> Result<(), Error> {
        if self.is_hydrated {
            return Ok(());
        }
        self.name = descriptor.name.clone().unwrap_or_default();
        self.path = PathBuf::from(&self.name);
        self.is_build_target = is_build_target;
        let opts = descriptor.options.as_ref().cloned().unwrap_or_default();
        self.syntax = parse_syntax(descriptor.syntax())?;
        self.java_package = opts.java_package;
        self.java_outer_classname = opts.java_outer_classname;
        self.java_multiple_files = opts.java_multiple_files();
        self.java_generate_equals_and_hash = opts.java_generate_equals_and_hash();
        self.java_string_check_utf8 = opts.java_string_check_utf8();
        self.java_generic_services = opts.java_generic_services();
        self.optimize_for = opts.optimize_for.map(Into::into);
        self.go_package = opts.go_package;
        self.cc_generic_services = opts.cc_generic_services();
        self.py_generic_services = opts.py_generic_services();
        self.php_generic_services = opts.php_generic_services();
        self.deprecated = opts.deprecated();
        self.cc_enable_arenas = opts.cc_enable_arenas();
        self.objc_class_prefix = opts.objc_class_prefix;
        self.csharp_namespace = opts.csharp_namespace;
        self.swift_prefix = opts.swift_prefix;
        self.php_class_prefix = opts.php_class_prefix;
        self.php_namespace = opts.php_namespace;
        self.php_metadata_namespace = opts.php_metadata_namespace;
        self.ruby_package = opts.ruby_package;
        self.uninterpreted_option = opts
            .uninterpreted_option
            .into_iter()
            .map(Into::into)
            .collect();

        self.messages.reserve(descriptor.message_type.len());
        self.services.reserve(descriptor.service.len());
        self.enums.reserve(descriptor.enum_type.len());
        self.defined_extensions.reserve(descriptor.extension.len());

        self.is_hydrated = true;

        Ok(())
    }
    /// Returns false if `is_hydrated` is false or if there are no messages,
    /// services, enums, or defined extensions. Otherwise, returns true.
    ///
    /// This method should only be used as a safeguard to avoid re-hydration.
    /// Re-hydrating an empty file erronously will not have adverse effects.
    pub(crate) fn is_fully_hydrated(&self) -> bool {
        self.is_hydrated
            && (!self.messages.is_empty()
                || !self.services.is_empty()
                || !self.enums.is_empty()
                || !self.defined_extensions.is_empty())
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
// pub(crate) struct Files {
//     files: Vec<Key>,
//     fqn_lookup: HashMap<FullyQualifiedName, usize>,
//     path_lookup: HashMap<PathBuf, usize>,
// }

// impl Files {
//     pub(crate) fn new() -> Self {
//         Self {
//             files: Vec::new(),
//             fqn_lookup: HashMap::default(),
//             path_lookup: HashMap::default(),
//         }
//     }
//     pub(crate) fn files(&self) -> &[File] {
//         &self.files
//     }
//     pub(crate) fn contains_fqn(&self, fqn: &FullyQualifiedName) -> bool {
//         self.fqn_lookup.contains_key(fqn)
//     }
//     pub(crate) fn contains_path(&self, path: &Path) -> bool {
//         self.path_lookup.contains_key(path)
//     }
//     pub(crate) fn get_by_fqn(&self, fqn: &FullyQualifiedName) ->
// Option<&File> {         self.fqn_lookup.get(fqn).map(|idx| &self.files[*idx])
//     }
//     pub(crate) fn get_by_path(&self, path: impl AsRef<Path>) -> Option<Key> {
//         self.path_lookup
//             .get(path.as_ref())
//             .map(|idx| &self.files[*idx])
//     }

//     pub(crate) fn push(&mut self, path: PathBuf, fqn: FullyQualifiedName,
// key: Key) {         if self.fqn_lookup.contains_key(&fqn) {
//             return;
//         }
//         let idx = self.files.len();
//         self.fqn_lookup.insert(fqn, self.files.len());
//         self.path_lookup.insert(path, idx);
//         self.files.push(key);
//     }
// }
