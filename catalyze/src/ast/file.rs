use crate::{
    error::{self, HydrationFailed},
    HashMap,
};
use protobuf::{
    descriptor::{file_options::OptimizeMode as ProtoOptimizeMode, FileOptions},
    SpecialFields,
};
use snafu::{Backtrace};
use std::{
    fmt,
    hash::Hash,
    path::{Path, PathBuf},
    str::FromStr,
};

use super::{
    access::NodeKeys,
    collection::Collection,
    dependency::{self, DependenciesInner},
    dependent::{self},
    enum_, extension, extension_decl, impl_traits_and_methods, location, message, node, package,
    reference,
    resolve::Resolver,
    service,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    #[doc(hidden)]
    pub struct Key;
}
pub(super) type Table = super::table::Table<Key, Inner>;
pub type Ident = node::Ident<Key>;

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) syntax: Option<String>,
    pub(super) options: protobuf::MessageField<FileOptions>,
    pub(super) package: Option<package::Key>,
    pub(super) messages: Vec<message::Ident>,
    pub(super) enums: Vec<enum_::Ident>,
    pub(super) services: Vec<service::Ident>,
    pub(super) extensions: Vec<extension::Ident>,
    pub(super) extension_decls: Vec<extension_decl::Key>,
    pub(super) dependencies: Vec<dependency::Inner>,
    pub(super) public_dependencies: Vec<i32>,
    pub(super) weak_dependencies: Vec<i32>,
    pub(super) ext_references: Vec<reference::Inner>,
    pub(super) all_references: Vec<reference::Inner>,
    pub(super) package_comments: Option<location::Detail>,
    pub(super) comments: Option<location::Detail>,
    pub(super) is_build_target: bool,
    pub(super) special_fields: SpecialFields,
    pub(super) nodes: HashMap<FullyQualifiedName, node::Key>,
}

pub struct File<'ast>(Resolver<'ast, Key, Inner>);
impl_traits_and_methods!(File, Key, Inner);

impl<'ast> File<'ast> {
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
    pub fn parse(s: &str) -> Result<Self, error::UnsupportedSyntax> {
        match s {
            Self::PROTO2 => Ok(Self::Proto2),
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
pub(super) struct Inner {
    pub(super) key: Key,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) path: PathBuf,

    package: Option<package::Key>,
    messages: Collection<message::Key>,
    enums: Collection<enum_::Key>,
    services: Collection<service::Key>,
    defined_extensions: Collection<extension::Key>,
    extension_decls: Vec<extension_decl::Key>,
    dependencies: DependenciesInner,
    package_comments: Option<location::Comments>,
    comments: Option<location::Comments>,

    all_rferences: Vec<reference::Inner>,
    ext_references: Vec<reference::Inner>,

    dependents: dependent::DependentsInner,

    is_build_target: bool,
    syntax: Syntax,

    nodes: HashMap<FullyQualifiedName, super::node::Key>,

    java_package: Option<String>,
    java_outer_classname: Option<String>,
    java_multiple_files: bool,
    java_generate_equals_and_hash: bool,
    java_string_check_utf8: bool,
    java_generic_services: bool,
    optimize_for: Option<OptimizeMode>,
    go_package: Option<String>,
    cc_generic_services: bool,
    py_generic_services: bool,
    php_generic_services: bool,
    deprecated: bool,
    cc_enable_arenas: bool,
    objc_class_prefix: Option<String>,
    csharp_namespace: Option<String>,
    swift_prefix: Option<String>,
    php_class_prefix: Option<String>,
    php_namespace: Option<String>,
    php_metadata_namespace: Option<String>,
    ruby_package: Option<String>,
    uninterpreted_options: Vec<UninterpretedOption>,

    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        std::iter::empty()
            .chain(self.messages.iter().copied().map(Into::into))
            .chain(self.enums.iter().copied().map(Into::into))
            .chain(self.services.iter().copied().map(Into::into))
            .chain(self.defined_extensions.iter().copied().map(Into::into))
    }
}

impl Inner {
    pub(super) fn set_name_and_path(&mut self, name: Name) {
        self.path = PathBuf::from(name.as_ref());
        self.name = name;
    }
    pub(super) fn set_fqn(&mut self, fqn: FullyQualifiedName) {
        self.fqn = fqn;
    }
    pub(super) fn set_package_comments(&mut self, package_comments: location::Comments) {
        self.package_comments = Some(package_comments);
    }
    pub(super) fn set_is_build_target(&mut self, is_build_target: bool) {
        self.is_build_target = is_build_target;
    }

    pub(super) fn hydrate(
        &mut self,
        hydrate: Hydrate,
    ) -> Result<node::Ident<Key>, crate::error::HydrationFailed> {
        let Hydrate {
            name,
            syntax,
            options,
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
        self.all_rferences = all_references;
        self.ext_references = ext_references;
        self.package_comments = package_comments.and_then(|c| c.comments);
        self.comments = comments.and_then(|c| c.comments);
        self.is_build_target = hydrate.is_build_target;
        self.nodes = nodes;
        self.hydrate_options(options.unwrap_or_default())?;
        Ok(self.into())
    }
    /// Hydrates the data within the descriptor.
    ///
    /// Note: References and nested nodes are not hydrated.
    fn hydrate_options(&mut self, opts: FileOptions) -> Result<(), HydrationFailed> {
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
        self.options_special_fields = opts.special_fields;
        Ok(())
    }

    pub(crate) fn set_nodes_by_fqn(
        &mut self,
        mut nodes: HashMap<FullyQualifiedName, super::node::Key>,
    ) {
        nodes.shrink_to_fit();
        self.nodes = nodes;
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
