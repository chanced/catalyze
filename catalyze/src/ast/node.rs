use std::fmt;

use crate::HashMap;

use super::{
    enum_::{self, Enum},
    enum_value::{self, EnumValue},
    extension::{self, Extension},
    field::{self, Field},
    file::{self},
    message::{self, Message},
    method::{self, Method},
    oneof::{self, Oneof},
    package::{self},
    service::{self, Service},
    FullyQualifiedName, Name,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Key {
    Package(package::Key),
    File(file::Key),
    Message(message::Key),
    Enum(enum_::Key),
    EnumValue(enum_value::Key),
    Service(service::Key),
    Method(method::Key),
    Field(field::Key),
    Oneof(oneof::Key),
    Extension(extension::Key),
}

pub(super) type Map = HashMap<FullyQualifiedName, Key>;

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
impl From<enum_::Key> for Key {
    fn from(key: enum_::Key) -> Self {
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

pub trait AsNode<'ast>: Into<Node<'ast>> + Copy {
    fn as_node(&self) -> Node<'ast> {
        (*self).into()
    }
}

/// A node's key, fully-qualified name, name, and node path.
pub struct Ident<K> {
    pub(super) key: K,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
}

impl<K> Ident<K>
where
    K: Copy + Into<Key>,
{
    pub(super) fn node_key(&self) -> Key {
        self.key.into()
    }
    pub(super) fn fqn(&self) -> FullyQualifiedName {
        self.fqn.clone()
    }
}
impl<K> Ident<K>
where
    K: Copy + Into<Key>,
{
    pub(super) fn as_node_entry(&self) -> (FullyQualifiedName, Key) {
        (self.fqn.clone(), self.key.into())
    }
}

pub(super) trait IdentIterExt<'iter, K>: Iterator<Item = &'iter Ident<K>>
where
    Self: Sized,
    K: 'static + Copy + Into<Key>,
{
    fn into_entries(self) -> impl Iterator<Item = (FullyQualifiedName, Key)> {
        self.map(Ident::as_node_entry)
    }
}

impl<'iter, I, K> IdentIterExt<'iter, K> for I
where
    I: Iterator<Item = &'iter Ident<K>> + Sized,
    K: 'static + Copy + Into<Key>,
{
}

impl<K> From<&Ident<K>> for (Key, FullyQualifiedName)
where
    K: Copy + Into<Key>,
{
    fn from(value: &Ident<K>) -> Self {
        (value.key.into(), value.fqn.clone())
    }
}

pub(super) trait ExtendNodes
where
    Self: Extend<(FullyQualifiedName, Key)> + Sized,
{
    fn extend_nodes<'iter, K>(&mut self, iter: impl IntoIterator<Item = &'iter Ident<K>>)
    where
        K: 'static + Copy + Into<Key>,
    {
        self.extend(iter.into_iter().into_entries());
    }
}

impl ExtendNodes for HashMap<FullyQualifiedName, Key> {}

macro_rules! ident_from {
    ($($mod:ident,)+) => {
        $(
            impl From<&mut $mod::Inner> for Ident<$mod::Key> {
                fn from(inner: &mut $mod::Inner) -> Self {
                    Self {
                        key: inner.key(),
                        fqn: inner.fqn().clone(),
                        name: inner.name().into(),
                    }
                }
            }
            impl From<&$mod::Inner> for Ident<$mod::Key> {
                fn from(inner: &$mod::Inner) -> Self {
                    Self {
                        key: inner.key(),
                        fqn: inner.fqn().clone(),
                        name: inner.name().into(),
                    }
                }
            }
        )+
    };
}
ident_from!(
    package, file, message, enum_, enum_value, service, method, field, oneof, extension,
);
