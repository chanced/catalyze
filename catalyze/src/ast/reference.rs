use std::{iter::Copied, option, slice};

use either::Either;

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

    fn from_inner(inner: Inner, ast: &'ast Ast) -> Self {
        let referrer = Referrer::new(inner.referrer, ast);
        let referent = Referent::new(inner.referent, ast);
        Self { referrer, referent }
    }
}

#[derive(Clone, Default, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Inner {
    /// referring field, extension, or method
    pub(super) referrer: ReferrerKey,
    /// referenced message or enum
    pub(super) referent: ReferentKey,
}

/// The [`Message`] or [`Enum`] which is referenced by the [`Field`],
/// [`Extension`], or [`Method`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(super) enum ReferentKey {
    Message(message::Key),
    Enum(enum_::Key),
}

impl From<enum_::Key> for ReferentKey {
    fn from(v: enum_::Key) -> Self {
        Self::Enum(v)
    }
}

impl From<message::Key> for ReferentKey {
    fn from(v: message::Key) -> Self {
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

impl<'ast> From<(enum_::Key, &'ast Ast)> for Referent<'ast> {
    fn from((key, ast): (enum_::Key, &'ast Ast)) -> Self {
        Self::Enum((key, ast).into())
    }
}
impl<'ast> From<(message::Key, &'ast Ast)> for Referent<'ast> {
    fn from((key, ast): (message::Key, &'ast Ast)) -> Self {
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

    // TODO: (change name, finish updates to method mod)
    // pub fn referent_or_referents(self) -> Referent<'ast> {
    //     match self {
    //         Self::Field(f) => f.referent(),
    //         Self::Extension(e) => e.referent(),
    //         Self::Method(m) => todo!(),
    //     }
    // }

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
    Field(field::Key),
    Extension(extension::Key),
    Method {
        key: method::Key,
        direction: method::Direction,
    },
}
impl Default for ReferrerKey {
    fn default() -> Self {
        Self::Field(field::Key::default())
    }
}
impl Default for ReferentKey {
    fn default() -> Self {
        Self::Message(message::Key::default())
    }
}
impl From<field::Key> for ReferrerKey {
    fn from(key: field::Key) -> Self {
        Self::Field(key)
    }
}
impl From<extension::Key> for ReferrerKey {
    fn from(key: extension::Key) -> Self {
        Self::Extension(key)
    }
}
impl From<(method::Key, method::Direction)> for ReferrerKey {
    fn from((method, direction): (method::Key, method::Direction)) -> Self {
        Self::Method {
            key: method,
            direction,
        }
    }
}

pub struct References<'ast> {
    ast: &'ast Ast,
    inner: Either<Copied<slice::Iter<'ast, Inner>>, option::IntoIter<Inner>>,
}
impl<'ast> References<'ast> {
    pub(crate) fn from_option(opt: Option<Inner>, ast: &'ast Ast) -> Self {
        Self {
            ast,
            inner: Either::Right(opt.into_iter()),
        }
    }
    pub(crate) fn from_slice(slice: &'ast [Inner], ast: &'ast Ast) -> Self {
        Self {
            ast,
            inner: Either::Left(slice.iter().copied()),
        }
    }
}

impl<'ast> Iterator for References<'ast> {
    type Item = Reference<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.inner.next() else {
            return None;
        };
        Some(Reference::from_inner(next, self.ast))
    }
}
