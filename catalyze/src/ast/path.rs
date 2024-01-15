pub(super) fn new_file_path() -> Vec<i32> {
    vec![File::SYNTAX, File::PACKAGE]
}
pub(super) fn append(path: &[i32], kind: impl Into<i32>, index: i32) -> Vec<i32> {
    let mut path = path.to_vec();
    path.reserve(2);
    path.push(kind.into());
    path.push(index);
    path
}

pub(super) fn new(kind: impl Into<i32>, index: usize) -> Vec<i32> {
    vec![kind.into(), index as i32]
}

#[derive(Clone, PartialEq)]
#[repr(i32)]
pub enum File {
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

impl File {
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
impl File {
    pub fn as_i32(self) -> i32 {
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
    pub fn from_i32(value: i32) -> Self {
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

impl From<File> for i32 {
    fn from(value: File) -> Self {
        value.as_i32()
    }
}
impl From<i32> for File {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl PartialEq<i32> for File {
    fn eq(&self, other: &i32) -> bool {
        self.as_i32() == *other
    }
}
impl PartialEq<File> for i32 {
    fn eq(&self, other: &File) -> bool {
        *other == *self
    }
}

pub fn append_message(path: &mut Vec<i32>, kind: Message, index: i32) {
    path.reserve(2);
    path.push(kind.as_i32());
    path.push(index);
}

#[derive(Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub enum Message {
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
impl Message {
    pub(super) const FIELD: i32 = 2;
    pub(super) const NESTED: i32 = 3;
    pub(super) const ENUM: i32 = 4;
    pub(super) const EXTENSION: i32 = 6;
    pub(super) const ONEOF: i32 = 8;
}

impl Message {
    pub fn as_i32(self) -> i32 {
        match self {
            Self::Field => Self::FIELD,
            Self::Nested => Self::NESTED,
            Self::Enum => Self::ENUM,
            Self::Extension => Self::EXTENSION,
            Self::Oneof => Self::ONEOF,
            Self::Unknown(value) => value,
        }
    }
    pub fn from_i32(value: i32) -> Self {
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
impl From<Message> for i32 {
    fn from(value: Message) -> Self {
        value.as_i32()
    }
}

impl From<i32> for Message {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}
impl PartialEq<i32> for Message {
    fn eq(&self, other: &i32) -> bool {
        *other == *self
    }
}
impl PartialEq<Message> for i32 {
    fn eq(&self, other: &Message) -> bool {
        *other == *self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Enum {
    /// EnumDescriptorProto.Value
    Value = 2,

    Unknown(i32),
}

impl Enum {
    const VALUE: i32 = 2;
    pub fn as_i32(self) -> i32 {
        match self {
            Self::Value => Self::VALUE,
            Self::Unknown(value) => value,
        }
    }
    pub fn from_i32(value: i32) -> Self {
        match value {
            Self::VALUE => Self::Value,
            _ => Self::Unknown(value),
        }
    }
}
impl From<Enum> for i32 {
    fn from(value: Enum) -> Self {
        value.as_i32()
    }
}
impl PartialEq<i32> for Enum {
    fn eq(&self, other: &i32) -> bool {
        *other == *self
    }
}
impl PartialEq<Enum> for i32 {
    fn eq(&self, other: &Enum) -> bool {
        *other == *self
    }
}

// Paths for nodes in an ServiceDescriptorProto
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Service {
    /// ServiceDescriptorProto.method
    Method = 2,
    Mixin = 6,

    Unknown(i32),
}
impl Service {
    const METHOD: i32 = 2;
    const MIXIN: i32 = 6;
    pub fn as_i32(self) -> i32 {
        match self {
            Self::Method => Self::METHOD,
            Self::Mixin => Self::MIXIN,
            Self::Unknown(value) => value,
        }
    }
    pub fn from_i32(value: i32) -> Self {
        match value {
            Self::METHOD => Self::Method,
            Self::MIXIN => Self::Mixin,
            _ => Self::Unknown(value),
        }
    }
}
impl From<Service> for i32 {
    fn from(value: Service) -> Self {
        value.as_i32()
    }
}
impl PartialEq<i32> for Service {
    fn eq(&self, other: &i32) -> bool {
        *other == self.as_i32()
    }
}
impl PartialEq<Service> for i32 {
    fn eq(&self, other: &Service) -> bool {
        other.as_i32() == *self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Oneof {
    Name = 1,
    Options = 2,
    Unknown(i32),
}
impl Oneof {
    const NAME: i32 = 1;
    const OPTIONS: i32 = 2;
    pub fn as_i32(self) -> i32 {
        match self {
            Self::Name => Self::NAME,
            Self::Options => Self::OPTIONS,
            Self::Unknown(value) => value,
        }
    }
    pub fn from_i32(value: i32) -> Self {
        match value {
            Self::NAME => Self::Name,
            Self::OPTIONS => Self::Options,
            _ => Self::Unknown(value),
        }
    }
}
impl From<Oneof> for i32 {
    fn from(value: Oneof) -> Self {
        value.as_i32()
    }
}
impl PartialEq<i32> for Oneof {
    fn eq(&self, other: &i32) -> bool {
        *other == self.as_i32()
    }
}
impl PartialEq<Oneof> for i32 {
    fn eq(&self, other: &Oneof) -> bool {
        other.as_i32() == *self
    }
}
