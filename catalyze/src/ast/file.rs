use crate::{
    error::{self, HydrationFailed},
    HashMap,
};
use protobuf::{
    descriptor::{
        self, file_options::OptimizeMode as ProtoOptimizeMode, uninterpreted_option,
        FileOptions as ProtoFileOpts,
    },
    SpecialFields,
};
use snafu::Backtrace;
use std::{
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    str::FromStr,
};

use super::{
    access::{AccessName, AccessNodeKeys},
    collection::Collection,
    dependency::{self, DependenciesInner},
    dependent::DependentsInner,
    enum_, extension, extension_decl, impl_traits_and_methods, location, message, node, package,
    reference,
    resolve::Resolver,
    service, table,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    #[doc(hidden)]
    pub struct FileKey;
}
pub type Ident = node::Ident<FileKey>;

pub(super) trait SetPath {
    fn set_path(&mut self, path: PathBuf);
}

pub struct File<'ast>(pub(super) Resolver<'ast, FileKey, FileInner>);
impl_traits_and_methods!(File, FileKey, FileInner);

impl<'ast> File<'ast> {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0.name
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.0.path.as_ref()
    }

    #[must_use]
    pub fn is_build_target(&self) -> bool {
        self.0.is_build_target
    }

    #[must_use]
    pub fn syntax(&self) -> Syntax {
        self.0.syntax
    }
}
impl<'ast> AccessName for File<'ast> {
    fn name(&self) -> &str {
        &self.0.name
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
    pub const fn supports_required_prefix(self) -> bool {
        self.is_proto2()
    }
    #[must_use]
    pub const fn is_proto2(self) -> bool {
        matches!(self, Self::Proto2)
    }
    #[must_use]
    pub const fn is_proto3(self) -> bool {
        matches!(self, Self::Proto3)
    }
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proto2 => Self::PROTO2,
            Self::Proto3 => Self::PROTO3,
        }
    }
    pub fn parse(s: &str) -> Result<Self, error::UnsupportedSyntax> {
        match s {
            Self::PROTO2 | "" => Ok(Self::Proto2),
            Self::PROTO3 => Ok(Self::Proto3),
            _ => Err(crate::error::UnsupportedSyntax {
                backtrace: Backtrace::capture(),
                value: s.to_string(),
            }),
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
    //
    /// [`Speed`]: OptimizeMode::Speed
    #[must_use]
    pub const fn is_speed(self) -> bool {
        matches!(self, Self::Speed)
    }

    /// Returns `true` if the optimize mode is [`CodeSize`].
    ///
    /// [`CodeSize`]: OptimizeMode::CodeSize
    #[must_use]
    pub const fn is_code_size(self) -> bool {
        matches!(self, Self::CodeSize)
    }

    /// Returns `true` if the optimize mode is [`LiteRuntime`].
    ///
    /// [`LiteRuntime`]: OptimizeMode::LiteRuntime
    #[must_use]
    pub const fn is_lite_runtime(self) -> bool {
        matches!(self, Self::LiteRuntime)
    }

    /// Returns `true` if the optimize mode is [`Unknown`].
    ///
    /// [`Unknown`]: OptimizeMode::Unknown
    #[must_use]
    pub const fn is_unknown(self) -> bool {
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
pub(super) struct FileInner {
    pub(super) key: FileKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) path: PathBuf,
    pub(super) package: Option<package::PackageKey>,
    pub(super) messages: Collection<message::MessageKey>,
    pub(super) enums: Collection<enum_::EnumKey>,
    pub(super) services: Collection<service::ServiceKey>,
    pub(super) defined_extensions: Collection<extension::ExtensionKey>,
    pub(super) extension_decls: Vec<extension_decl::ExtensionDeclKey>,
    pub(super) dependencies: DependenciesInner,
    pub(super) package_comments: Option<location::Comments>,
    pub(super) comments: Option<location::Comments>,
    pub(super) all_references: Vec<reference::ReferenceInner>,
    pub(super) ext_references: Vec<reference::ReferenceInner>,
    pub(super) dependents: DependentsInner,
    pub(super) is_build_target: bool,
    pub(super) syntax: Syntax,
    pub(super) nodes: HashMap<FullyQualifiedName, super::node::NodeKey>,
    pub(super) special_fields: SpecialFields,
    pub(super) options_special_fields: SpecialFields,
    pub(super) options: FileOptions,
    pub(super) proto_opts: descriptor::FileOptions,
}

impl AccessNodeKeys for FileInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        std::iter::empty()
            .chain(self.messages.iter().copied().map(Into::into))
            .chain(self.enums.iter().copied().map(Into::into))
            .chain(self.services.iter().copied().map(Into::into))
            .chain(self.defined_extensions.iter().copied().map(Into::into))
    }
}

