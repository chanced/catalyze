use core::fmt;
use std::{default, ops::Deref};

use protobuf::{
    descriptor::{method_options, MethodOptions},
    EnumOrUnknown, SpecialFields,
};

use crate::error::HydrationFailed;

use super::{
    access::NodeKeys,
    file, impl_traits_and_methods,
    location::{self, Comments, Span},
    message::{self, Message},
    node, package,
    reference::{self, References},
    resolve::Resolver,
    service,
    uninterpreted::{into_uninterpreted_options, UninterpretedOption},
    Ast, FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PayloadInner {
    Unary(message::Key),
    Streaming(message::Key),
}

impl PayloadInner {
    fn new(message: message::Key, streaming: bool) -> Self {
        if streaming {
            Self::Streaming(message)
        } else {
            Self::Unary(message)
        }
    }
    fn resolve<'ast>(&self, ast: &'ast Ast) -> Payload<'ast> {
        match self {
            Self::Unary(v) => Payload::Unary(Message::new(*v, ast)),
            Self::Streaming(v) => Payload::Streaming(Message::new(*v, ast)),
        }
    }
}

impl Default for PayloadInner {
    fn default() -> Self {
        Self::Unary(message::Key::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payload<'ast> {
    Unary(Message<'ast>),
    Streaming(Message<'ast>),
}

impl<'ast> Payload<'ast> {
    pub fn message(self) -> Message<'ast> {
        match self {
            Self::Unary(v) | Self::Streaming(v) => v,
        }
    }
    pub fn is_streaming(self) -> bool {
        matches!(self, Self::Streaming(..))
    }
    pub fn is_unary(self) -> bool {
        matches!(self, Self::Unary(..))
    }
}
impl<'ast> Deref for Payload<'ast> {
    type Target = Message<'ast>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Unary(v) | Self::Streaming(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Io<'ast> {
    pub input: Payload<'ast>,
    pub output: Payload<'ast>,
}

impl<'ast> Io<'ast> {
    pub(super) fn new(inner: IoInner, ast: &'ast Ast) -> Self {
        inner.resolve(ast)
    }
    pub fn input(self) -> Payload<'ast> {
        self.input
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct IoInner {
    pub(super) input: PayloadInner,
    pub(super) output: PayloadInner,
}

impl IoInner {
    fn resolve<'ast>(&self, ast: &'ast Ast) -> Io<'ast> {
        Io {
            input: self.input.resolve(ast),
            output: self.output.resolve(ast),
        }
    }

    pub(super) fn new(
        input: message::Key,
        client_streaming: bool,
        output: message::Key,
        server_streaming: bool,
    ) -> Self {
        Self {
            input: PayloadInner::new(input, client_streaming),
            output: PayloadInner::new(output, server_streaming),
        }
    }
}

pub(super) type Ident = node::Ident<Key>;
pub(super) type Table = super::table::Table<Key, Inner>;

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Name,
    service: service::Key,
    node_path: Box<[i32]>,
    span: Span,
    comments: Option<Comments>,
    client_streaming: bool,
    server_streaming: bool,
    package: Option<package::Key>,
    file: file::Key,
    uninterpreted_options: Vec<UninterpretedOption>,
    io: IoInner,
    deprecated: bool,
    input_proto_type: String,
    output_proto_type: String,
    idempotency_level: IdempotencyLevel,
    special_fields: SpecialFields,
    option_special_fields: SpecialFields,
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = super::node::Key> {
        std::iter::empty()
    }
}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) io: IoInner,
    pub(super) service: service::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) location: location::Detail,
    pub(super) special_fields: SpecialFields,
    pub(super) options: protobuf::MessageField<MethodOptions>,
}

impl Inner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            io,
            location,
            options,
            service,
            file,
            package,
            special_fields,
        } = hydrate;
        self.name = name;
        self.file = file;
        self.service = service;
        self.package = package;
        self.io = io;
        self.special_fields = special_fields;
        self.hydrate_options(options.unwrap_or_default())?;
        self.hydrate_location(location);
        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: MethodOptions) -> Result<(), HydrationFailed> {
        let MethodOptions {
            deprecated,
            idempotency_level,
            uninterpreted_option,
            special_fields,
        } = opts;
        self.deprecated = deprecated.unwrap_or(false);
        self.option_special_fields = special_fields;
        self.uninterpreted_options = into_uninterpreted_options(uninterpreted_option);
        self.idempotency_level = idempotency_level.unwrap_or_default().into();
        Ok(())
    }
}

pub struct Method<'ast>(Resolver<'ast, Key, Inner>);

impl<'ast> Method<'ast> {
    pub fn io(self) -> Io<'ast> {
        self.0.io.resolve(self.0.ast)
    }
}
impl<'ast> Method<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::References::references(self)
    }
}

impl<'ast> super::access::References<'ast> for Method<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        todo!()
    }
}
impl_traits_and_methods!(Method, Key, Inner);

///  Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
///  or neither? HTTP based RPC implementation may choose GET verb for safe
///  methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IdempotencyLevel {
    IdempotencyUnknown = 0,
    NoSideEffects = 1,
    Idempotent = 2,
    Unknown(i32),
}

impl Default for IdempotencyLevel {
    fn default() -> Self {
        Self::IdempotencyUnknown
    }
}
impl IdempotencyLevel {
    const IDEMPOTENCY_UNKNOWN: &'static str = "IdempotencyUnknown";
    const NO_SIDE_EFFECTS: &'static str = "NoSideEffects";
    const IDEMPOTENT: &'static str = "Idempotent";
    const UNKNOWN: &'static str = "Unknown";

    /// Returns `true` if the idempotency level is [`NoSideEffects`].
    ///
    /// [`NoSideEffects`]: IdempotencyLevel::NoSideEffects
    #[must_use]
    pub fn is_no_side_effects(&self) -> bool {
        matches!(self, Self::NoSideEffects)
    }

    /// Returns `true` if the idempotency level is [`Idempotent`].
    ///
    /// [`Idempotent`]: IdempotencyLevel::Idempotent
    #[must_use]
    pub fn is_idempotent(&self) -> bool {
        matches!(self, Self::Idempotent)
    }

    /// Returns `true` if the idempotency level is [`Unknown`].
    ///
    /// [`Unknown`]: IdempotencyLevel::Unknown
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(..))
    }
}

impl fmt::Display for IdempotencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdempotencyUnknown => write!(f, "{}", Self::IDEMPOTENCY_UNKNOWN),
            Self::NoSideEffects => write!(f, "{}", Self::NO_SIDE_EFFECTS),
            Self::Idempotent => write!(f, "{}", Self::IDEMPOTENT),
            Self::Unknown(v) => write!(f, "{}({})", Self::UNKNOWN, v),
        }
    }
}

impl From<EnumOrUnknown<method_options::IdempotencyLevel>> for IdempotencyLevel {
    fn from(value: EnumOrUnknown<method_options::IdempotencyLevel>) -> Self {
        match value.enum_value() {
            Ok(v) => v.into(),
            Err(v) => Self::Unknown(v),
        }
    }
}

impl From<method_options::IdempotencyLevel> for IdempotencyLevel {
    fn from(value: method_options::IdempotencyLevel) -> Self {
        use method_options::IdempotencyLevel as ProtoIdempotencyLevel;
        match value {
            ProtoIdempotencyLevel::IDEMPOTENCY_UNKNOWN => Self::IdempotencyUnknown,
            ProtoIdempotencyLevel::NO_SIDE_EFFECTS => Self::NoSideEffects,
            ProtoIdempotencyLevel::IDEMPOTENT => Self::Idempotent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Input,
    Output,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
