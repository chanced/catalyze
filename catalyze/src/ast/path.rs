#[derive(Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub(super) enum FilePath {
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
    Message = 4,
    /// FileDescriptorProto.enum_type
    Enum = 5,
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

    Unknown(i32),
}

impl FilePath {
    const NAME: i32 = 1;
    const PACKAGE: i32 = 2;
    const DEPENDENCY: i32 = 3;
    const PUBLIC_DEPENDENCY: i32 = 10;
    const WEAK_DEPENDENCY: i32 = 11;
    const MESSAGE: i32 = 4;
    const ENUM_TYPE: i32 = 5;
    const SERVICE: i32 = 6;
    const EXTENSION: i32 = 7;
    const OPTIONS: i32 = 8;
    const SOURCE_CODE_INFO: i32 = 9;
    const SYNTAX: i32 = 12;
}
impl FilePath {
    pub(super) fn as_i32(self) -> i32 {
        match self {
            Self::Name => Self::NAME,
            Self::Package => Self::PACKAGE,
            Self::Dependency => Self::DEPENDENCY,
            Self::PublicDependency => Self::PUBLIC_DEPENDENCY,
            Self::WeakDependency => Self::WEAK_DEPENDENCY,
            Self::Message => Self::MESSAGE,
            Self::Enum => Self::ENUM_TYPE,
            Self::Service => Self::SERVICE,
            Self::Extension => Self::EXTENSION,
            Self::Options => Self::OPTIONS,
            Self::SourceCodeInfo => Self::SOURCE_CODE_INFO,
            Self::Syntax => Self::SYNTAX,
            Self::Unknown(value) => value,
        }
    }
    pub(super) fn from_i32(value: i32) -> Self {
        match value {
            Self::NAME => Self::Name,
            Self::PACKAGE => Self::Package,
            Self::DEPENDENCY => Self::Dependency,
            Self::PUBLIC_DEPENDENCY => Self::PublicDependency,
            Self::WEAK_DEPENDENCY => Self::WeakDependency,
            Self::MESSAGE => Self::Message,
            Self::ENUM_TYPE => Self::Enum,
            Self::SERVICE => Self::Service,
            Self::EXTENSION => Self::Extension,
            Self::OPTIONS => Self::Options,
            Self::SOURCE_CODE_INFO => Self::SourceCodeInfo,
            Self::SYNTAX => Self::Syntax,
            _ => Self::Unknown(value),
        }
    }
}

impl From<i32> for FilePath {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl PartialEq<i32> for FilePath {
    fn eq(&self, other: &i32) -> bool {
        self.as_i32() == *other
    }
}
impl PartialEq<FilePath> for i32 {
    fn eq(&self, other: &FilePath) -> bool {
        *other == *self
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub(super) enum MessagePath {
    /// DescriptorProto.field
    Field = 2,
    /// DescriptorProto.nested_type
    Nested = 3,
    /// DescriptorProto.enum_type
    Enum = 4,

    Extension = 6,

    /// DescriptorProto.oneof_decl
    Oneof = 8,

    Unknown(i32),
}
impl MessagePath {
    const FIELD: i32 = 2;
    const NESTED: i32 = 3;
    const ENUM: i32 = 4;
    const EXTENSION: i32 = 6;
    const ONEOF: i32 = 8;
}
impl MessagePath {
    pub(super) fn as_i32(self) -> i32 {
        match self {
            Self::Field => Self::FIELD,
            Self::Nested => Self::NESTED,
            Self::Enum => Self::ENUM,
            Self::Extension => Self::EXTENSION,
            Self::Oneof => Self::ONEOF,
            Self::Unknown(value) => value,
        }
    }
    pub(super) fn from_i32(value: i32) -> Self {
        match value {
            Self::FIELD => Self::Field,
            Self::NESTED => Self::Nested,
            Self::ENUM => Self::Enum,
            Self::EXTENSION => Self::Extension,
            Self::ONEOF => Self::Oneof,
            _ => Self::Unknown(value),
        }
    }
}
impl From<i32> for MessagePath {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}
impl PartialEq<i32> for MessagePath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self
    }
}
impl PartialEq<MessagePath> for i32 {
    fn eq(&self, other: &MessagePath) -> bool {
        *other == *self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EnumPath {
    /// EnumDescriptorProto.Value
    Value = 2,

    Unknown(i32),
}

impl EnumPath {
    const VALUE: i32 = 2;
    pub(super) fn as_i32(self) -> i32 {
        match self {
            Self::Value => Self::VALUE,
            Self::Unknown(value) => value,
        }
    }
    pub(super) fn from_i32(value: i32) -> Self {
        match value {
            Self::VALUE => Self::Value,
            _ => Self::Unknown(value),
        }
    }
}

impl PartialEq<i32> for EnumPath {
    fn eq(&self, other: &i32) -> bool {
        *other == *self
    }
}
impl PartialEq<EnumPath> for i32 {
    fn eq(&self, other: &EnumPath) -> bool {
        *other == *self
    }
}

// Paths for nodes in an ServiceDescriptorProto
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ServicePath {
    /// ServiceDescriptorProto.method
    Method = 2,
    Mixin = 6,

    Unknown(i32),
}
impl ServicePath {
    const METHOD: i32 = 2;
    const MIXIN: i32 = 6;
    pub(super) fn as_i32(self) -> i32 {
        match self {
            Self::Method => Self::METHOD,
            Self::Mixin => Self::MIXIN,
            Self::Unknown(value) => value,
        }
    }
    pub(super) fn from_i32(value: i32) -> Self {
        match value {
            Self::METHOD => Self::Method,
            Self::MIXIN => Self::Mixin,
            _ => Self::Unknown(value),
        }
    }
}
impl PartialEq<i32> for ServicePath {
    fn eq(&self, other: &i32) -> bool {
        *other == self.as_i32()
    }
}
impl PartialEq<ServicePath> for i32 {
    fn eq(&self, other: &ServicePath) -> bool {
        other.as_i32() == *self
    }
}