impl SetPath for FileInner {
    fn set_path(&mut self, path: PathBuf) {
        self.set_path_and_name(path);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileOptions {
    pub java_package: Option<String>,
    pub java_outer_classname: Option<String>,
    pub java_multiple_files: Option<bool>,
    pub java_generate_equals_and_hash: Option<bool>,
    pub java_string_check_utf8: Option<bool>,
    pub java_generic_services: Option<bool>,
    pub optimize_for: Option<OptimizeMode>,
    pub go_package: Option<String>,
    pub cc_generic_services: Option<bool>,
    pub py_generic_services: Option<bool>,
    pub php_generic_services: Option<bool>,
    pub deprecated: Option<bool>,
    pub cc_enable_arenas: Option<bool>,
    pub objc_class_prefix: Option<String>,
    pub csharp_namespace: Option<String>,
    pub swift_prefix: Option<String>,
    pub php_class_prefix: Option<String>,
    pub php_namespace: Option<String>,
    pub php_metadata_namespace: Option<String>,
    pub ruby_package: Option<String>,
    pub uninterpreted_options: Vec<UninterpretedOption>,
    pub special_fields: SpecialFields,
}
impl FileOptions {
    fn hydrate(&mut self, opts: &mut descriptor::FileOptions) {
        self.java_package = opts.java_package.take();
        self.java_outer_classname = opts.java_outer_classname.take();
        self.java_multiple_files = opts.java_multiple_files.take();
        self.java_generate_equals_and_hash = opts.java_generate_equals_and_hash.take();
        self.java_string_check_utf8 = opts.java_string_check_utf8.take();
        self.java_generic_services = opts.java_generic_services.take();
        self.optimize_for = opts.optimize_for.take().map(Into::into);
        self.go_package = opts.go_package.take();
        self.cc_generic_services = opts.cc_generic_services.take();
        self.py_generic_services = opts.py_generic_services.take();
        self.php_generic_services = opts.php_generic_services.take();
        self.deprecated = opts.deprecated.take();
        self.cc_enable_arenas = opts.cc_enable_arenas.take();
        self.objc_class_prefix = opts.objc_class_prefix.take();
        self.csharp_namespace = opts.csharp_namespace.take();
        self.swift_prefix = opts.swift_prefix.take();
        self.php_class_prefix = opts.php_class_prefix.take();
        self.php_namespace = opts.php_namespace.take();
        self.php_metadata_namespace = opts.php_metadata_namespace.take();
        self.ruby_package = opts.ruby_package.take();
        let uninterpreted_options = std::mem::take(&mut opts.uninterpreted_option);
        self.uninterpreted_options = uninterpreted_options.into_iter().map(Into::into).collect();
    }
    #[must_use]
    pub fn java_multiple_files(&self) -> bool {
        self.java_multiple_files.unwrap_or(false)
    }

    #[must_use]
    pub fn java_package(&self) -> Option<&str> {
        self.java_package.as_deref()
    }

    #[must_use]
    pub fn java_outer_classname(&self) -> Option<&str> {
        self.java_outer_classname.as_deref()
    }

    #[must_use]
    pub fn java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.unwrap_or(false)
    }

    #[must_use]
    pub fn java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.unwrap_or(false)
    }

    #[must_use]
    pub fn optimize_for(&self) -> Option<OptimizeMode> {
        self.optimize_for
    }

    #[must_use]
    pub fn go_package(&self) -> Option<&str> {
        self.go_package.as_deref()
    }

    #[must_use]
    pub fn cc_generic_services(&self) -> bool {
        self.cc_generic_services.unwrap_or(false)
    }

    #[must_use]
    pub fn java_generic_services(&self) -> bool {
        self.java_generic_services.unwrap_or(false)
    }

    #[must_use]
    pub fn py_generic_services(&self) -> bool {
        self.py_generic_services.unwrap_or(false)
    }

    #[must_use]
    pub fn php_generic_services(&self) -> bool {
        self.php_generic_services.unwrap_or(false)
    }

