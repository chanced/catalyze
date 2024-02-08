use std::fmt;

use crate::HashMap;

use super::{
    access::{AccessFqn, AccessName},
    enum_::{self, Enum, EnumInner, EnumKey},
    enum_value::{self, EnumValue, EnumValueInner, EnumValueKey},
    extension::{self, Extension, ExtensionInner, ExtensionKey},
    field::{self, Field, FieldInner, FieldKey},
    file::{self, FileInner, FileKey},
    message::{self, Message, MessageInner, MessageKey},
    method::{self, Method, MethodInner, MethodKey},
    oneof::{self, Oneof, OneofInner, OneofKey},
    package::{self, PackageInner, PackageKey},
    service::{self, Service, ServiceInner, ServiceKey},
    FullyQualifiedName, Name,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NodeKey {
    Package(package::PackageKey),
    File(file::FileKey),
    Message(message::MessageKey),
    Enum(enum_::EnumKey),
    EnumValue(enum_value::EnumValueKey),
    Service(service::ServiceKey),
    Method(method::MethodKey),
    Field(field::FieldKey),
    Oneof(oneof::OneofKey),
    Extension(extension::ExtensionKey),
}
macro_rules! impl_from {
    ($($var:ident @ $key:ident,)+) => {
        $(
            impl From<$key> for NodeKey {
                fn from(key: $key) -> Self {
                    Self::$var(key)
                }
            }
        )+
    };
}

impl_from!(
    Package @ PackageKey,
    File @ FileKey,
    Message @ MessageKey,
    Enum @ EnumKey,
    EnumValue @ EnumValueKey,
    Service @ ServiceKey,
    Method @ MethodKey,
    Field @ FieldKey,
    Oneof @ OneofKey,
    Extension @ ExtensionKey,
);
pub(super) type NodeMap = HashMap<FullyQualifiedName, NodeKey>;

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
macro_rules! impl_from {
    ($($variant:ident,)+) => {
        $(
            impl<'ast> From<$variant<'ast>> for Node<'ast> {
                fn from(node: $variant<'ast>) -> Self {
                    Self::$variant(node)
                }
            }
            impl<'ast> From<&$variant<'ast>> for Node<'ast> {
                fn from(node: &$variant<'ast>) -> Self {
                    Self::$variant(*node)
                }
            }
        )+
    };
}

impl_from!(
    Message, Oneof, Enum, EnumValue, Service, Method, Field, Extension,
);

macro_rules! delegate {
    ($self:ident, $method: ident) => {
        match $self {
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
impl<'ast> AccessFqn for Node<'ast> {
    fn fqn(&self) -> &FullyQualifiedName {
        delegate!(self, fqn)
    }
}
impl<'ast> AccessName for Node<'ast> {
    fn name(&self) -> &str {
        delegate!(self, name)
    }
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

#[derive(Debug, Clone)]
/// A node's key, fully-qualified name, and local name.
pub struct Ident<K> {
    pub(super) key: K,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
}

impl<K> Ident<K>
where
    K: Copy + Into<NodeKey>,
{
    pub(super) fn node_key(&self) -> NodeKey {
        self.key.into()
    }
    pub(super) fn fqn(&self) -> FullyQualifiedName {
        self.fqn.clone()
    }
}
impl<K> Ident<K>
where
    K: Copy + Into<NodeKey>,
{
    pub(super) fn as_node_entry(&self) -> (FullyQualifiedName, NodeKey) {
        (self.fqn.clone(), self.key.into())
    }
}

pub(super) trait IdentIterExt<'iter, K>: Iterator<Item = &'iter Ident<K>>
where
    Self: Sized,
    K: 'static + Copy + Into<NodeKey>,
{
    fn into_entries(self) -> impl Iterator<Item = (FullyQualifiedName, NodeKey)> {
        self.map(Ident::as_node_entry)
    }
}

impl<'iter, I, K> IdentIterExt<'iter, K> for I
where
    I: Iterator<Item = &'iter Ident<K>> + Sized,
    K: 'static + Copy + Into<NodeKey>,
{
}

impl<K> From<&Ident<K>> for (NodeKey, FullyQualifiedName)
where
    K: Copy + Into<NodeKey>,
{
    fn from(value: &Ident<K>) -> Self {
        (value.key.into(), value.fqn.clone())
    }
}

pub(super) trait ExtendNodes
where
    Self: Extend<(FullyQualifiedName, NodeKey)> + Sized,
{
    fn extend_nodes<'iter, K>(&mut self, iter: impl IntoIterator<Item = &'iter Ident<K>>)
    where
        K: 'static + Copy + Into<NodeKey>,
    {
        self.extend(iter.into_iter().into_entries());
    }
}

impl ExtendNodes for HashMap<FullyQualifiedName, NodeKey> {}

macro_rules! ident_from {
    ($($inner:ident @ $key:ident,)+) => {
        $(
            impl From<&mut $inner> for Ident<$key> {
                fn from(inner: &mut $inner) -> Self {
                    Self {
                        key: inner.key,
                        fqn: inner.fqn.clone(),
                        name: inner.name.as_ref().into(),
                    }
                }
            }
            impl From<&$inner> for Ident<$key> {
                fn from(inner: &$inner) -> Self {
                    Self {
                        key: inner.key,
                        fqn: inner.fqn.clone(),
                        name: inner.name.as_ref().into(),
                    }
                }
            }
        )+
    };
}

ident_from!(
    PackageInner @ PackageKey,
    FileInner @ FileKey,
    MessageInner @ MessageKey,
    EnumInner @ EnumKey,
    EnumValueInner @ EnumValueKey,
    ServiceInner @ ServiceKey,
    MethodInner @ MethodKey,
    FieldInner @ FieldKey,
    OneofInner @ OneofKey,
    ExtensionInner @ ExtensionKey,
);
