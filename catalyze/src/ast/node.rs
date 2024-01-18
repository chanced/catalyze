use std::fmt;

use super::{
    r#enum::{self, Enum},
    enum_value::{self, EnumValue},
    extension::{self, Extension},
    field::{self, Field},
    file::{self, File},
    message::{self, Message},
    method::{self, Method},
    oneof::{self, Oneof},
    package::{self, Package},
    service::{self, Service},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Key {
    Package(package::Key),
    File(file::Key),
    Message(message::Key),
    Enum(r#enum::Key),
    EnumValue(enum_value::Key),
    Service(service::Key),
    Method(method::Key),
    Field(field::Key),
    Oneof(oneof::Key),
    Extension(extension::Key),
}

impl From<package::Key> for Key {
    fn from(key: package::Key) -> Self {
        Self::Package(key)
    }
}
impl From<file::Key> for Key {
    fn from(key: file::Key) -> Self {
        Self::File(key)
    }
}
impl From<message::Key> for Key {
    fn from(key: message::Key) -> Self {
        Self::Message(key)
    }
}
impl From<r#enum::Key> for Key {
    fn from(key: r#enum::Key) -> Self {
        Self::Enum(key)
    }
}
impl From<enum_value::Key> for Key {
    fn from(key: enum_value::Key) -> Self {
        Self::EnumValue(key)
    }
}
impl From<service::Key> for Key {
    fn from(key: service::Key) -> Self {
        Self::Service(key)
    }
}
impl From<method::Key> for Key {
    fn from(key: method::Key) -> Self {
        Self::Method(key)
    }
}
impl From<field::Key> for Key {
    fn from(key: field::Key) -> Self {
        Self::Field(key)
    }
}
impl From<oneof::Key> for Key {
    fn from(key: oneof::Key) -> Self {
        Self::Oneof(key)
    }
}
impl From<extension::Key> for Key {
    fn from(key: extension::Key) -> Self {
        Self::Extension(key)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Node<'ast> {
    Package(Package<'ast>),
    File(File<'ast>),
    Message(Message<'ast>),
    Oneof(Oneof<'ast>),
    Enum(Enum<'ast>),
    EnumValue(EnumValue<'ast>),
    Service(Service<'ast>),
    Method(Method<'ast>),
    Field(Field<'ast>),
    Extension(Extension<'ast>),
}

macro_rules! delegate {
    ($method: ident) => {
        match self {
            Self::Package(n) => n.$method(),
            Self::File(n) => n.$method(),
            Self::Message(n) => n.$method(),
            Self::Oneof(n) => n.$method(),
            Self::Enum(n) => n.$method(),
            Self::EnumValue(n) => n.$method(),
            Self::Service(n) => n.$method(),
            Self::Method(n) => n.$method(),
            Self::Field(n) => n.$method(),
            Self::Extension(n) => n.$method(),
        }
    };
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Package(n) => n.fmt(f),
            Self::File(n) => n.fmt(f),
            Self::Message(n) => n.fmt(f),
            Self::Oneof(n) => n.fmt(f),
            Self::Enum(n) => n.fmt(f),
            Self::EnumValue(n) => n.fmt(f),
            Self::Service(n) => n.fmt(f),
            Self::Method(n) => n.fmt(f),
            Self::Field(n) => n.fmt(f),
            Self::Extension(n) => n.fmt(f),
        }
    }
}
