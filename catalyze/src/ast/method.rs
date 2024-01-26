use core::fmt;

use protobuf::{
    descriptor::{method_options, MethodOptions},
    EnumOrUnknown, SpecialFields,
};

use crate::error::HydrationError;

use super::{
    access::NodeKeys,
    file, impl_traits_and_methods,
    location::{self, Comments, Span},
    message::{self, Message},
    node, package,
    reference::{ReferenceInner, References},
    resolve::Resolver,
    service,
    uninterpreted::{into_uninterpreted_options, UninterpretedOption},
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct Key;
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
    input: message::Key,
    input_proto_type: String,
    deprecated: bool,
    output: message::Key,
    output_proto_type: String,
    references: [ReferenceInner; 2],
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
    pub(super) service: service::Key,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) location: location::Detail,
    pub(super) input_type: Option<String>,
    pub(super) output_type: Option<String>,
    pub(super) client_streaming: Option<bool>,
    pub(super) server_streaming: Option<bool>,
    pub(super) options: protobuf::MessageField<MethodOptions>,
}

impl Inner {
    pub(super) fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
        self.references.iter_mut()
    }

    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationError> {
        let Hydrate {
            name,
            location,
            input_type,
            output_type,
            client_streaming,
            server_streaming,
            options,
            service,
            file,
            package,
        } = hydrate;
        self.name = name;
        self.file = file;
        self.service = service;
        self.package = package;
        self.input_proto_type = input_type.unwrap_or_default();
        self.output_proto_type = output_type.unwrap_or_default();
        self.client_streaming = client_streaming.unwrap_or_default();
        self.server_streaming = server_streaming.unwrap_or_default();
        self.hydrate_options(options.unwrap_or_default())?;
        self.hydrate_location(location);
        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: MethodOptions) -> Result<(), HydrationError> {
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
    pub fn input(self) -> Message<'ast> {
        Message::new(self.0.input, self.0.ast)
    }
}
impl<'ast> Method<'ast> {
    pub fn references(&'ast self) -> References<'ast> {
        super::access::References::references(self)
    }
}

impl<'ast> super::access::References<'ast> for Method<'ast> {
    fn references(&'ast self) -> super::reference::References<'ast> {
        References::from_slice(&self.0.references, self.ast())
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
