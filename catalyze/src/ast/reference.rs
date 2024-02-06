use std::{
    iter::{Copied, Fuse},
    option, slice,
};

use super::{
    enum_::{self, Enum},
    extension::{self, Extension},
    field::{self, Field},
    message::{self, Message},
    method::{self, Method},
    Ast,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reference<'ast> {
    /// The [`Field`], [`Extension`], or [`Method`] which references the
    /// [`Message`] or [`Enum`].
    pub referrer: Referrer<'ast>,
    /// The [`Message`] or [`Enum`] which is referenced by the [`Field`],
    /// [`Extension`], or [`Method`].
    pub referent: Referent<'ast>,
}

impl<'ast> Reference<'ast> {
    /// The [`Field`], [`Extension`], or [`Method`] which references the
    /// [`Message`] or [`Enum`].
    pub fn referrer(self) -> Referrer<'ast> {
        self.referrer
    }
    /// The [`Message`] or [`Enum`] which is referenced by the [`Field`],
    /// [`Extension`], or [`Method`].
    pub fn referent(self) -> Referent<'ast> {
        self.referent
    }

    fn from_inner(inner: ReferenceInner, ast: &'ast Ast) -> Self {
        let referrer = Referrer::new(inner.referrer, ast);
        let referent = Referent::new(inner.referent, ast);
        Self { referrer, referent }
    }
}

#[derive(Clone, Default, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ReferenceInner {
    /// referring field, extension, or method
    pub(super) referrer: ReferrerKey,
    /// referenced message or enum
    pub(super) referent: ReferentKey,
}

/// The [`Message`] or [`Enum`] which is referenced by the [`Field`],
/// [`Extension`], or [`Method`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(super) enum ReferentKey {
    Message(message::MessageKey),
    Enum(enum_::EnumKey),
}

impl From<enum_::EnumKey> for ReferentKey {
    fn from(v: enum_::EnumKey) -> Self {
        Self::Enum(v)
    }
}

impl From<message::MessageKey> for ReferentKey {
    fn from(v: message::MessageKey) -> Self {
        Self::Message(v)
    }
}

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub(super) struct ReferentInner {
    referent: ReferentKey,
    referrer: ReferrerKey,
}

/// The [`Message`] or [`Enum`] which is referenced by the [`Field`],
/// [`Extension`], or [`Method`].
///
/// [`Referent`] is returned from [`Field::referent`], [`Extension::referent`],
/// [`Method::input_referent`], and [`Method::output_referent`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Referent<'ast> {
    Message(Message<'ast>),
    Enum(Enum<'ast>),
}

impl<'ast> Referent<'ast> {
    fn new(key: impl Into<ReferentKey>, ast: &'ast Ast) -> Self {
        match key.into() {
            ReferentKey::Message(key) => Self::Message(Message::new(key, ast)),
            ReferentKey::Enum(key) => Self::Enum(Enum::new(key, ast)),
        }
    }
}

impl<'ast> From<(enum_::EnumKey, &'ast Ast)> for Referent<'ast> {
    fn from((key, ast): (enum_::EnumKey, &'ast Ast)) -> Self {
        Self::Enum((key, ast).into())
    }
}
impl<'ast> From<(message::MessageKey, &'ast Ast)> for Referent<'ast> {
    fn from((key, ast): (message::MessageKey, &'ast Ast)) -> Self {
        Self::Message((key, ast).into())
    }
}