    ///  Is this file deprecated?
    ///  Depending on the target platform, this can emit Deprecated annotations
    ///  for everything in the file, or it will be completely ignored; in the
    /// very  least, this is a formalization for deprecating files.
    #[must_use]
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    ///  Enables the use of arenas for the proto messages in this file. This
    /// applies  only to generated classes for C++.
    #[must_use]
    pub fn cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.unwrap_or(false)
    }

    ///  Sets the objective c class prefix which is prepended to all objective c
    ///  generated classes from this .proto. There is no default.
    #[must_use]
    pub fn objc_class_prefix(&self) -> Option<&str> {
        self.objc_class_prefix.as_deref()
    }

    ///  Namespace for generated classes; defaults to the package.
    #[must_use]
    pub fn csharp_namespace(&self) -> Option<&str> {
        self.csharp_namespace.as_deref()
    }

    ///  By default Swift generators will take the proto package and CamelCase
    /// it  replacing '.' with underscore and use that to prefix the
    /// types/symbols  defined. When this options is provided, they will use
    /// this value instead  to prefix the types/symbols defined.
    #[must_use]
    pub fn swift_prefix(&self) -> Option<&str> {
        self.swift_prefix.as_deref()
    }

    ///  Sets the php class prefix which is prepended to all php generated
    /// classes  from this .proto. Default is empty.
    #[must_use]
    pub fn php_class_prefix(&self) -> Option<&str> {
        self.php_class_prefix.as_deref()
    }

    ///  Use this option to change the namespace of php generated classes.
    /// Default  is empty. When this option is empty, the package name will
    /// be used for  determining the namespace.
    #[must_use]
    pub fn php_namespace(&self) -> Option<&str> {
        self.php_namespace.as_deref()
    }

    ///  Use this option to change the namespace of php generated metadata
    /// classes.  Default is empty. When this option is empty, the proto
    /// file name will be  used for determining the namespace.
    #[must_use]
    pub fn php_metadata_namespace(&self) -> Option<&str> {
        self.php_metadata_namespace.as_deref()
    }

    ///  Use this option to change the package of ruby generated classes.
    /// Default  is empty. When this option is not set, the package name
    /// will be used for  determining the ruby package.
    #[must_use]
    pub fn ruby_package(&self) -> Option<&str> {
        self.ruby_package.as_deref()
    }

    ///  The parser stores options it doesn't recognize here.
    ///  See the documentation for the "Options" section above.
    #[must_use]
    pub fn uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_options
    }
}
impl FileInner {
    pub(super) fn set_name_and_path(&mut self, name: Name) {
        self.path = PathBuf::from(name.as_ref());
        self.name = name;
    }
    pub(super) fn set_path_and_name(&mut self, path: PathBuf) {
        self.name = Name(path.to_str().unwrap().into());
        self.path = path;
    }

    pub(super) fn hydrate(
        &mut self,
        hydrate: Hydrate,
    ) -> Result<node::Ident<FileKey>, crate::error::HydrationFailed> {
        let Hydrate {
            name,
            syntax,
            mut options,
            package,
            messages,
            enums,
            services,
            extensions,
            extension_decls,
            dependencies,
            all_references,
            ext_references,
            package_comments,
            comments,
            is_build_target,
            nodes,
            public_dependencies,
            weak_dependencies,
            special_fields,
        } = hydrate;
        self.set_name_and_path(name);
        self.syntax = Syntax::parse(&syntax.unwrap_or_default())?;

        self.package = package;
        self.messages = messages.into();
        self.enums = enums.into();
        self.services = services.into();
        self.defined_extensions = extensions.into();
        self.extension_decls = extension_decls;
        self.is_build_target = is_build_target;
        self.special_fields = special_fields;
        self.dependencies =
            DependenciesInner::new(dependencies, public_dependencies, weak_dependencies)?;
        self.all_references = all_references;
        self.ext_references = ext_references;
        self.package_comments = package_comments.and_then(|c| c.comments);
        self.comments = comments.and_then(|c| c.comments);
        self.is_build_target = hydrate.is_build_target;
        self.nodes = nodes;
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        Ok(self.into())
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

#[derive(Debug, Clone, Default)]
pub(super) struct Table(table::Table<FileKey, FileInner, HashMap<PathBuf, FileKey>>);
impl Table {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self(table::Table::with_capacity(capacity))
    }
    pub(super) fn get_by_name(&self, name: &str) -> Option<&FileInner> {
        self.0.iter().find(|(_, v)| v.name == *name).map(|(_, v)| v)
    }
}
impl Deref for Table {
    type Target = table::Table<FileKey, FileInner, HashMap<PathBuf, FileKey>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) syntax: Option<String>,
    pub(super) options: ProtoFileOpts,
    pub(super) package: Option<package::PackageKey>,
    pub(super) messages: Vec<message::Ident>,
    pub(super) enums: Vec<enum_::EnumIdent>,
    pub(super) services: Vec<service::Ident>,
    pub(super) extensions: Vec<extension::Ident>,
    pub(super) extension_decls: Vec<extension_decl::ExtensionDeclKey>,
    pub(super) dependencies: Vec<dependency::DependencyInner>,
    pub(super) public_dependencies: Vec<i32>,
    pub(super) weak_dependencies: Vec<i32>,
    pub(super) ext_references: Vec<reference::ReferenceInner>,
    pub(super) all_references: Vec<reference::ReferenceInner>,
    pub(super) package_comments: Option<location::Location>,
    pub(super) comments: Option<location::Location>,
    pub(super) is_build_target: bool,
    pub(super) special_fields: SpecialFields,
    pub(super) nodes: HashMap<FullyQualifiedName, node::NodeKey>,
}
