use core::fmt;
use std::ops::Deref;

use ahash::HashMap;
use protobuf::{
    descriptor::{self, method_options, MethodOptions as ProtoMethodOpts},
    EnumOrUnknown, SpecialFields,
};

use crate::error::HydrationFailed;

use super::{
    access::{
        AccessComments, AccessFile, AccessFqn, AccessKey, AccessName, AccessNodeKeys,
        AccessPackage, AccessReferences,
    },
    file::FileKey,
    impl_traits_and_methods,
    location::{self, Comments, Span},
    message::{Message, MessageKey},
    node,
    package::PackageKey,
    reference::References,
    resolve::Resolver,
    service::ServiceKey,
    uninterpreted::{into_uninterpreted_options, UninterpretedOption},
    Ast, FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct MethodKey;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PayloadInner {
    Unary(MessageKey),
    Streaming(MessageKey),
}

impl PayloadInner {
    fn new(message: MessageKey, streaming: bool) -> Self {
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
        Self::Unary(MessageKey::default())
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
        input: MessageKey,
        client_streaming: bool,
        output: MessageKey,
        server_streaming: bool,
    ) -> Self {
        Self {
            input: PayloadInner::new(input, client_streaming),
            output: PayloadInner::new(output, server_streaming),
        }
    }
}

pub(super) type MethodIdent = node::Ident<MethodKey>;
pub(super) type MethodTable =
    super::table::Table<MethodKey, MethodInner, HashMap<FullyQualifiedName, MethodKey>>;

#[derive(Debug, Default, Clone, PartialEq)]
pub(super) struct MethodInner {
    pub(super) key: MethodKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) service: ServiceKey,
    pub(super) proto_path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
    pub(super) client_streaming: bool,
    pub(super) server_streaming: bool,
    pub(super) package: Option<PackageKey>,
    pub(super) file: FileKey,
    pub(super) io: IoInner,
    pub(super) input_proto_type: String,
    pub(super) output_proto_type: String,
    pub(super) special_fields: SpecialFields,
    pub(super) options: MethodOptions,
    pub(super) proto_opts: ProtoMethodOpts,
}
impl AccessFqn for MethodInner {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.fqn
    }
}
impl AccessKey for MethodInner {
    type Key = MethodKey;

    fn key(&self) -> Self::Key {
        self.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.key
    }
}
impl AccessNodeKeys for MethodInner {
    fn keys(&self) -> impl Iterator<Item = super::node::NodeKey> {
        std::iter::empty()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MethodOptions {
    pub deprecated: Option<bool>,
    pub idempotency_level: IdempotencyLevel,
    pub uninterpreted_options: Vec<UninterpretedOption>,
}
impl MethodOptions {
    fn hydrate(
        &mut self,
        proto_opts: &mut descriptor::MethodOptions,
    ) -> Result<(), HydrationFailed> {
        self.deprecated = proto_opts.deprecated.take();
        self.idempotency_level = proto_opts
            .idempotency_level
            .take()
            .map(Into::into)
            .unwrap_or_default();
        self.uninterpreted_options = into_uninterpreted_options(&proto_opts.uninterpreted_option);
        Ok(())
    }
}
impl MethodInner {
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<MethodIdent, HydrationFailed> {
        let Hydrate {
            name,
            io,
            location,
            mut options,
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
        self.options.hydrate(&mut options)?;
        self.proto_opts = options;
        self.hydrate_location(location);
        Ok(self.into())
    }
}

pub struct Method<'ast>(pub(super) Resolver<'ast, MethodKey, MethodInner>);
impl AccessName for Method<'_> {
    fn name(&self) -> &str {
        &self.0.name
    }
}
impl AccessKey for Method<'_> {
    type Key = MethodKey;

    fn key(&self) -> Self::Key {
        self.0.key
    }

    fn key_mut(&mut self) -> &mut Self::Key {
        &mut self.0.key
    }
}
impl AccessComments for Method<'_> {
    fn comments(&self) -> Option<&Comments> {
        self.0.comments.as_ref()
    }
}
impl<'ast> AccessPackage<'ast> for Method<'ast> {
    fn package(&self) -> Option<super::package::Package<'ast>> {
        self.0.package.map(|key| (key, self.ast()).into())
    }
}
impl<'ast> AccessFile<'ast> for Method<'ast> {
    fn file(&self) -> super::file::File<'ast> {
        (self.0.file, self.ast()).into()
    }
}
impl AccessFqn for Method<'_> {
    fn fqn(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}

impl<'ast> Method<'ast> {
    pub fn io(self) -> Io<'ast> {
        self.0.io.resolve(self.0.ast)
    }
    pub fn name(&self) -> &str {
        &self.0.name
    }
    pub fn references(&'ast self) -> References<'ast> {
        AccessReferences::references(self)
    }
}

impl<'ast> AccessReferences<'ast> for Method<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        todo!()
    }
}
impl_traits_and_methods!(Method, MethodKey, MethodInner);

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
    pub fn is_no_side_effects(self) -> bool {
        matches!(self, Self::NoSideEffects)
    }

    /// Returns `true` if the idempotency level is [`Idempotent`].
    ///
    /// [`Idempotent`]: IdempotencyLevel::Idempotent
    #[must_use]
    pub fn is_idempotent(self) -> bool {
        matches!(self, Self::Idempotent)
    }

    /// Returns `true` if the idempotency level is [`Unknown`].
    ///
    /// [`Unknown`]: IdempotencyLevel::Unknown
    #[must_use]
    pub fn is_unknown(self) -> bool {
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

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) io: IoInner,
    pub(super) service: ServiceKey,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) location: location::Location,
    pub(super) special_fields: SpecialFields,
    pub(super) options: ProtoMethodOpts,
}