/// The [`Field`], [`Extension`], or [`Method`] which references a [`Message`]
/// or [`Enum`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Referrer<'ast> {
    Field(Field<'ast>),
    Extension(Extension<'ast>),
    Method(Method<'ast>),
}

impl<'ast> Referrer<'ast> {
    pub(super) fn new(key: impl Into<ReferrerKey>, ast: &'ast Ast) -> Self {
        match key.into() {
            ReferrerKey::Field(key) => Self::Field(Field::new(key, ast)),
            ReferrerKey::Extension(key) => Self::Extension(Extension::new(key, ast)),
            ReferrerKey::Method { key, .. } => Self::Method(Method::new(key, ast)),
        }
    }

    /// Returns `true` if the referrer is [`Field`].
    ///
    /// [`Field`]: Referrer::Field
    #[must_use]
    pub fn is_field(&self) -> bool {
        matches!(self, Self::Field(..))
    }

    #[must_use]
    pub fn as_field(&self) -> Option<&Field<'ast>> {
        if let Self::Field(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_field(self) -> Result<Field<'ast>, Self> {
        if let Self::Field(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the referrer is [`Extension`].
    ///
    /// [`Extension`]: Referrer::Extension
    #[must_use]
    pub fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(..))
    }

    #[must_use]
    pub fn as_extension(&self) -> Option<&Extension<'ast>> {
        if let Self::Extension(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_extension(self) -> Result<Extension<'ast>, Self> {
        if let Self::Extension(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the referrer is [`Method`].
    ///
    /// [`Method`]: Referrer::Method
    #[must_use]
    pub fn is_method(&self) -> bool {
        matches!(self, Self::Method(..))
    }

    #[must_use]
    pub fn as_method(&self) -> Option<&Method<'ast>> {
        if let Self::Method(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_method(self) -> Result<Method<'ast>, Self> {
        if let Self::Method(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(super) enum ReferrerKey {
    Field(field::FieldKey),
    Extension(extension::ExtensionKey),
    Method {
        key: method::MethodKey,
        direction: method::Direction,
    },
}
impl Default for ReferrerKey {
    fn default() -> Self {
        Self::Field(field::FieldKey::default())
    }
}
impl Default for ReferentKey {
    fn default() -> Self {
        Self::Message(message::MessageKey::default())
    }
}
impl From<field::FieldKey> for ReferrerKey {
    fn from(key: field::FieldKey) -> Self {
        Self::Field(key)
    }
}
impl From<extension::ExtensionKey> for ReferrerKey {
    fn from(key: extension::ExtensionKey) -> Self {
        Self::Extension(key)
    }
}
impl From<(method::MethodKey, method::Direction)> for ReferrerKey {
    fn from((method, direction): (method::MethodKey, method::Direction)) -> Self {
        Self::Method {
            key: method,
            direction,
        }
    }
}

enum ReferencesInner<'ast> {
    ReferrerKeys {
        keys: Copied<slice::Iter<'ast, ReferrerKey>>,
        referent: ReferentKey,
        ast: &'ast Ast,
    },
    Reference {
        inner: Fuse<option::IntoIter<&'ast ReferenceInner>>,
    },
    Slice {
        inner: slice::Iter<'ast, ReferenceInner>,
    },
}

pub struct References<'ast> {
    ast: &'ast Ast,
    inner: ReferencesInner<'ast>,
}
impl<'ast> References<'ast> {
    pub(super) fn from_option(
        inner: Option<&'ast ReferenceInner>,
        ast: &'ast Ast,
    ) -> References<'ast> {
        Self {
            ast,
            inner: ReferencesInner::Reference {
                inner: inner.into_iter().fuse(),
            },
        }
    }

    pub(super) fn from_ref_slice(
        references: &'ast [ReferenceInner],
        ast: &'ast Ast,
    ) -> References<'ast> {
        Self {
            ast,
            inner: ReferencesInner::Slice {
                inner: references.iter(),
            },
        }
    }
    pub(super) fn from_ref_key_slice(
        keys: &'ast [ReferrerKey],
        referent: ReferentKey,
        ast: &'ast Ast,
    ) -> References<'ast> {
        Self {
            ast,
            inner: ReferencesInner::ReferrerKeys {
                keys: keys.iter().copied(),
                referent,
                ast,
            },
        }
    }
}

impl<'ast> Iterator for References<'ast> {
    type Item = Reference<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
