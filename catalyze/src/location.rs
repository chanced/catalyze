#[derive(Clone, Debug, Default)]
pub struct Comments {
    /// Any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub(crate) leading: Option<String>,
    pub(crate) trailing: Option<String>,
    pub(crate) leading_detached: Vec<String>,
}

impl Comments {
    /// Any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading(&self) -> Option<&str> {
        self.leading.as_deref()
    }

    /// Any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing(&self) -> Option<&str> {
        self.trailing.as_deref()
    }

    /// Each comment block or line above the entity but seperated by whitespace.
    pub fn leading_detached(&self) -> &[String] {
        &self.leading_detached
    }
}

pub(crate) struct Location {
    path: Vec<i32>,
    ///  Always has exactly three or four elements: start line, start column,
    ///  end line (optional, otherwise assumed same as start line), end column.
    ///  These are packed into a single field for efficiency.  Note that line
    ///  and column numbers are zero-based -- typically you will want to add
    ///  1 to each before displaying to a user.
    span: Vec<i32>,
}
impl Location {
    pub fn path(&self) -> &[i32] {
        &self.path
    }
    ///  Always has exactly three or four elements: start line, start column,
    ///  end line (optional, otherwise assumed same as start line), end column.
    ///  These are packed into a single field for efficiency.  Note that line
    ///  and column numbers are zero-based -- typically you will want to add
    ///  1 to each before displaying to a user.
    pub fn span(&self) -> &[i32] {
        &self.span
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum FileDescriptorPath {
    /// file name, relative to root of source tree
    Name = 1,
    /// FileDescriptorProto.package
    Package = 2,
    /// Names of files imported by this file.
    Dependency = 3,

    /// Indexes of the public imported files in the dependency list above.
    PublicDependency = 10,

    /// Indexes of the weak imported files in the dependency list.
    /// For Google-internal migration only. Do not use.
    WeakDependency = 11,

    // All top-level definitions in this file.
    MessageType = 4,
    /// FileDescriptorProto.enum_type
    EnumType = 5,
    /// FileDescriptorProto.service
    Service = 6,
    /// FileDescriptorProto.extension
    Extension = 7,

    Options = 8,
    /// This field contains optional information about the original source code.
    /// You may safely remove this entire field without harming runtime
    /// functionality of the descriptors -- the information is needed only by
    /// development tools.
    SourceCodeInfo = 9,

    /// FileDescriptorProto.syntax
    Syntax = 12,
}
impl FileDescriptorPath {
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    pub(crate) const NAME: i32 = Self::Name.as_i32();
    pub(crate) const PACKAGE: i32 = Self::Package.as_i32();
    pub(crate) const DEPENDENCY: i32 = Self::Dependency.as_i32();
    pub(crate) const PUBLIC_DEPENDENCY: i32 = Self::PublicDependency.as_i32();
    pub(crate) const WEAK_DEPENDENCY: i32 = Self::WeakDependency.as_i32();
    pub(crate) const MESSAGE_TYPE: i32 = Self::MessageType.as_i32();
    pub(crate) const ENUM_TYPE: i32 = Self::EnumType.as_i32();
    pub(crate) const SERVICE: i32 = Self::Service.as_i32();
    pub(crate) const EXTENSION: i32 = Self::Extension.as_i32();
    pub(crate) const OPTIONS: i32 = Self::Options.as_i32();
    pub(crate) const SOURCE_CODE_INFO: i32 = Self::SourceCodeInfo.as_i32();
    pub(crate) const SYNTAX: i32 = Self::Syntax.as_i32();
}

impl TryFrom<i32> for FileDescriptorPath {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            Self::NAME => Ok(Self::Name),
            Self::PACKAGE => Ok(Self::Package),
            Self::DEPENDENCY => Ok(Self::Dependency),
            Self::PUBLIC_DEPENDENCY => Ok(Self::PublicDependency),
            Self::WEAK_DEPENDENCY => Ok(Self::WeakDependency),
            Self::MESSAGE_TYPE => Ok(Self::MessageType),
            Self::ENUM_TYPE => Ok(Self::EnumType),
            Self::SERVICE => Ok(Self::Service),
            Self::EXTENSION => Ok(Self::Extension),
            Self::OPTIONS => Ok(Self::Options),
            Self::SOURCE_CODE_INFO => Ok(Self::SourceCodeInfo),
            Self::SYNTAX => Ok(Self::Syntax),
            _ => Err(v),
        }
    }
}

impl PartialEq<i32> for FileDescriptorPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self as i32
    }
}
impl PartialEq<FileDescriptorPath> for i32 {
    fn eq(&self, other: &FileDescriptorPath) -> bool {
        *other == *self
    }
}

/// Paths for nodes in a [`DescriptorProto`]
#[derive(Clone, PartialEq, Eq, Copy)]
pub(crate) enum DescriptorPath {
    /// DescriptorProto.field
    Field = 2,
    /// DescriptorProto.nested_type
    NestedType = 3,
    /// DescriptorProto.enum_type
    EnumType = 4,
    Extension = 6,

    /// DescriptorProto.oneof_decl
    OneofDecl = 8,
}

impl DescriptorPath {
    pub const fn as_i32(self) -> i32 {
        self as i32
    }
    pub(crate) const FIELD: i32 = Self::Field.as_i32();
    pub(crate) const NESTED_TYPE: i32 = Self::NestedType.as_i32();
    pub(crate) const ENUM_TYPE: i32 = Self::EnumType.as_i32();
    pub(crate) const EXTENSION: i32 = Self::Extension.as_i32();
    pub(crate) const ONEOF_DECL: i32 = Self::OneofDecl.as_i32();
}

impl TryFrom<i32> for DescriptorPath {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            Self::FIELD => Ok(Self::Field),
            Self::NESTED_TYPE => Ok(Self::NestedType),
            Self::ENUM_TYPE => Ok(Self::EnumType),
            Self::EXTENSION => Ok(Self::Extension),
            Self::ONEOF_DECL => Ok(Self::OneofDecl),
            _ => Err(v),
        }
    }
}
