#![feature(prelude_import)]
//!
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::result_large_err,
    clippy::enum_glob_use,
    clippy::implicit_hasher,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::wildcard_imports,
    clippy::module_inception,
    clippy::struct_excessive_bools,
    clippy::missing_const_for_fn
)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{fmt::Display, ops::DerefMut};
pub mod ast {
    pub mod access {
        use super::{container, node, reference, uninterpreted::UninterpretedOption};
        pub trait References<'ast> {
            fn references(&'ast self) -> reference::References<'ast>;
        }
        pub trait ReferencedBy<'ast> {
            fn referenced_by(&'ast self) -> reference::References<'ast>;
        }
        /// A trait implemented by nodes with parent nodes, providing access to
        /// the [`Container`](super::Container) node.
        pub trait Container<'ast> {
            fn container(self) -> container::Container<'ast>;
        }
        /// A trait implemented by all nodes (except `Package` itself) which
        /// returns the [`Package`](super::package::Package) of the
        /// node, if any.
        pub trait Package<'ast> {
            fn package(self) -> Option<super::package::Package<'ast>>;
        }
        /// A trait implemented by all nodes (except `File` and `Package`) which
        /// returns the containing [`File`](super::file::File).
        pub trait File<'ast> {
            fn file(self) -> super::file::File<'ast>;
        }
        /// A trait implemented by all nodes which returns the name of the node.
        pub trait Name {
            fn name(&self) -> &str;
        }
        /// A trait which returns a slice of
        /// [`UninterpretedOption`](super::UninterpretedOption)s.
        pub trait UninterpretedOptions {
            fn uninterpreted_options(&self) -> &[UninterpretedOption];
        }
        /// A trait implemented by nodes with reserved names and ranges.
        pub trait Reserved {
            fn reserved(&self) -> &super::reserved::Reserved;
            fn reserved_names(&self) -> &[String] {
                &self.reserved().names
            }
            fn reserved_ranges(&self) -> &[super::reserved::ReservedRange] {
                &self.reserved().ranges
            }
        }
        /// A trait implemented by all nodes, returning the
        /// [`FullyQualifiedName`](crate::ast::FullyQualifiedName) of the node.
        pub trait FullyQualifiedName {
            /// Returns the [`FullyQualifiedName`] of the node.
            fn fully_qualified_name(&self) -> &super::FullyQualifiedName;
            /// Alias for `fully_qualified_name` - returns the
            /// [`FullyQualifiedName`] of the node.
            fn fqn(&self) -> &super::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        pub trait NodePath {
            fn node_path(&self) -> &[i32];
        }
        pub trait Comments {
            fn comments(&self) -> Option<&super::location::Comments>;
        }
        pub trait Span {
            fn span(&self) -> super::location::Span;
        }
        pub(crate) trait ReferencesMut {
            fn references_mut(
                &mut self,
            ) -> impl '_ + Iterator<Item = &'_ mut reference::ReferenceInner>;
        }
        pub(super) trait NodeKeys {
            fn keys(&self) -> impl Iterator<Item = node::Key>;
        }
        pub(super) trait Key {
            type Key: slotmap::Key + Copy;
            fn key(&self) -> Self::Key;
            fn key_mut(&mut self) -> &mut Self::Key;
            fn set_key(&mut self, key: Self::Key) {
                *self.key_mut() = key;
            }
        }
    }
    pub mod container {
        use super::{
            file::{self, File},
            message::{self, Message},
        };
        pub(super) enum Key {
            Message(message::Key),
            File(file::Key),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Key::Message(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                    }
                    Key::File(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "File", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<message::Key>;
                let _: ::core::clone::AssertParamIsClone<file::Key>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Key::Message(__self_0), Key::Message(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::File(__self_0), Key::File(__arg1_0)) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<message::Key>;
                let _: ::core::cmp::AssertParamIsEq<file::Key>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (Key::Message(__self_0), Key::Message(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (Key::File(__self_0), Key::File(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (Key::Message(__self_0), Key::Message(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (Key::File(__self_0), Key::File(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    },
                    cmp => cmp,
                }
            }
        }
        impl Default for Key {
            fn default() -> Self {
                Self::File(file::Key::default())
            }
        }
        impl From<message::Key> for Key {
            fn from(key: message::Key) -> Self {
                Self::Message(key)
            }
        }
        impl From<file::Key> for Key {
            fn from(key: file::Key) -> Self {
                Self::File(key)
            }
        }
        pub enum Container<'ast> {
            Message(Message<'ast>),
            File(File<'ast>),
        }
        #[automatically_derived]
        impl<'ast> ::core::fmt::Debug for Container<'ast> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Container::Message(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                    }
                    Container::File(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "File", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Container<'ast> {
            #[inline]
            fn clone(&self) -> Container<'ast> {
                let _: ::core::clone::AssertParamIsClone<Message<'ast>>;
                let _: ::core::clone::AssertParamIsClone<File<'ast>>;
                *self
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::Copy for Container<'ast> {}
        impl<'ast> From<File<'ast>> for Container<'ast> {
            fn from(v: File<'ast>) -> Self {
                Self::File(v)
            }
        }
        impl<'ast> From<Message<'ast>> for Container<'ast> {
            fn from(v: Message<'ast>) -> Self {
                Self::Message(v)
            }
        }
        impl<'ast> Container<'ast> {
            /// Returns `true` if the container is [`Message`].
            ///
            /// [`Message`]: Container::Message
            #[must_use]
            pub const fn is_message(self) -> bool {
                match self {
                    Self::Message(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_message(self) -> Option<Message<'ast>> {
                if let Self::Message(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_message(self) -> Result<Message<'ast>, Self> {
                if let Self::Message(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            /// Returns `true` if the container is [`File`].
            ///
            /// [`File`]: Container::File
            #[must_use]
            pub fn is_file(self) -> bool {
                match self {
                    Self::File(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_file(self) -> Option<File<'ast>> {
                if let Self::File(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_file(self) -> Result<File<'ast>, Self> {
                if let Self::File(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
        }
    }
    pub mod r#enum {
        use super::{container, Hydrated, Set};
        use crate::ast::{
            access::NodeKeys,
            enum_value, file, impl_traits_and_methods, location,
            location::{Comments, Span},
            package,
            reference::ReferrerKey,
            resolve::Resolver,
            uninterpreted::UninterpretedOption,
            FullyQualifiedName,
        };
        use protobuf::{descriptor::EnumOptions, SpecialFields};
        use std::{fmt, str::FromStr};
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub(super) struct Hydrate {
            pub(super) name: Box<str>,
            pub(super) values: Vec<Hydrated<enum_value::Key>>,
            pub(super) location: location::Detail,
            pub(super) options: protobuf::MessageField<EnumOptions>,
            pub(super) special_fields: protobuf::SpecialFields,
            pub(super) reserved_names: Vec<String>,
            pub(super) reserved_ranges:
                Vec<protobuf::descriptor::enum_descriptor_proto::EnumReservedRange>,
            pub(super) container: container::Key,
            pub(super) well_known: Option<WellKnownEnum>,
        }
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            name: Box<str>,
            node_path: Box<[i32]>,
            span: Span,
            comments: Option<Comments>,
            reserved: super::reserved::Reserved,
            package: Option<package::Key>,
            file: file::Key,
            container: container::Key,
            referenced_by: Vec<ReferrerKey>,
            values: Set<super::enum_value::Key>,
            well_known: Option<WellKnownEnum>,
            allow_alias: bool,
            deprecated: bool,
            option_special_fields: SpecialFields,
            uninterpreted_options: Vec<UninterpretedOption>,
            special_fields: SpecialFields,
            options_special_fields: SpecialFields,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "name",
                    "node_path",
                    "span",
                    "comments",
                    "reserved",
                    "package",
                    "file",
                    "container",
                    "referenced_by",
                    "values",
                    "well_known",
                    "allow_alias",
                    "deprecated",
                    "option_special_fields",
                    "uninterpreted_options",
                    "special_fields",
                    "options_special_fields",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.name,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.reserved,
                    &self.package,
                    &self.file,
                    &self.container,
                    &self.referenced_by,
                    &self.values,
                    &self.well_known,
                    &self.allow_alias,
                    &self.deprecated,
                    &self.option_special_fields,
                    &self.uninterpreted_options,
                    &self.special_fields,
                    &&self.options_special_fields,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    reserved: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    container: ::core::default::Default::default(),
                    referenced_by: ::core::default::Default::default(),
                    values: ::core::default::Default::default(),
                    well_known: ::core::default::Default::default(),
                    allow_alias: ::core::default::Default::default(),
                    deprecated: ::core::default::Default::default(),
                    option_special_fields: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    special_fields: ::core::default::Default::default(),
                    options_special_fields: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    name: ::core::clone::Clone::clone(&self.name),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    reserved: ::core::clone::Clone::clone(&self.reserved),
                    package: ::core::clone::Clone::clone(&self.package),
                    file: ::core::clone::Clone::clone(&self.file),
                    container: ::core::clone::Clone::clone(&self.container),
                    referenced_by: ::core::clone::Clone::clone(&self.referenced_by),
                    values: ::core::clone::Clone::clone(&self.values),
                    well_known: ::core::clone::Clone::clone(&self.well_known),
                    allow_alias: ::core::clone::Clone::clone(&self.allow_alias),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    option_special_fields: ::core::clone::Clone::clone(&self.option_special_fields),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    special_fields: ::core::clone::Clone::clone(&self.special_fields),
                    options_special_fields: ::core::clone::Clone::clone(
                        &self.options_special_fields,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.name == other.name
                    && self.node_path == other.node_path
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.reserved == other.reserved
                    && self.package == other.package
                    && self.file == other.file
                    && self.container == other.container
                    && self.referenced_by == other.referenced_by
                    && self.values == other.values
                    && self.well_known == other.well_known
                    && self.allow_alias == other.allow_alias
                    && self.deprecated == other.deprecated
                    && self.option_special_fields == other.option_special_fields
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.special_fields == other.special_fields
                    && self.options_special_fields == other.options_special_fields
            }
        }
        impl Inner {
            pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> Hydrated<Key> {
                let Hydrate {
                    name,
                    values,
                    location,
                    options,
                    reserved_names,
                    reserved_ranges,
                    container: container_key,
                    special_fields,
                    well_known,
                } = hydrate;
                self.values = values.into();
                self.name = name;
                self.set_reserved(reserved_names, reserved_ranges);
                self.container = container_key;
                self.well_known = well_known;
                self.special_fields = special_fields;
                self.hydrate_location(location);
                self.hydrate_options(options.unwrap_or_default());
                (self.key, self.fqn.clone(), self.name.clone())
            }
            fn hydrate_options(&mut self, options: EnumOptions) {
                let EnumOptions {
                    allow_alias,
                    deprecated,
                    uninterpreted_option,
                    special_fields,
                } = options;
                self.allow_alias = allow_alias.unwrap_or(false);
                self.deprecated = deprecated.unwrap_or(false);
                self.set_uninterpreted_options(uninterpreted_option);
                self.option_special_fields = special_fields;
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = super::node::Key> {
                self.values.iter().copied().map(super::node::Key::EnumValue)
            }
        }
        pub struct Enum<'ast>(Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Enum<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Enum<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Enum<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Enum<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Enum<'ast> {}
        impl<'ast> PartialEq for Enum<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Enum<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Enum<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Enum<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Enum<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Enum<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Enum<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Enum<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Enum<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Enum<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl Inner {
            fn set_reserved<R>(&mut self, names: Vec<String>, ranges: Vec<R>)
            where
                R: Into<crate::ast::reserved::ReservedRange>,
            {
                self.reserved = crate::ast::reserved::Reserved {
                    names: names.into(),
                    ranges: ranges.into_iter().map(Into::into).collect(),
                };
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn reserved_names(&self) -> &[String] {
                &self.0.reserved.names
            }
            pub fn reserved_ranges(&self) -> &[crate::ast::reserved::ReservedRange] {
                &self.0.reserved.ranges
            }
            pub fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
        impl<'ast> crate::ast::access::Reserved for Enum<'ast> {
            fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Enum<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Enum<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Enum<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Enum<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Enum<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Enum<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
        pub enum WellKnownEnum {
            /// Whether a field is optional, required, or repeated.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#cardinality>
            FieldCardinality,
            /// Basic field types.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#kind>
            FieldKind,
            /// NullValue is a singleton enumeration to represent the null value
            /// for the Value type union.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#nullvalue>
            NullValue,
            /// The syntax in which a protocol buffer element is defined.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#syntax>
            Syntax,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WellKnownEnum {
            #[inline]
            fn clone(&self) -> WellKnownEnum {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WellKnownEnum {}
        #[automatically_derived]
        impl ::core::fmt::Debug for WellKnownEnum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        WellKnownEnum::FieldCardinality => "FieldCardinality",
                        WellKnownEnum::FieldKind => "FieldKind",
                        WellKnownEnum::NullValue => "NullValue",
                        WellKnownEnum::Syntax => "Syntax",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WellKnownEnum {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WellKnownEnum {
            #[inline]
            fn eq(&self, other: &WellKnownEnum) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for WellKnownEnum {}
        #[automatically_derived]
        impl ::core::cmp::Eq for WellKnownEnum {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl WellKnownEnum {
            const FIELD_CARDINALITY: &'static str = "FieldCardinality";
            const FIELD_KIND: &'static str = "FieldKind";
            const NULL_VALUE: &'static str = "NullValue";
            const SYNTAX: &'static str = "Syntax";
            pub const fn as_str(&self) -> &'static str {
                match self {
                    Self::FieldCardinality => Self::FIELD_CARDINALITY,
                    Self::FieldKind => Self::FIELD_KIND,
                    Self::NullValue => Self::NULL_VALUE,
                    Self::Syntax => Self::SYNTAX,
                }
            }
        }
        impl FromStr for WellKnownEnum {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    Self::FIELD_CARDINALITY => Ok(Self::FieldCardinality),
                    Self::FIELD_KIND => Ok(Self::FieldKind),
                    Self::NULL_VALUE => Ok(Self::NullValue),
                    Self::SYNTAX => Ok(Self::Syntax),
                    _ => Err(()),
                }
            }
        }
        impl fmt::Display for WellKnownEnum {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fmt.write_str(self.as_str())
            }
        }
    }
    pub mod enum_value {
        use super::{access::NodeKeys, file, location, node, package};
        use crate::ast::{
            impl_traits_and_methods, resolve::Resolver, uninterpreted::UninterpretedOption,
            FullyQualifiedName,
        };
        use protobuf::{descriptor::EnumValueOptions, SpecialFields};
        pub struct EnumValue<'ast>(Resolver<'ast, Key, Inner>);
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for EnumValue<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for EnumValue<'ast> {}
        impl<'ast> PartialEq for EnumValue<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for EnumValue<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for EnumValue<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for EnumValue<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> EnumValue<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for EnumValue<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for EnumValue<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for EnumValue<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for EnumValue<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for EnumValue<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for EnumValue<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for EnumValue<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for EnumValue<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for EnumValue<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for EnumValue<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> EnumValue<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
        pub(super) struct Hydrate {
            pub(super) name: Box<str>,
            pub(super) number: i32,
            pub(super) location: location::Detail,
            pub(super) options: protobuf::MessageField<EnumValueOptions>,
            pub(super) special_fields: protobuf::SpecialFields,
            pub(super) r#enum: super::r#enum::Key,
            pub(super) file: file::Key,
            pub(super) package: Option<package::Key>,
        }
        /// [`EnumValue`] inner data.
        pub(super) struct Inner {
            /// enum_value::Key
            key: Key,
            fqn: FullyQualifiedName,
            name: Box<str>,
            node_path: Box<[i32]>,
            number: i32,
            r#enum: super::r#enum::Key,
            file: file::Key,
            package: Option<package::Key>,
            span: location::Span,
            comments: Option<location::Comments>,
            deprecated: bool,
            uninterpreted_options: Vec<UninterpretedOption>,
            special_fields: SpecialFields,
            options_special_fields: SpecialFields,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "name",
                    "node_path",
                    "number",
                    "enum",
                    "file",
                    "package",
                    "span",
                    "comments",
                    "deprecated",
                    "uninterpreted_options",
                    "special_fields",
                    "options_special_fields",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.name,
                    &self.node_path,
                    &self.number,
                    &self.r#enum,
                    &self.file,
                    &self.package,
                    &self.span,
                    &self.comments,
                    &self.deprecated,
                    &self.uninterpreted_options,
                    &self.special_fields,
                    &&self.options_special_fields,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    number: ::core::default::Default::default(),
                    r#enum: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    deprecated: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    special_fields: ::core::default::Default::default(),
                    options_special_fields: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    name: ::core::clone::Clone::clone(&self.name),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    number: ::core::clone::Clone::clone(&self.number),
                    r#enum: ::core::clone::Clone::clone(&self.r#enum),
                    file: ::core::clone::Clone::clone(&self.file),
                    package: ::core::clone::Clone::clone(&self.package),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    special_fields: ::core::clone::Clone::clone(&self.special_fields),
                    options_special_fields: ::core::clone::Clone::clone(
                        &self.options_special_fields,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.name == other.name
                    && self.node_path == other.node_path
                    && self.number == other.number
                    && self.r#enum == other.r#enum
                    && self.file == other.file
                    && self.package == other.package
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.deprecated == other.deprecated
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.special_fields == other.special_fields
                    && self.options_special_fields == other.options_special_fields
            }
        }
        impl Inner {
            pub(crate) fn hydrate(&mut self, hydrate: Hydrate) -> super::Hydrated<Key> {
                let Hydrate {
                    name,
                    number,
                    location,
                    options,
                    special_fields,
                    r#enum,
                    file,
                    package,
                } = hydrate;
                self.name = name;
                self.number = number;
                self.comments = location.comments;
                self.file = file;
                self.span = location.span;
                self.package = package;
                self.special_fields = special_fields;
                self.r#enum = r#enum;
                let opts = options.clone().unwrap();
                self.hydrate_options(options.unwrap_or_default());
                (self.key, self.fqn.clone(), self.name.clone())
            }
            fn hydrate_options(&mut self, options: EnumValueOptions) {
                self.options_special_fields = options.special_fields;
                self.deprecated = options.deprecated.unwrap_or(false);
                self.set_uninterpreted_options(options.uninterpreted_option);
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = node::Key> {
                std::iter::empty()
            }
        }
    }
    pub mod extension {
        pub use super::field::{CType, JsType, Label};
        use super::{
            access::NodeKeys,
            extension_block,
            field::{TypeInner, ValueInner},
            file, location, message, package,
            reference::{ReferenceInner, References},
            resolve,
            uninterpreted::UninterpretedOption,
            FullyQualifiedName,
        };
        use crate::ast::impl_traits_and_methods;
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub(super) struct Inner {
            key: Key,
            name: Box<str>,
            value: ValueInner,
            block: extension_block::Key,
            fqn: FullyQualifiedName,
            node_path: Vec<i32>,
            span: location::Span,
            comments: Option<location::Comments>,
            number: i32,
            label: Option<Label>,
            ///  If type_name is set, this need not be set.  If both this and
            /// type_name  are set, this must be one of TYPE_ENUM,
            /// TYPE_MESSAGE or TYPE_GROUP.
            field_type: TypeInner,
            ///  For message and enum types, this is the name of the type.  If
            /// the name  starts with a '.', it is fully-qualified.
            /// Otherwise, C++-like scoping  rules are used to find
            /// the type (i.e. first the nested types within
            /// this  message are searched, then within the parent, on up to the
            /// root  namespace).
            type_name: Option<String>,
            ///  For extensions, this is the name of the type being extended.
            /// It is  resolved in the same manner as type_name.
            extendee: message::Key,
            ///  For numeric types, contains the original text representation of
            /// the value.  For booleans, "true" or "false".
            ///  For strings, contains the default text contents (not escaped in
            /// any way).  For bytes, contains the C escaped value.
            /// All bytes >= 128 are escaped.  TODO(kenton):
            /// Base-64 encode?
            default_value: Option<String>,
            ///  If set, gives the index of a oneof in the containing type's
            /// oneof_decl  list.  This field is a member of that
            /// oneof.
            oneof_index: Option<i32>,
            ///  JSON name of this field. The value is set by protocol compiler.
            /// If the  user has set a "json_name" option on this
            /// field, that option's value  will be used. Otherwise,
            /// it's deduced from the field's name by converting  it
            /// to camelCase.
            json_name: Option<String>,
            ///  The ctype option instructs the C++ code generator to use a
            /// different  representation of the field than it
            /// normally would.  See the specific  options below.
            /// This option is not yet implemented in the open source
            ///  release -- sorry, we'll try to include it in a future version!
            ctype: Option<CType>,
            ///  The packed option can be enabled for repeated primitive fields
            /// to enable  a more efficient representation on the
            /// wire. Rather than repeatedly  writing the tag and
            /// type for each element, the entire array is encoded
            /// as  a single length-delimited blob. In proto3, only
            /// explicit setting it to  false will avoid using packed encoding.
            packed: bool,
            ///  The jstype option determines the JavaScript type used for
            /// values of the  field.  The option is permitted only
            /// for 64 bit integral and fixed types  (int64, uint64,
            /// sint64, fixed64, sfixed64).  A field with
            /// jstype JS_STRING  is represented as JavaScript string, which
            /// avoids loss of precision that  can happen when a
            /// large value is converted to a floating point
            /// JavaScript.  Specifying JS_NUMBER for the jstype
            /// causes the generated JavaScript code to  use the JavaScript
            /// "number" type.  The behavior of the default option
            /// JS_NORMAL is implementation dependent.
            ///
            ///  This option is an enum to permit additional types to be added,
            /// e.g.  goog.math.Integer.
            jstype: Option<JsType>,
            ///  Should this field be parsed lazily?  Lazy applies only to
            /// message-type  fields.  It means that when the outer
            /// message is initially parsed, the  inner message's
            /// contents will not be parsed but instead stored in
            /// encoded  form.  The inner message will actually be parsed when
            /// it is first accessed.
            ///
            ///  This is only a hint.  Implementations are free to choose
            /// whether to use  eager or lazy parsing regardless of
            /// the value of this option.  However,  setting this
            /// option true suggests that the protocol author believes
            /// that  using lazy parsing on this field is worth the additional
            /// bookkeeping  overhead typically needed to implement it.
            ///
            ///  This option does not affect the public interface of any
            /// generated code;  all method signatures remain the
            /// same.  Furthermore, thread-safety of the  interface
            /// is not affected by this option; const methods remain
            /// safe to  call from multiple threads concurrently, while
            /// non-const methods continue  to require exclusive
            /// access.
            ///
            ///
            ///  Note that implementations may choose not to check required
            /// fields within  a lazy sub-message.  That is, calling
            /// IsInitialized() on the outer message  may return
            /// true even if the inner message has missing
            /// required fields.  This is necessary because otherwise the inner
            /// message would have to be  parsed in order to perform the check,
            /// defeating the purpose of lazy  parsing.  An implementation which
            /// chooses not to check required fields  must be consistent about
            /// it. That is, for any particular sub-message, the
            /// implementation must either *always* check its
            /// required fields, or *never*  check its
            /// required fields, regardless of whether or not the message has
            ///  been parsed.
            lazy: bool,
            ///  Is this field deprecated?
            ///  Depending on the target platform, this can emit Deprecated
            /// annotations  for accessors, or it will be completely
            /// ignored; in the very least, this  is a formalization
            /// for deprecating fields.
            deprecated: bool,
            ///  For Google-internal migration only. Do not use.
            weak: bool,
            ///  The parser stores options it doesn't recognize here. See above.
            uninterpreted_options: Vec<UninterpretedOption>,
            ///  If true, this is a proto3 "optional". When a proto3 field is
            /// optional, it  tracks presence regardless of field
            /// type.
            ///
            ///  When proto3_optional is true, this field must be belong to a
            /// oneof to  signal to old proto3 clients that presence
            /// is tracked for this field. This  oneof is known as a
            /// "synthetic" oneof, and this field must be
            /// its sole  member (each proto3 optional field gets its own
            /// synthetic oneof). Synthetic  oneofs exist in the
            /// descriptor only, and do not generate any API.
            /// Synthetic  oneofs must be ordered after all "real"
            /// oneofs.
            ///
            ///  For message fields, proto3_optional doesn't create any semantic
            /// change,  since non-repeated message fields always
            /// track presence. However it still  indicates the
            /// semantic detail of whether the user wrote "optional"
            /// or not.  This can be useful for round-tripping the .proto
            /// file. For consistency we  give message fields a synthetic oneof
            /// also, even though it is not required  to track presence. This is
            /// especially important because the parser can't  tell if a field
            /// is a message or an enum, so it must always create a
            /// synthetic oneof.
            ///
            ///  Proto2 optional fields do not set this flag, because they
            /// already indicate  optional with `LABEL_OPTIONAL`.
            proto3_optional: Option<bool>,
            package: Option<package::Key>,
            reference: Option<ReferenceInner>,
            file: file::Key,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "name",
                    "value",
                    "block",
                    "fqn",
                    "node_path",
                    "span",
                    "comments",
                    "number",
                    "label",
                    "field_type",
                    "type_name",
                    "extendee",
                    "default_value",
                    "oneof_index",
                    "json_name",
                    "ctype",
                    "packed",
                    "jstype",
                    "lazy",
                    "deprecated",
                    "weak",
                    "uninterpreted_options",
                    "proto3_optional",
                    "package",
                    "reference",
                    "file",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.name,
                    &self.value,
                    &self.block,
                    &self.fqn,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.number,
                    &self.label,
                    &self.field_type,
                    &self.type_name,
                    &self.extendee,
                    &self.default_value,
                    &self.oneof_index,
                    &self.json_name,
                    &self.ctype,
                    &self.packed,
                    &self.jstype,
                    &self.lazy,
                    &self.deprecated,
                    &self.weak,
                    &self.uninterpreted_options,
                    &self.proto3_optional,
                    &self.package,
                    &self.reference,
                    &&self.file,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    value: ::core::default::Default::default(),
                    block: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    number: ::core::default::Default::default(),
                    label: ::core::default::Default::default(),
                    field_type: ::core::default::Default::default(),
                    type_name: ::core::default::Default::default(),
                    extendee: ::core::default::Default::default(),
                    default_value: ::core::default::Default::default(),
                    oneof_index: ::core::default::Default::default(),
                    json_name: ::core::default::Default::default(),
                    ctype: ::core::default::Default::default(),
                    packed: ::core::default::Default::default(),
                    jstype: ::core::default::Default::default(),
                    lazy: ::core::default::Default::default(),
                    deprecated: ::core::default::Default::default(),
                    weak: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    proto3_optional: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    reference: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    name: ::core::clone::Clone::clone(&self.name),
                    value: ::core::clone::Clone::clone(&self.value),
                    block: ::core::clone::Clone::clone(&self.block),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    number: ::core::clone::Clone::clone(&self.number),
                    label: ::core::clone::Clone::clone(&self.label),
                    field_type: ::core::clone::Clone::clone(&self.field_type),
                    type_name: ::core::clone::Clone::clone(&self.type_name),
                    extendee: ::core::clone::Clone::clone(&self.extendee),
                    default_value: ::core::clone::Clone::clone(&self.default_value),
                    oneof_index: ::core::clone::Clone::clone(&self.oneof_index),
                    json_name: ::core::clone::Clone::clone(&self.json_name),
                    ctype: ::core::clone::Clone::clone(&self.ctype),
                    packed: ::core::clone::Clone::clone(&self.packed),
                    jstype: ::core::clone::Clone::clone(&self.jstype),
                    lazy: ::core::clone::Clone::clone(&self.lazy),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    weak: ::core::clone::Clone::clone(&self.weak),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    proto3_optional: ::core::clone::Clone::clone(&self.proto3_optional),
                    package: ::core::clone::Clone::clone(&self.package),
                    reference: ::core::clone::Clone::clone(&self.reference),
                    file: ::core::clone::Clone::clone(&self.file),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.name == other.name
                    && self.value == other.value
                    && self.block == other.block
                    && self.fqn == other.fqn
                    && self.node_path == other.node_path
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.number == other.number
                    && self.label == other.label
                    && self.field_type == other.field_type
                    && self.type_name == other.type_name
                    && self.extendee == other.extendee
                    && self.default_value == other.default_value
                    && self.oneof_index == other.oneof_index
                    && self.json_name == other.json_name
                    && self.ctype == other.ctype
                    && self.packed == other.packed
                    && self.jstype == other.jstype
                    && self.lazy == other.lazy
                    && self.deprecated == other.deprecated
                    && self.weak == other.weak
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.proto3_optional == other.proto3_optional
                    && self.package == other.package
                    && self.reference == other.reference
                    && self.file == other.file
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = super::node::Key> {
                std::iter::empty()
            }
        }
        impl Inner {
            pub(super) fn references_mut(
                &mut self,
            ) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
                self.reference.iter_mut()
            }
        }
        pub struct Extension<'ast>(resolve::Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Extension<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Extension<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Extension<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Extension<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Extension<'ast> {}
        impl<'ast> PartialEq for Extension<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Extension<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Extension<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Extension<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Extension<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Extension<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Extension<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Extension<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Extension<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Extension<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Extension<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Extension<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Extension<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Extension<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Extension<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
        impl<'ast> Extension<'ast> {
            pub fn references(&'ast self) -> References<'ast> {
                super::access::References::references(self)
            }
        }
        impl<'ast> super::access::References<'ast> for Extension<'ast> {
            fn references(&'ast self) -> super::reference::References<'ast> {
                References::from_option(self.0.reference, self.ast())
            }
        }
    }
    pub mod extension_block {
        use super::{file, location, package, resolve};
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub(super) struct Inner {
            key: Key,
            span: location::Span,
            node_path: Box<[i32]>,
            comments: Option<location::Comments>,
            extensions: Vec<Key>,
            file: file::Key,
            package: Option<package::Key>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "span",
                    "node_path",
                    "comments",
                    "extensions",
                    "file",
                    "package",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.span,
                    &self.node_path,
                    &self.comments,
                    &self.extensions,
                    &self.file,
                    &&self.package,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    extensions: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    span: ::core::clone::Clone::clone(&self.span),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    extensions: ::core::clone::Clone::clone(&self.extensions),
                    file: ::core::clone::Clone::clone(&self.file),
                    package: ::core::clone::Clone::clone(&self.package),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.span == other.span
                    && self.node_path == other.node_path
                    && self.comments == other.comments
                    && self.extensions == other.extensions
                    && self.file == other.file
                    && self.package == other.package
            }
        }
        /// A set of [`Extension`] which are defined together in a single
        /// message-like structure.
        ///
        /// ```proto
        /// extend Foo {
        ///    optional int32 bar = 126;
        ///    optional int32 baz = 127;
        /// }
        /// ```
        ///
        /// In the above example, `bar` and `baz` would be included the same
        /// block.
        ///
        /// Note that `ExtensionDecl` is not a [`node`](crate::ast::Node) in the
        /// AST, but rather a construct used to organize the
        /// [`Extension`] as they are defined in the protobuf. As such,
        /// the block does not have a [`FullyQualifiedName`].  It does,
        /// however, have a [`Span`] and possibly [`Comments`].
        pub struct ExtensionDecl<'ast>(resolve::Resolver<'ast, Key, Inner>);
        impl<'ast> ExtensionDecl<'ast> {}
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
    }
    pub mod field {
        use super::{
            access::NodeKeys,
            r#enum::{self, Enum},
            file, location,
            message::{self, Message},
            node, package,
            reference::{ReferenceInner, References},
            resolve::Resolver,
        };
        use crate::{
            ast::{
                impl_traits_and_methods, uninterpreted::UninterpretedOption, Ast,
                FullyQualifiedName,
            },
            error::Error,
        };
        use ::std::vec::Vec;
        use protobuf::{
            descriptor::{field_descriptor_proto, field_options::CType as ProtobufCType},
            EnumOrUnknown,
        };
        use std::fmt;
        pub(super) struct Inner {
            key: Key,
            value: ValueInner,
            fqn: FullyQualifiedName,
            node_path: Box<[i32]>,
            span: location::Span,
            comments: Option<location::Comments>,
            name: Box<str>,
            number: i32,
            label: Option<Label>,
            ///  If type_name is set, this need not be set.  If both this and
            /// type_name  are set, this must be one of TYPE_ENUM,
            /// TYPE_MESSAGE or TYPE_GROUP.
            field_type: TypeInner,
            ///  For message and enum types, this is the name of the type.  If
            /// the name  starts with a '.', it is fully-qualified.
            /// Otherwise, C++-like scoping  rules are used to find
            /// the type (i.e. first the nested types within
            /// this  message are searched, then within the parent, on up to the
            /// root  namespace).
            type_name: Option<String>,
            ///  For extensions, this is the name of the type being extended.
            /// It is  resolved in the same manner as type_name.
            extendee: Option<String>,
            ///  For numeric types, contains the original text representation of
            /// the value.  For booleans, "true" or "false".
            ///  For strings, contains the default text contents (not escaped in
            /// any way).  For bytes, contains the C escaped value.
            /// All bytes >= 128 are escaped.  TODO(kenton):
            /// Base-64 encode?
            default_value: Option<String>,
            ///  If set, gives the index of a oneof in the containing type's
            /// oneof_decl  list.  This field is a member of that
            /// oneof.
            oneof_index: Option<i32>,
            ///  JSON name of this field. The value is set by protocol compiler.
            /// If the  user has set a "json_name" option on this
            /// field, that option's value  will be used. Otherwise,
            /// it's deduced from the field's name by converting  it
            /// to camelCase.
            json_name: Option<String>,
            ///  The ctype option instructs the C++ code generator to use a
            /// different  representation of the field than it
            /// normally would.  See the specific  options below.
            /// This option is not yet implemented in the open source
            ///  release -- sorry, we'll try to include it in a future version!
            ctype: Option<CType>,
            ///  The packed option can be enabled for repeated primitive fields
            /// to enable  a more efficient representation on the
            /// wire. Rather than repeatedly  writing the tag and
            /// type for each element, the entire array is encoded
            /// as  a single length-delimited blob. In proto3, only
            /// explicit setting it to  false will avoid using packed encoding.
            packed: bool,
            ///  The jstype option determines the JavaScript type used for
            /// values of the  field.  The option is permitted only
            /// for 64 bit integral and fixed types  (int64, uint64,
            /// sint64, fixed64, sfixed64).  A field with
            /// jstype JS_STRING  is represented as JavaScript string, which
            /// avoids loss of precision that  can happen when a
            /// large value is converted to a floating point
            /// JavaScript.  Specifying JS_NUMBER for the jstype
            /// causes the generated JavaScript code to  use the JavaScript
            /// "number" type.  The behavior of the default option
            /// JS_NORMAL is implementation dependent.
            ///
            ///  This option is an enum to permit additional types to be added,
            /// e.g.  goog.math.Integer.
            jstype: Option<JsType>,
            ///  Should this field be parsed lazily?  Lazy applies only to
            /// message-type  fields.  It means that when the outer
            /// message is initially parsed, the  inner message's
            /// contents will not be parsed but instead stored in
            /// encoded  form.  The inner message will actually be parsed when
            /// it is first accessed.
            ///
            ///  This is only a hint.  Implementations are free to choose
            /// whether to use  eager or lazy parsing regardless of
            /// the value of this option.  However,  setting this
            /// option true suggests that the protocol author believes
            /// that  using lazy parsing on this field is worth the additional
            /// bookkeeping  overhead typically needed to implement it.
            ///
            ///  This option does not affect the public interface of any
            /// generated code;  all method signatures remain the
            /// same.  Furthermore, thread-safety of the  interface
            /// is not affected by this option; const methods remain
            /// safe to  call from multiple threads concurrently, while
            /// non-const methods continue  to require exclusive
            /// access.
            ///
            ///
            ///  Note that implementations may choose not to check required
            /// fields within  a lazy sub-message.  That is, calling
            /// IsInitialized() on the outer message  may return
            /// true even if the inner message has missing
            /// required fields.  This is necessary because otherwise the inner
            /// message would have to be  parsed in order to perform the check,
            /// defeating the purpose of lazy  parsing.  An implementation which
            /// chooses not to check required fields  must be consistent about
            /// it. That is, for any particular sub-message, the
            /// implementation must either *always* check its
            /// required fields, or *never*  check its
            /// required fields, regardless of whether or not the message has
            ///  been parsed.
            lazy: bool,
            ///  Is this field deprecated?
            ///  Depending on the target platform, this can emit Deprecated
            /// annotations  for accessors, or it will be completely
            /// ignored; in the very least, this  is a formalization
            /// for deprecating fields.
            deprecated: bool,
            ///  For Google-internal migration only. Do not use.
            weak: bool,
            ///  The parser stores options it doesn't recognize here. See above.
            uninterpreted_options: Vec<UninterpretedOption>,
            ///  If true, this is a proto3 "optional". When a proto3 field is
            /// optional, it  tracks presence regardless of field
            /// type.
            ///
            ///  When proto3_optional is true, this field must be belong to a
            /// oneof to  signal to old proto3 clients that presence
            /// is tracked for this field. This  oneof is known as a
            /// "synthetic" oneof, and this field must be
            /// its sole  member (each proto3 optional field gets its own
            /// synthetic oneof). Synthetic  oneofs exist in the
            /// descriptor only, and do not generate any API.
            /// Synthetic  oneofs must be ordered after all "real"
            /// oneofs.
            ///
            ///  For message fields, proto3_optional doesn't create any semantic
            /// change,  since non-repeated message fields always
            /// track presence. However it still  indicates the
            /// semantic detail of whether the user wrote "optional"
            /// or not.  This can be useful for round-tripping the .proto
            /// file. For consistency we  give message fields a synthetic oneof
            /// also, even though it is not required  to track presence. This is
            /// especially important because the parser can't  tell if a field
            /// is a message or an enum, so it must always create a
            /// synthetic oneof.
            ///
            ///  Proto2 optional fields do not set this flag, because they
            /// already indicate  optional with `LABEL_OPTIONAL`.
            proto3_optional: Option<bool>,
            package: Option<package::Key>,
            reference: Option<ReferenceInner>,
            file: file::Key,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "value",
                    "fqn",
                    "node_path",
                    "span",
                    "comments",
                    "name",
                    "number",
                    "label",
                    "field_type",
                    "type_name",
                    "extendee",
                    "default_value",
                    "oneof_index",
                    "json_name",
                    "ctype",
                    "packed",
                    "jstype",
                    "lazy",
                    "deprecated",
                    "weak",
                    "uninterpreted_options",
                    "proto3_optional",
                    "package",
                    "reference",
                    "file",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.value,
                    &self.fqn,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.name,
                    &self.number,
                    &self.label,
                    &self.field_type,
                    &self.type_name,
                    &self.extendee,
                    &self.default_value,
                    &self.oneof_index,
                    &self.json_name,
                    &self.ctype,
                    &self.packed,
                    &self.jstype,
                    &self.lazy,
                    &self.deprecated,
                    &self.weak,
                    &self.uninterpreted_options,
                    &self.proto3_optional,
                    &self.package,
                    &self.reference,
                    &&self.file,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    value: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    number: ::core::default::Default::default(),
                    label: ::core::default::Default::default(),
                    field_type: ::core::default::Default::default(),
                    type_name: ::core::default::Default::default(),
                    extendee: ::core::default::Default::default(),
                    default_value: ::core::default::Default::default(),
                    oneof_index: ::core::default::Default::default(),
                    json_name: ::core::default::Default::default(),
                    ctype: ::core::default::Default::default(),
                    packed: ::core::default::Default::default(),
                    jstype: ::core::default::Default::default(),
                    lazy: ::core::default::Default::default(),
                    deprecated: ::core::default::Default::default(),
                    weak: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    proto3_optional: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    reference: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    value: ::core::clone::Clone::clone(&self.value),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    name: ::core::clone::Clone::clone(&self.name),
                    number: ::core::clone::Clone::clone(&self.number),
                    label: ::core::clone::Clone::clone(&self.label),
                    field_type: ::core::clone::Clone::clone(&self.field_type),
                    type_name: ::core::clone::Clone::clone(&self.type_name),
                    extendee: ::core::clone::Clone::clone(&self.extendee),
                    default_value: ::core::clone::Clone::clone(&self.default_value),
                    oneof_index: ::core::clone::Clone::clone(&self.oneof_index),
                    json_name: ::core::clone::Clone::clone(&self.json_name),
                    ctype: ::core::clone::Clone::clone(&self.ctype),
                    packed: ::core::clone::Clone::clone(&self.packed),
                    jstype: ::core::clone::Clone::clone(&self.jstype),
                    lazy: ::core::clone::Clone::clone(&self.lazy),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    weak: ::core::clone::Clone::clone(&self.weak),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    proto3_optional: ::core::clone::Clone::clone(&self.proto3_optional),
                    package: ::core::clone::Clone::clone(&self.package),
                    reference: ::core::clone::Clone::clone(&self.reference),
                    file: ::core::clone::Clone::clone(&self.file),
                }
            }
        }
        impl Inner {
            pub(super) fn references_mut(
                &mut self,
            ) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
                self.reference.iter_mut()
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = node::Key> {
                std::iter::empty()
            }
        }
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        #[repr(i32)]
        pub enum Label {
            Required = 1,
            Optional = 2,
            Repeated = 3,
            Unkown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Label {
            #[inline]
            fn clone(&self) -> Label {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Label {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Label {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Label::Required => ::core::fmt::Formatter::write_str(f, "Required"),
                    Label::Optional => ::core::fmt::Formatter::write_str(f, "Optional"),
                    Label::Repeated => ::core::fmt::Formatter::write_str(f, "Repeated"),
                    Label::Unkown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unkown", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Label {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Label {
            #[inline]
            fn eq(&self, other: &Label) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Label::Unkown(__self_0), Label::Unkown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Label {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Label {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Label {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state);
                match self {
                    Label::Unkown(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    _ => {}
                }
            }
        }
        #[repr(i32)]
        pub enum CType {
            /// Default mode.
            String = 0,
            Cord = 1,
            StringPiece = 2,
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CType {
            #[inline]
            fn clone(&self) -> CType {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CType {}
        #[automatically_derived]
        impl ::core::fmt::Debug for CType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CType::String => ::core::fmt::Formatter::write_str(f, "String"),
                    CType::Cord => ::core::fmt::Formatter::write_str(f, "Cord"),
                    CType::StringPiece => ::core::fmt::Formatter::write_str(f, "StringPiece"),
                    CType::Unknown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CType {
            #[inline]
            fn eq(&self, other: &CType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (CType::Unknown(__self_0), CType::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for CType {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for CType {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state);
                match self {
                    CType::Unknown(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for CType {
            #[inline]
            fn partial_cmp(&self, other: &CType) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (CType::Unknown(__self_0), CType::Unknown(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for CType {
            #[inline]
            fn cmp(&self, other: &CType) -> ::core::cmp::Ordering {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (CType::Unknown(__self_0), CType::Unknown(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        impl From<EnumOrUnknown<ProtobufCType>> for CType {
            fn from(value: EnumOrUnknown<ProtobufCType>) -> Self {
                match value.enum_value() {
                    Ok(v) => v.into(),
                    Err(v) => Self::Unknown(v),
                }
            }
        }
        impl From<&ProtobufCType> for CType {
            fn from(value: &ProtobufCType) -> Self {
                match value {
                    ProtobufCType::STRING => Self::String,
                    ProtobufCType::CORD => Self::Cord,
                    ProtobufCType::STRING_PIECE => Self::StringPiece,
                }
            }
        }
        impl From<ProtobufCType> for CType {
            fn from(value: ProtobufCType) -> Self {
                Self::from(&value)
            }
        }
        pub enum Scalar {
            Double = 1,
            Float = 2,
            /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use
            /// TYPE_SINT64 if negative values are likely.
            Int64 = 3,
            Uint64 = 4,
            /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use
            /// TYPE_SINT32 if negative values are likely.
            Int32 = 5,
            Fixed64 = 6,
            Fixed32 = 7,
            Bool = 8,
            String = 9,
            /// New in version 2.
            Bytes = 12,
            Uint32 = 13,
            Enum = 14,
            Sfixed32 = 15,
            Sfixed64 = 16,
            /// Uses ZigZag encoding.
            Sint32 = 17,
            /// Uses ZigZag encoding.
            Sint64 = 18,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Scalar {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Scalar::Double => "Double",
                        Scalar::Float => "Float",
                        Scalar::Int64 => "Int64",
                        Scalar::Uint64 => "Uint64",
                        Scalar::Int32 => "Int32",
                        Scalar::Fixed64 => "Fixed64",
                        Scalar::Fixed32 => "Fixed32",
                        Scalar::Bool => "Bool",
                        Scalar::String => "String",
                        Scalar::Bytes => "Bytes",
                        Scalar::Uint32 => "Uint32",
                        Scalar::Enum => "Enum",
                        Scalar::Sfixed32 => "Sfixed32",
                        Scalar::Sfixed64 => "Sfixed64",
                        Scalar::Sint32 => "Sint32",
                        Scalar::Sint64 => "Sint64",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Scalar {
            #[inline]
            fn clone(&self) -> Scalar {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Scalar {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Scalar {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Scalar {
            #[inline]
            fn eq(&self, other: &Scalar) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Scalar {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Scalar {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Scalar {
            #[inline]
            fn partial_cmp(&self, other: &Scalar) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Scalar {
            #[inline]
            fn cmp(&self, other: &Scalar) -> ::core::cmp::Ordering {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
            }
        }
        impl fmt::Display for Scalar {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    Self::Double => "double",
                    Self::Float => "float",
                    Self::Int64 => "int64",
                    Self::Uint64 => "uint64",
                    Self::Int32 => "int32",
                    Self::Fixed64 => "fixed64",
                    Self::Fixed32 => "fixed32",
                    Self::Bool => "bool",
                    Self::String => "string",
                    Self::Bytes => "bytes",
                    Self::Uint32 => "uint32",
                    Self::Enum => "enum",
                    Self::Sfixed32 => "sfixed32",
                    Self::Sfixed64 => "sfixed64",
                    Self::Sint32 => "sint32",
                    Self::Sint64 => "sint64",
                };
                f.write_fmt(format_args!("{0}", s))
            }
        }
        pub enum MapKey {
            Int64 = 3,
            Uint64 = 4,
            Int32 = 5,
            Fixed64 = 6,
            Fixed32 = 7,
            String = 9,
            Uint32 = 13,
            Sfixed32 = 15,
            Sfixed64 = 16,
            Sint32 = 17,
            Sint64 = 18,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MapKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        MapKey::Int64 => "Int64",
                        MapKey::Uint64 => "Uint64",
                        MapKey::Int32 => "Int32",
                        MapKey::Fixed64 => "Fixed64",
                        MapKey::Fixed32 => "Fixed32",
                        MapKey::String => "String",
                        MapKey::Uint32 => "Uint32",
                        MapKey::Sfixed32 => "Sfixed32",
                        MapKey::Sfixed64 => "Sfixed64",
                        MapKey::Sint32 => "Sint32",
                        MapKey::Sint64 => "Sint64",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MapKey {
            #[inline]
            fn clone(&self) -> MapKey {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for MapKey {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MapKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MapKey {
            #[inline]
            fn eq(&self, other: &MapKey) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for MapKey {}
        #[automatically_derived]
        impl ::core::cmp::Eq for MapKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for MapKey {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        pub struct Map<'ast> {
            pub key: MapKey,
            pub value: Value<'ast>,
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralPartialEq for Map<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::PartialEq for Map<'ast> {
            #[inline]
            fn eq(&self, other: &Map<'ast>) -> bool {
                self.key == other.key && self.value == other.value
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralEq for Map<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::Eq for Map<'ast> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<MapKey>;
                let _: ::core::cmp::AssertParamIsEq<Value<'ast>>;
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Map<'ast> {
            #[inline]
            fn clone(&self) -> Map<'ast> {
                Map {
                    key: ::core::clone::Clone::clone(&self.key),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        impl fmt::Debug for Map<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Map")
                    .field("key", &self.key)
                    .field("value", &self.value)
                    .finish()
            }
        }
        impl<'ast> Map<'ast> {
            pub const fn new(key: MapKey, value: Value<'ast>) -> Self {
                Self { key, value }
            }
            pub const fn key(&self) -> MapKey {
                self.key
            }
            pub const fn value(&self) -> &Value<'ast> {
                &self.value
            }
        }
        struct MapInner {
            key: MapKey,
            value: ValueInner,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MapInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "MapInner",
                    "key",
                    &self.key,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MapInner {
            #[inline]
            fn clone(&self) -> MapInner {
                let _: ::core::clone::AssertParamIsClone<MapKey>;
                let _: ::core::clone::AssertParamIsClone<ValueInner>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for MapInner {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MapInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MapInner {
            #[inline]
            fn eq(&self, other: &MapInner) -> bool {
                self.key == other.key && self.value == other.value
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for MapInner {}
        #[automatically_derived]
        impl ::core::cmp::Eq for MapInner {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<MapKey>;
                let _: ::core::cmp::AssertParamIsEq<ValueInner>;
            }
        }
        pub enum Type<'ast> {
            Single(Value<'ast>),
            Repeated(Value<'ast>),
            Map(Map<'ast>),
            Unknown(i32),
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Type<'ast> {
            #[inline]
            fn clone(&self) -> Type<'ast> {
                match self {
                    Type::Single(__self_0) => Type::Single(::core::clone::Clone::clone(__self_0)),
                    Type::Repeated(__self_0) => {
                        Type::Repeated(::core::clone::Clone::clone(__self_0))
                    }
                    Type::Map(__self_0) => Type::Map(::core::clone::Clone::clone(__self_0)),
                    Type::Unknown(__self_0) => Type::Unknown(::core::clone::Clone::clone(__self_0)),
                }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::fmt::Debug for Type<'ast> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Type::Single(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Single", &__self_0)
                    }
                    Type::Repeated(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Repeated", &__self_0)
                    }
                    Type::Map(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Map", &__self_0)
                    }
                    Type::Unknown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                    }
                }
            }
        }
        pub(super) enum TypeInner {
            Single(ValueInner),
            Repeated(ValueInner),
            Map(MapInner),
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TypeInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    TypeInner::Single(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Single", &__self_0)
                    }
                    TypeInner::Repeated(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Repeated", &__self_0)
                    }
                    TypeInner::Map(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Map", &__self_0)
                    }
                    TypeInner::Unknown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TypeInner {
            #[inline]
            fn clone(&self) -> TypeInner {
                let _: ::core::clone::AssertParamIsClone<ValueInner>;
                let _: ::core::clone::AssertParamIsClone<MapInner>;
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TypeInner {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TypeInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TypeInner {
            #[inline]
            fn eq(&self, other: &TypeInner) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (TypeInner::Single(__self_0), TypeInner::Single(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (TypeInner::Repeated(__self_0), TypeInner::Repeated(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (TypeInner::Map(__self_0), TypeInner::Map(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (TypeInner::Unknown(__self_0), TypeInner::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for TypeInner {}
        #[automatically_derived]
        impl ::core::cmp::Eq for TypeInner {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ValueInner>;
                let _: ::core::cmp::AssertParamIsEq<MapInner>;
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        impl Default for TypeInner {
            fn default() -> Self {
                Self::Unknown(0)
            }
        }
        pub(super) enum ValueInner {
            Scalar(Scalar),
            Enum(r#enum::Key),
            Message(message::Key),
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ValueInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ValueInner::Scalar(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Scalar", &__self_0)
                    }
                    ValueInner::Enum(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Enum", &__self_0)
                    }
                    ValueInner::Message(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                    }
                    ValueInner::Unknown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValueInner {
            #[inline]
            fn clone(&self) -> ValueInner {
                let _: ::core::clone::AssertParamIsClone<Scalar>;
                let _: ::core::clone::AssertParamIsClone<r#enum::Key>;
                let _: ::core::clone::AssertParamIsClone<message::Key>;
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ValueInner {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValueInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValueInner {
            #[inline]
            fn eq(&self, other: &ValueInner) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (ValueInner::Scalar(__self_0), ValueInner::Scalar(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (ValueInner::Enum(__self_0), ValueInner::Enum(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (ValueInner::Message(__self_0), ValueInner::Message(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (ValueInner::Unknown(__self_0), ValueInner::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ValueInner {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ValueInner {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Scalar>;
                let _: ::core::cmp::AssertParamIsEq<r#enum::Key>;
                let _: ::core::cmp::AssertParamIsEq<message::Key>;
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        impl Default for ValueInner {
            fn default() -> Self {
                Self::Unknown(0)
            }
        }
        impl ValueInner {
            fn resolve_with<'ast>(&self, ast: &'ast Ast) -> Value<'ast> {
                match *self {
                    Self::Scalar(s) => Value::Scalar(s),
                    Self::Enum(key) => (key, ast).into(),
                    Self::Message(key) => (key, ast).into(),
                    Self::Unknown(u) => Value::Unknown(u),
                }
            }
        }
        pub enum Value<'ast> {
            Scalar(Scalar),
            Enum(Enum<'ast>),
            Message(Message<'ast>),
            Unknown(i32),
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralPartialEq for Value<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::PartialEq for Value<'ast> {
            #[inline]
            fn eq(&self, other: &Value<'ast>) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Value::Scalar(__self_0), Value::Scalar(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Value::Enum(__self_0), Value::Enum(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Value::Message(__self_0), Value::Message(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Value::Unknown(__self_0), Value::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralEq for Value<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::Eq for Value<'ast> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Scalar>;
                let _: ::core::cmp::AssertParamIsEq<Enum<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Message<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Value<'ast> {
            #[inline]
            fn clone(&self) -> Value<'ast> {
                let _: ::core::clone::AssertParamIsClone<Scalar>;
                let _: ::core::clone::AssertParamIsClone<Enum<'ast>>;
                let _: ::core::clone::AssertParamIsClone<Message<'ast>>;
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::Copy for Value<'ast> {}
        impl<'ast> From<i32> for Value<'ast> {
            fn from(v: i32) -> Self {
                Self::Unknown(v)
            }
        }
        impl<'ast> From<Message<'ast>> for Value<'ast> {
            fn from(v: Message<'ast>) -> Self {
                Self::Message(v)
            }
        }
        impl<'ast> From<Enum<'ast>> for Value<'ast> {
            fn from(v: Enum<'ast>) -> Self {
                Self::Enum(v)
            }
        }
        impl<'ast> From<Scalar> for Value<'ast> {
            fn from(v: Scalar) -> Self {
                Self::Scalar(v)
            }
        }
        impl<'ast> From<(message::Key, &'ast Ast)> for Value<'ast> {
            fn from((key, ast): (message::Key, &'ast Ast)) -> Self {
                Self::from(Message::from((key, ast)))
            }
        }
        impl<'ast> From<(r#enum::Key, &'ast Ast)> for Value<'ast> {
            fn from((key, ast): (r#enum::Key, &'ast Ast)) -> Self {
                Self::from(Enum::from((key, ast)))
            }
        }
        impl<'ast> fmt::Debug for Value<'ast> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Scalar(s) => fmt::Debug::fmt(s, f),
                    Self::Enum(e) => fmt::Debug::fmt(e, f),
                    Self::Message(m) => fmt::Debug::fmt(m, f),
                    Self::Unknown(i) => fmt::Debug::fmt(i, f),
                }
            }
        }
        impl<'ast> Value<'ast> {
            /// Returns `true` if the type is [`Unknown`].
            ///
            /// [`Unknown`]: Type::Unknown
            #[must_use]
            pub const fn is_unknown(&self) -> bool {
                match self {
                    Self::Unknown(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub const fn is_scalar(&self) -> bool {
                match self {
                    Self::Scalar(_) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub const fn is_group(&self) -> bool {
                match self {
                    Self::Unknown(10) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub const fn is_message(&self) -> bool {
                match self {
                    Self::Message(_) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub const fn is_enum(&self) -> bool {
                match self {
                    Self::Enum(_) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub const fn as_enum(&self) -> Option<Enum> {
                if let Self::Enum(v) = self {
                    Some(*v)
                } else {
                    None
                }
            }
            #[must_use]
            pub const fn as_scalar(&self) -> Option<Scalar> {
                if let Self::Scalar(v) = self {
                    Some(*v)
                } else {
                    None
                }
            }
            #[must_use]
            pub const fn as_message(&self) -> Option<Message> {
                if let Self::Message(v) = self {
                    Some(*v)
                } else {
                    None
                }
            }
            #[must_use]
            pub const fn as_unknown(&self) -> Option<i32> {
                if let Self::Unknown(v) = self {
                    Some(*v)
                } else {
                    None
                }
            }
            pub const fn try_into_scalar(self) -> Result<Scalar, Self> {
                if let Self::Scalar(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            pub const fn try_into_enum(self) -> Result<Enum<'ast>, Self> {
                if let Self::Enum(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            pub const fn try_into_message(self) -> Result<Message<'ast>, Self> {
                if let Self::Message(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            pub const fn try_into_unknown(self) -> Result<i32, Self> {
                if let Self::Unknown(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
        }
        impl ValueInner {
            pub(super) fn new(
                typ: field_descriptor_proto::Type,
                enum_: Option<r#enum::Key>,
                msg: Option<message::Key>,
            ) -> Result<Self, Error> {
                use field_descriptor_proto::Type::*;
                match typ {
                    TYPE_DOUBLE => Ok(Self::Scalar(Scalar::Double)),
                    TYPE_FLOAT => Ok(Self::Scalar(Scalar::Float)),
                    TYPE_INT64 => Ok(Self::Scalar(Scalar::Int64)),
                    TYPE_UINT64 => Ok(Self::Scalar(Scalar::Uint64)),
                    TYPE_INT32 => Ok(Self::Scalar(Scalar::Int32)),
                    TYPE_FIXED64 => Ok(Self::Scalar(Scalar::Fixed64)),
                    TYPE_FIXED32 => Ok(Self::Scalar(Scalar::Fixed32)),
                    TYPE_BOOL => Ok(Self::Scalar(Scalar::Bool)),
                    TYPE_STRING => Ok(Self::Scalar(Scalar::String)),
                    TYPE_BYTES => Ok(Self::Scalar(Scalar::Bytes)),
                    TYPE_UINT32 => Ok(Self::Scalar(Scalar::Uint32)),
                    TYPE_SFIXED32 => Ok(Self::Scalar(Scalar::Sfixed32)),
                    TYPE_SFIXED64 => Ok(Self::Scalar(Scalar::Sfixed64)),
                    TYPE_SINT32 => Ok(Self::Scalar(Scalar::Sint32)),
                    TYPE_SINT64 => Ok(Self::Scalar(Scalar::Sint64)),
                    TYPE_ENUM => Ok(Self::Enum(enum_.unwrap())),
                    TYPE_MESSAGE => Ok(Self::Message(msg.unwrap())),
                    TYPE_GROUP => Err(Error::GroupNotSupported),
                }
            }
        }
        #[repr(i32)]
        pub enum JsType {
            /// Use the default type.
            Normal = 0,
            /// Use JavaScript strings.
            String = 1,
            /// Use JavaScript numbers.
            Number = 2,
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for JsType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    JsType::Normal => ::core::fmt::Formatter::write_str(f, "Normal"),
                    JsType::String => ::core::fmt::Formatter::write_str(f, "String"),
                    JsType::Number => ::core::fmt::Formatter::write_str(f, "Number"),
                    JsType::Unknown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for JsType {
            #[inline]
            fn clone(&self) -> JsType {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for JsType {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for JsType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for JsType {
            #[inline]
            fn eq(&self, other: &JsType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (JsType::Unknown(__self_0), JsType::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for JsType {}
        #[automatically_derived]
        impl ::core::cmp::Eq for JsType {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for JsType {
            #[inline]
            fn partial_cmp(&self, other: &JsType) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (JsType::Unknown(__self_0), JsType::Unknown(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for JsType {
            #[inline]
            fn cmp(&self, other: &JsType) -> ::core::cmp::Ordering {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (JsType::Unknown(__self_0), JsType::Unknown(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        impl From<EnumOrUnknown<protobuf::descriptor::field_options::JSType>> for JsType {
            fn from(value: EnumOrUnknown<protobuf::descriptor::field_options::JSType>) -> Self {
                match value.enum_value() {
                    Ok(v) => v.into(),
                    Err(v) => Self::Unknown(v),
                }
            }
        }
        impl From<protobuf::descriptor::field_options::JSType> for JsType {
            fn from(value: protobuf::descriptor::field_options::JSType) -> Self {
                use protobuf::descriptor::field_options::JSType::*;
                match value {
                    JS_NORMAL => Self::Normal,
                    JS_STRING => Self::String,
                    JS_NUMBER => Self::Number,
                }
            }
        }
        impl<'ast> Field<'ast> {
            pub fn references(&'ast self) -> References<'ast> {
                super::access::References::references(self)
            }
        }
        impl<'ast> super::access::References<'ast> for Field<'ast> {
            fn references(&'ast self) -> super::reference::References<'ast> {
                References::from_option(self.0.reference, self.ast())
            }
        }
        impl super::access::ReferencesMut for Inner {
            fn references_mut(
                &mut self,
            ) -> impl '_ + Iterator<Item = &'_ mut super::reference::ReferenceInner> {
                self.reference.iter_mut()
            }
        }
        pub struct Field<'ast>(Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Field<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Field<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Field<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Field<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Field<'ast> {}
        impl<'ast> PartialEq for Field<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Field<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Field<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Field<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Field<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Field<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Field<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Field<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Field<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Field<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Field<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Field<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Field<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Field<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Field<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Field<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Field<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Field<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Field<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Field<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Field<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
    }
    pub mod file {
        use super::{
            access::NodeKeys, r#enum, extension, extension_block, impl_traits_and_methods,
            location, message, package, resolve::Resolver, service,
            uninterpreted::UninterpretedOption, FullyQualifiedName, Hydrated, Set,
        };
        use crate::error::Error;
        use ahash::HashMap;
        use protobuf::{
            descriptor::{file_options::OptimizeMode as ProtoOptimizeMode, FileOptions},
            SpecialFields,
        };
        use std::{
            fmt,
            hash::Hash,
            ops::Deref,
            path::{Path, PathBuf},
            str::FromStr,
        };
        #[doc(hidden)]
        #[repr(transparent)]
        pub struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub struct File<'ast>(Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> File<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> File<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> File<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for File<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for File<'ast> {}
        impl<'ast> PartialEq for File<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for File<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for File<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for File<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> File<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for File<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for File<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for File<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for File<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for File<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> File<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for File<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> File<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> crate::ast::access::Comments for File<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> File<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl<'ast> File<'ast> {
            #[must_use]
            pub fn path(&self) -> &Path {
                self.0.path.as_ref()
            }
            #[must_use]
            pub fn is_build_target(&self) -> bool {
                self.0.is_build_target
            }
            #[must_use]
            pub fn syntax(&self) -> Syntax {
                self.0.syntax
            }
            #[must_use]
            pub fn java_multiple_files(&self) -> bool {
                self.0.java_multiple_files
            }
            #[must_use]
            pub fn java_package(&self) -> Option<&str> {
                self.0.java_package.as_deref()
            }
            #[must_use]
            pub fn java_outer_classname(&self) -> Option<&str> {
                self.0.java_outer_classname.as_deref()
            }
            #[must_use]
            pub fn java_generate_equals_and_hash(&self) -> bool {
                self.0.java_generate_equals_and_hash
            }
            #[must_use]
            pub fn java_string_check_utf8(&self) -> bool {
                self.0.java_string_check_utf8
            }
            #[must_use]
            pub fn optimize_for(&self) -> Option<OptimizeMode> {
                self.0.optimize_for
            }
            #[must_use]
            pub fn go_package(&self) -> Option<&str> {
                self.0.go_package.as_deref()
            }
            #[must_use]
            pub fn cc_generic_services(&self) -> bool {
                self.0.cc_generic_services
            }
            #[must_use]
            pub fn java_generic_services(&self) -> bool {
                self.0.java_generic_services
            }
            #[must_use]
            pub fn py_generic_services(&self) -> bool {
                self.0.py_generic_services
            }
            #[must_use]
            pub fn php_generic_services(&self) -> bool {
                self.0.php_generic_services
            }
            ///  Is this file deprecated?
            ///  Depending on the target platform, this can emit Deprecated
            /// annotations  for everything in the file, or it will
            /// be completely ignored; in the very  least, this is a
            /// formalization for deprecating files.
            #[must_use]
            pub fn deprecated(&self) -> bool {
                self.0.deprecated
            }
            ///  Enables the use of arenas for the proto messages in this file.
            /// This applies  only to generated classes for C++.
            #[must_use]
            pub fn cc_enable_arenas(&self) -> bool {
                self.0.cc_enable_arenas
            }
            ///  Sets the objective c class prefix which is prepended to all
            /// objective c  generated classes from this .proto.
            /// There is no default.
            #[must_use]
            pub fn objc_class_prefix(&self) -> Option<&str> {
                self.0.objc_class_prefix.as_deref()
            }
            ///  Namespace for generated classes; defaults to the package.
            #[must_use]
            pub fn csharp_namespace(&self) -> Option<&str> {
                self.0.csharp_namespace.as_deref()
            }
            ///  By default Swift generators will take the proto package and
            /// CamelCase it  replacing '.' with underscore and use
            /// that to prefix the types/symbols  defined. When this
            /// options is provided, they will use this value
            /// instead  to prefix the types/symbols defined.
            #[must_use]
            pub fn swift_prefix(&self) -> Option<&str> {
                self.0.swift_prefix.as_deref()
            }
            ///  Sets the php class prefix which is prepended to all php
            /// generated classes  from this .proto. Default is
            /// empty.
            #[must_use]
            pub fn php_class_prefix(&self) -> Option<&str> {
                self.0.php_class_prefix.as_deref()
            }
            ///  Use this option to change the namespace of php generated
            /// classes. Default  is empty. When this option is
            /// empty, the package name will be used for
            /// determining the namespace.
            #[must_use]
            pub fn php_namespace(&self) -> Option<&str> {
                self.0.php_namespace.as_deref()
            }
            ///  Use this option to change the namespace of php generated
            /// metadata classes.  Default is empty. When this
            /// option is empty, the proto file name will be  used
            /// for determining the namespace.
            #[must_use]
            pub fn php_metadata_namespace(&self) -> Option<&str> {
                self.0.php_metadata_namespace.as_deref()
            }
            ///  Use this option to change the package of ruby generated
            /// classes. Default  is empty. When this option is not
            /// set, the package name will be used for  determining
            /// the ruby package.
            #[must_use]
            pub fn ruby_package(&self) -> Option<&str> {
                self.0.ruby_package.as_deref()
            }
            ///  The parser stores options it doesn't recognize here.
            ///  See the documentation for the "Options" section above.
            #[must_use]
            pub fn uninterpreted_option(&self) -> &[UninterpretedOption] {
                &self.0.uninterpreted_options
            }
        }
        /// Syntax of the proto file. Lorem ipsum dolor sit amet, consectetur
        /// adipiscing elit. Sed non risus. Suspendisse lectus tortor,
        /// dignissim sit amet, adipiscing nec, ultricies sed, dolor.
        pub enum Syntax {
            Proto2,
            Proto3,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Syntax {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Syntax::Proto2 => "Proto2",
                        Syntax::Proto3 => "Proto3",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Syntax {
            #[inline]
            fn clone(&self) -> Syntax {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Syntax {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Syntax {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Syntax {
            #[inline]
            fn eq(&self, other: &Syntax) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Syntax {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Syntax {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Syntax {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        impl Syntax {
            const PROTO2: &'static str = "proto2";
            const PROTO3: &'static str = "proto3";
            #[must_use]
            pub const fn supports_required_prefix(&self) -> bool {
                self.is_proto2()
            }
            #[must_use]
            pub const fn is_proto2(&self) -> bool {
                match self {
                    Self::Proto2 => true,
                    _ => false,
                }
            }
            #[must_use]
            pub const fn is_proto3(&self) -> bool {
                match self {
                    Self::Proto3 => true,
                    _ => false,
                }
            }
            pub const fn as_str(&self) -> &'static str {
                match self {
                    Self::Proto2 => Self::PROTO2,
                    Self::Proto3 => Self::PROTO3,
                }
            }
            pub fn parse(s: &str) -> Result<Self, Error> {
                match s {
                    Self::PROTO2 => Ok(Self::Proto2),
                    Self::PROTO3 => Ok(Self::Proto3),
                    _ => Err(Error::unsupported_syntax(s.to_owned())),
                }
            }
        }
        impl fmt::Display for Syntax {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl FromStr for Syntax {
            type Err = ();
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                match &*v.to_lowercase() {
                    Self::PROTO2 | "" => Ok(Self::Proto2),
                    Self::PROTO3 => Ok(Self::Proto3),
                    _ => Err(()),
                }
            }
        }
        impl Default for Syntax {
            fn default() -> Self {
                Self::Proto2
            }
        }
        impl TryFrom<&str> for Syntax {
            type Error = ();
            fn try_from(v: &str) -> Result<Self, Self::Error> {
                Self::from_str(v)
            }
        }
        impl TryFrom<String> for Syntax {
            type Error = ();
            fn try_from(v: String) -> Result<Self, Self::Error> {
                Self::from_str(&v)
            }
        }
        /// Generated classes can be optimized for speed or code size.
        #[repr(i32)]
        pub enum OptimizeMode {
            /// Generate complete code for parsing, serialization,
            Speed = 1,
            /// etc.
            ///
            /// Use ReflectionOps to implement these methods.
            CodeSize = 2,
            /// Generate code using MessageLite and the lite runtime.
            LiteRuntime = 3,
            /// Unknown optimize mode
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OptimizeMode {
            #[inline]
            fn clone(&self) -> OptimizeMode {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for OptimizeMode {}
        #[automatically_derived]
        impl ::core::fmt::Debug for OptimizeMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    OptimizeMode::Speed => ::core::fmt::Formatter::write_str(f, "Speed"),
                    OptimizeMode::CodeSize => ::core::fmt::Formatter::write_str(f, "CodeSize"),
                    OptimizeMode::LiteRuntime => {
                        ::core::fmt::Formatter::write_str(f, "LiteRuntime")
                    }
                    OptimizeMode::Unknown(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OptimizeMode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OptimizeMode {
            #[inline]
            fn eq(&self, other: &OptimizeMode) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (OptimizeMode::Unknown(__self_0), OptimizeMode::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for OptimizeMode {}
        #[automatically_derived]
        impl ::core::cmp::Eq for OptimizeMode {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for OptimizeMode {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state);
                match self {
                    OptimizeMode::Unknown(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for OptimizeMode {
            #[inline]
            fn partial_cmp(
                &self,
                other: &OptimizeMode,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (OptimizeMode::Unknown(__self_0), OptimizeMode::Unknown(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for OptimizeMode {
            #[inline]
            fn cmp(&self, other: &OptimizeMode) -> ::core::cmp::Ordering {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (OptimizeMode::Unknown(__self_0), OptimizeMode::Unknown(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        impl OptimizeMode {
            /// Returns `true` if the optimize mode is [`Speed`].
            ///
            /// [`Speed`]: OptimizeMode::Speed
            #[must_use]
            pub const fn is_speed(&self) -> bool {
                match self {
                    Self::Speed => true,
                    _ => false,
                }
            }
            /// Returns `true` if the optimize mode is [`CodeSize`].
            ///
            /// [`CodeSize`]: OptimizeMode::CodeSize
            #[must_use]
            pub const fn is_code_size(&self) -> bool {
                match self {
                    Self::CodeSize => true,
                    _ => false,
                }
            }
            /// Returns `true` if the optimize mode is [`LiteRuntime`].
            ///
            /// [`LiteRuntime`]: OptimizeMode::LiteRuntime
            #[must_use]
            pub const fn is_lite_runtime(&self) -> bool {
                match self {
                    Self::LiteRuntime => true,
                    _ => false,
                }
            }
            /// Returns `true` if the optimize mode is [`Unknown`].
            ///
            /// [`Unknown`]: OptimizeMode::Unknown
            #[must_use]
            pub const fn is_unknown(&self) -> bool {
                match self {
                    Self::Unknown(..) => true,
                    _ => false,
                }
            }
        }
        impl From<protobuf::EnumOrUnknown<ProtoOptimizeMode>> for OptimizeMode {
            fn from(value: protobuf::EnumOrUnknown<ProtoOptimizeMode>) -> Self {
                match value.enum_value() {
                    Ok(o) => Self::from(o),
                    Err(i) => Self::Unknown(i),
                }
            }
        }
        impl From<protobuf::descriptor::file_options::OptimizeMode> for OptimizeMode {
            fn from(value: protobuf::descriptor::file_options::OptimizeMode) -> Self {
                match value {
                    ProtoOptimizeMode::SPEED => Self::Speed,
                    ProtoOptimizeMode::CODE_SIZE => Self::CodeSize,
                    ProtoOptimizeMode::LITE_RUNTIME => Self::LiteRuntime,
                }
            }
        }
        pub(super) struct DependentInner {
            pub(super) is_used: bool,
            pub(super) is_public: bool,
            pub(super) is_weak: bool,
            pub(super) dependent: Key,
            pub(super) dependency: Key,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DependentInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "DependentInner",
                    "is_used",
                    &self.is_used,
                    "is_public",
                    &self.is_public,
                    "is_weak",
                    &self.is_weak,
                    "dependent",
                    &self.dependent,
                    "dependency",
                    &&self.dependency,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DependentInner {
            #[inline]
            fn default() -> DependentInner {
                DependentInner {
                    is_used: ::core::default::Default::default(),
                    is_public: ::core::default::Default::default(),
                    is_weak: ::core::default::Default::default(),
                    dependent: ::core::default::Default::default(),
                    dependency: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DependentInner {
            #[inline]
            fn clone(&self) -> DependentInner {
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<Key>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for DependentInner {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DependentInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DependentInner {
            #[inline]
            fn eq(&self, other: &DependentInner) -> bool {
                self.is_used == other.is_used
                    && self.is_public == other.is_public
                    && self.is_weak == other.is_weak
                    && self.dependent == other.dependent
                    && self.dependency == other.dependency
            }
        }
        impl DependentInner {
            pub(super) fn set_is_used(&mut self, is_used: bool) {
                self.is_used = is_used;
            }
        }
        impl From<DependencyInner> for DependentInner {
            fn from(dep: DependencyInner) -> Self {
                Self {
                    is_used: dep.is_used,
                    is_public: dep.is_public,
                    is_weak: dep.is_weak,
                    dependent: dep.dependent,
                    dependency: dep.dependency,
                }
            }
        }
        impl From<DependentInner> for DependencyInner {
            fn from(dep: DependentInner) -> Self {
                Self {
                    is_used: dep.is_used,
                    is_public: dep.is_public,
                    is_weak: dep.is_weak,
                    dependent: dep.dependent,
                    dependency: dep.dependency,
                }
            }
        }
        pub struct Dependent<'ast> {
            pub is_used: bool,
            pub is_public: bool,
            pub is_weak: bool,
            /// The `File`
            pub dependent: File<'ast>,
            /// The [`File`] containing this import.
            pub dependency: File<'ast>,
        }
        impl<'ast> Dependent<'ast> {
            pub fn as_dependency(self) -> Dependency<'ast> {
                Dependency {
                    is_used: self.is_used,
                    is_public: self.is_public,
                    is_weak: self.is_weak,
                    dependency: self.dependency,
                    dependent: self.dependent,
                }
            }
            #[must_use]
            pub fn is_used(self) -> bool {
                self.is_used
            }
            #[must_use]
            pub fn is_public(self) -> bool {
                self.is_public
            }
            #[must_use]
            pub fn is_weak(self) -> bool {
                self.is_weak
            }
            #[must_use]
            pub fn dependent(self) -> File<'ast> {
                self.dependent
            }
            #[must_use]
            pub fn dependency(self) -> File<'ast> {
                self.dependency
            }
            #[must_use]
            pub fn as_file(self) -> File<'ast> {
                self.dependent
            }
        }
        impl<'ast> Deref for Dependent<'ast> {
            type Target = File<'ast>;
            fn deref(&self) -> &Self::Target {
                &self.dependent
            }
        }
        pub(super) struct DependencyInner {
            pub(super) is_used: bool,
            pub(super) is_public: bool,
            pub(super) is_weak: bool,
            pub(super) dependency: Key,
            pub(super) dependent: Key,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DependencyInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "DependencyInner",
                    "is_used",
                    &self.is_used,
                    "is_public",
                    &self.is_public,
                    "is_weak",
                    &self.is_weak,
                    "dependency",
                    &self.dependency,
                    "dependent",
                    &&self.dependent,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DependencyInner {
            #[inline]
            fn default() -> DependencyInner {
                DependencyInner {
                    is_used: ::core::default::Default::default(),
                    is_public: ::core::default::Default::default(),
                    is_weak: ::core::default::Default::default(),
                    dependency: ::core::default::Default::default(),
                    dependent: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DependencyInner {
            #[inline]
            fn clone(&self) -> DependencyInner {
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<Key>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for DependencyInner {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DependencyInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DependencyInner {
            #[inline]
            fn eq(&self, other: &DependencyInner) -> bool {
                self.is_used == other.is_used
                    && self.is_public == other.is_public
                    && self.is_weak == other.is_weak
                    && self.dependency == other.dependency
                    && self.dependent == other.dependent
            }
        }
        impl DependencyInner {
            pub(super) fn set_is_used(&mut self, is_used: bool) {
                self.is_used = is_used;
            }
        }
        pub struct Dependency<'ast> {
            pub is_used: bool,
            pub is_public: bool,
            pub is_weak: bool,
            /// The imported `File`
            pub dependency: File<'ast>,
            /// The [`File`] containing this import.
            pub dependent: File<'ast>,
        }
        impl<'ast> Deref for Dependency<'ast> {
            type Target = File<'ast>;
            fn deref(&self) -> &Self::Target {
                &self.dependency
            }
        }
        impl<'ast> Dependency<'ast> {
            #[must_use]
            pub fn is_used(self) -> bool {
                self.is_used
            }
            #[must_use]
            pub fn is_public(self) -> bool {
                self.is_public
            }
            #[must_use]
            pub fn is_weak(self) -> bool {
                self.is_weak
            }
            #[must_use]
            pub fn import(self) -> File<'ast> {
                self.dependency
            }
            #[must_use]
            pub fn imported_by(self) -> File<'ast> {
                self.dependent
            }
            #[must_use]
            pub fn as_file(self) -> File<'ast> {
                self.dependency
            }
        }
        pub(super) struct DependenciesInner {
            pub(super) all: Vec<DependencyInner>,
            pub(super) public: Vec<DependencyInner>,
            pub(super) weak: Vec<DependencyInner>,
            pub(super) unusued: Vec<DependencyInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DependenciesInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "DependenciesInner",
                    "all",
                    &self.all,
                    "public",
                    &self.public,
                    "weak",
                    &self.weak,
                    "unusued",
                    &&self.unusued,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DependenciesInner {
            #[inline]
            fn default() -> DependenciesInner {
                DependenciesInner {
                    all: ::core::default::Default::default(),
                    public: ::core::default::Default::default(),
                    weak: ::core::default::Default::default(),
                    unusued: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DependenciesInner {
            #[inline]
            fn clone(&self) -> DependenciesInner {
                DependenciesInner {
                    all: ::core::clone::Clone::clone(&self.all),
                    public: ::core::clone::Clone::clone(&self.public),
                    weak: ::core::clone::Clone::clone(&self.weak),
                    unusued: ::core::clone::Clone::clone(&self.unusued),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DependenciesInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DependenciesInner {
            #[inline]
            fn eq(&self, other: &DependenciesInner) -> bool {
                self.all == other.all
                    && self.public == other.public
                    && self.weak == other.weak
                    && self.unusued == other.unusued
            }
        }
        pub(super) struct Hydrate {
            pub(super) name: Box<str>,
            pub(super) syntax: Option<String>,
            pub(super) options: FileOptions,
            pub(super) package: Option<package::Key>,
            pub(super) messages: Vec<Hydrated<message::Key>>,
            pub(super) enums: Vec<Hydrated<r#enum::Key>>,
            pub(super) services: Vec<Hydrated<service::Key>>,
            pub(super) extensions: Vec<Hydrated<extension::Key>>,
            pub(super) extension_blocks: Vec<extension_block::Key>,
            pub(super) dependencies: DependenciesInner,
            pub(super) package_comments: Option<location::Comments>,
            pub(super) comments: Option<location::Comments>,
            pub(super) is_build_target: bool,
        }
        #[doc(hidden)]
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            name: Box<str>,
            path: PathBuf,
            package: Option<package::Key>,
            messages: Set<message::Key>,
            enums: Set<r#enum::Key>,
            services: Set<service::Key>,
            extensions: Set<extension::Key>,
            extension_blocks: Vec<extension_block::Key>,
            dependencies: DependenciesInner,
            used_imports: Vec<DependencyInner>,
            unused_dependencies: Vec<DependencyInner>,
            transitive_dependencies: Vec<DependencyInner>,
            package_comments: Option<location::Comments>,
            comments: Option<location::Comments>,
            dependents: Vec<DependentInner>,
            transitive_dependents: Vec<DependentInner>,
            is_build_target: bool,
            syntax: Syntax,
            nodes_by_path: HashMap<Box<[i32]>, super::node::Key>,
            nodes_by_fqn: HashMap<FullyQualifiedName, super::node::Key>,
            java_package: Option<String>,
            java_outer_classname: Option<String>,
            java_multiple_files: bool,
            java_generate_equals_and_hash: bool,
            java_string_check_utf8: bool,
            java_generic_services: bool,
            optimize_for: Option<OptimizeMode>,
            go_package: Option<String>,
            cc_generic_services: bool,
            py_generic_services: bool,
            php_generic_services: bool,
            deprecated: bool,
            cc_enable_arenas: bool,
            objc_class_prefix: Option<String>,
            csharp_namespace: Option<String>,
            swift_prefix: Option<String>,
            php_class_prefix: Option<String>,
            php_namespace: Option<String>,
            php_metadata_namespace: Option<String>,
            ruby_package: Option<String>,
            uninterpreted_options: Vec<UninterpretedOption>,
            special_fields: SpecialFields,
            options_special_fields: SpecialFields,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "name",
                    "path",
                    "package",
                    "messages",
                    "enums",
                    "services",
                    "extensions",
                    "extension_blocks",
                    "dependencies",
                    "used_imports",
                    "unused_dependencies",
                    "transitive_dependencies",
                    "package_comments",
                    "comments",
                    "dependents",
                    "transitive_dependents",
                    "is_build_target",
                    "syntax",
                    "nodes_by_path",
                    "nodes_by_fqn",
                    "java_package",
                    "java_outer_classname",
                    "java_multiple_files",
                    "java_generate_equals_and_hash",
                    "java_string_check_utf8",
                    "java_generic_services",
                    "optimize_for",
                    "go_package",
                    "cc_generic_services",
                    "py_generic_services",
                    "php_generic_services",
                    "deprecated",
                    "cc_enable_arenas",
                    "objc_class_prefix",
                    "csharp_namespace",
                    "swift_prefix",
                    "php_class_prefix",
                    "php_namespace",
                    "php_metadata_namespace",
                    "ruby_package",
                    "uninterpreted_options",
                    "special_fields",
                    "options_special_fields",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.name,
                    &self.path,
                    &self.package,
                    &self.messages,
                    &self.enums,
                    &self.services,
                    &self.extensions,
                    &self.extension_blocks,
                    &self.dependencies,
                    &self.used_imports,
                    &self.unused_dependencies,
                    &self.transitive_dependencies,
                    &self.package_comments,
                    &self.comments,
                    &self.dependents,
                    &self.transitive_dependents,
                    &self.is_build_target,
                    &self.syntax,
                    &self.nodes_by_path,
                    &self.nodes_by_fqn,
                    &self.java_package,
                    &self.java_outer_classname,
                    &self.java_multiple_files,
                    &self.java_generate_equals_and_hash,
                    &self.java_string_check_utf8,
                    &self.java_generic_services,
                    &self.optimize_for,
                    &self.go_package,
                    &self.cc_generic_services,
                    &self.py_generic_services,
                    &self.php_generic_services,
                    &self.deprecated,
                    &self.cc_enable_arenas,
                    &self.objc_class_prefix,
                    &self.csharp_namespace,
                    &self.swift_prefix,
                    &self.php_class_prefix,
                    &self.php_namespace,
                    &self.php_metadata_namespace,
                    &self.ruby_package,
                    &self.uninterpreted_options,
                    &self.special_fields,
                    &&self.options_special_fields,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    path: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    messages: ::core::default::Default::default(),
                    enums: ::core::default::Default::default(),
                    services: ::core::default::Default::default(),
                    extensions: ::core::default::Default::default(),
                    extension_blocks: ::core::default::Default::default(),
                    dependencies: ::core::default::Default::default(),
                    used_imports: ::core::default::Default::default(),
                    unused_dependencies: ::core::default::Default::default(),
                    transitive_dependencies: ::core::default::Default::default(),
                    package_comments: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    dependents: ::core::default::Default::default(),
                    transitive_dependents: ::core::default::Default::default(),
                    is_build_target: ::core::default::Default::default(),
                    syntax: ::core::default::Default::default(),
                    nodes_by_path: ::core::default::Default::default(),
                    nodes_by_fqn: ::core::default::Default::default(),
                    java_package: ::core::default::Default::default(),
                    java_outer_classname: ::core::default::Default::default(),
                    java_multiple_files: ::core::default::Default::default(),
                    java_generate_equals_and_hash: ::core::default::Default::default(),
                    java_string_check_utf8: ::core::default::Default::default(),
                    java_generic_services: ::core::default::Default::default(),
                    optimize_for: ::core::default::Default::default(),
                    go_package: ::core::default::Default::default(),
                    cc_generic_services: ::core::default::Default::default(),
                    py_generic_services: ::core::default::Default::default(),
                    php_generic_services: ::core::default::Default::default(),
                    deprecated: ::core::default::Default::default(),
                    cc_enable_arenas: ::core::default::Default::default(),
                    objc_class_prefix: ::core::default::Default::default(),
                    csharp_namespace: ::core::default::Default::default(),
                    swift_prefix: ::core::default::Default::default(),
                    php_class_prefix: ::core::default::Default::default(),
                    php_namespace: ::core::default::Default::default(),
                    php_metadata_namespace: ::core::default::Default::default(),
                    ruby_package: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    special_fields: ::core::default::Default::default(),
                    options_special_fields: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    name: ::core::clone::Clone::clone(&self.name),
                    path: ::core::clone::Clone::clone(&self.path),
                    package: ::core::clone::Clone::clone(&self.package),
                    messages: ::core::clone::Clone::clone(&self.messages),
                    enums: ::core::clone::Clone::clone(&self.enums),
                    services: ::core::clone::Clone::clone(&self.services),
                    extensions: ::core::clone::Clone::clone(&self.extensions),
                    extension_blocks: ::core::clone::Clone::clone(&self.extension_blocks),
                    dependencies: ::core::clone::Clone::clone(&self.dependencies),
                    used_imports: ::core::clone::Clone::clone(&self.used_imports),
                    unused_dependencies: ::core::clone::Clone::clone(&self.unused_dependencies),
                    transitive_dependencies: ::core::clone::Clone::clone(
                        &self.transitive_dependencies,
                    ),
                    package_comments: ::core::clone::Clone::clone(&self.package_comments),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    dependents: ::core::clone::Clone::clone(&self.dependents),
                    transitive_dependents: ::core::clone::Clone::clone(&self.transitive_dependents),
                    is_build_target: ::core::clone::Clone::clone(&self.is_build_target),
                    syntax: ::core::clone::Clone::clone(&self.syntax),
                    nodes_by_path: ::core::clone::Clone::clone(&self.nodes_by_path),
                    nodes_by_fqn: ::core::clone::Clone::clone(&self.nodes_by_fqn),
                    java_package: ::core::clone::Clone::clone(&self.java_package),
                    java_outer_classname: ::core::clone::Clone::clone(&self.java_outer_classname),
                    java_multiple_files: ::core::clone::Clone::clone(&self.java_multiple_files),
                    java_generate_equals_and_hash: ::core::clone::Clone::clone(
                        &self.java_generate_equals_and_hash,
                    ),
                    java_string_check_utf8: ::core::clone::Clone::clone(
                        &self.java_string_check_utf8,
                    ),
                    java_generic_services: ::core::clone::Clone::clone(&self.java_generic_services),
                    optimize_for: ::core::clone::Clone::clone(&self.optimize_for),
                    go_package: ::core::clone::Clone::clone(&self.go_package),
                    cc_generic_services: ::core::clone::Clone::clone(&self.cc_generic_services),
                    py_generic_services: ::core::clone::Clone::clone(&self.py_generic_services),
                    php_generic_services: ::core::clone::Clone::clone(&self.php_generic_services),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    cc_enable_arenas: ::core::clone::Clone::clone(&self.cc_enable_arenas),
                    objc_class_prefix: ::core::clone::Clone::clone(&self.objc_class_prefix),
                    csharp_namespace: ::core::clone::Clone::clone(&self.csharp_namespace),
                    swift_prefix: ::core::clone::Clone::clone(&self.swift_prefix),
                    php_class_prefix: ::core::clone::Clone::clone(&self.php_class_prefix),
                    php_namespace: ::core::clone::Clone::clone(&self.php_namespace),
                    php_metadata_namespace: ::core::clone::Clone::clone(
                        &self.php_metadata_namespace,
                    ),
                    ruby_package: ::core::clone::Clone::clone(&self.ruby_package),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    special_fields: ::core::clone::Clone::clone(&self.special_fields),
                    options_special_fields: ::core::clone::Clone::clone(
                        &self.options_special_fields,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.name == other.name
                    && self.path == other.path
                    && self.package == other.package
                    && self.messages == other.messages
                    && self.enums == other.enums
                    && self.services == other.services
                    && self.extensions == other.extensions
                    && self.extension_blocks == other.extension_blocks
                    && self.dependencies == other.dependencies
                    && self.used_imports == other.used_imports
                    && self.unused_dependencies == other.unused_dependencies
                    && self.transitive_dependencies == other.transitive_dependencies
                    && self.package_comments == other.package_comments
                    && self.comments == other.comments
                    && self.dependents == other.dependents
                    && self.transitive_dependents == other.transitive_dependents
                    && self.is_build_target == other.is_build_target
                    && self.syntax == other.syntax
                    && self.nodes_by_path == other.nodes_by_path
                    && self.nodes_by_fqn == other.nodes_by_fqn
                    && self.java_package == other.java_package
                    && self.java_outer_classname == other.java_outer_classname
                    && self.java_multiple_files == other.java_multiple_files
                    && self.java_generate_equals_and_hash == other.java_generate_equals_and_hash
                    && self.java_string_check_utf8 == other.java_string_check_utf8
                    && self.java_generic_services == other.java_generic_services
                    && self.optimize_for == other.optimize_for
                    && self.go_package == other.go_package
                    && self.cc_generic_services == other.cc_generic_services
                    && self.py_generic_services == other.py_generic_services
                    && self.php_generic_services == other.php_generic_services
                    && self.deprecated == other.deprecated
                    && self.cc_enable_arenas == other.cc_enable_arenas
                    && self.objc_class_prefix == other.objc_class_prefix
                    && self.csharp_namespace == other.csharp_namespace
                    && self.swift_prefix == other.swift_prefix
                    && self.php_class_prefix == other.php_class_prefix
                    && self.php_namespace == other.php_namespace
                    && self.php_metadata_namespace == other.php_metadata_namespace
                    && self.ruby_package == other.ruby_package
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.special_fields == other.special_fields
                    && self.options_special_fields == other.options_special_fields
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = super::node::Key> {
                std::iter::empty()
                    .chain(self.messages.iter().copied().map(Into::into))
                    .chain(self.enums.iter().copied().map(Into::into))
                    .chain(self.services.iter().copied().map(Into::into))
                    .chain(self.extensions.iter().copied().map(Into::into))
            }
        }
        impl Inner {
            pub(super) fn add_dependent(&mut self, dependent: DependentInner) {
                self.dependents.push(dependent);
                self.transitive_dependents.push(dependent);
            }
            pub(super) fn add_transitive_dependent(&mut self, dependent: DependentInner) {
                self.transitive_dependents.push(dependent);
            }
            pub(super) fn add_transitive_dependency(&mut self, import: DependencyInner) {
                self.transitive_dependencies.push(import);
            }
            pub(super) fn set_name_and_path(&mut self, name: Box<str>) {
                self.path = PathBuf::from(name.as_ref());
                self.set_name(name);
            }
            pub(super) fn set_fqn(&mut self, fqn: FullyQualifiedName) {
                self.fqn = fqn;
            }
            pub(super) fn set_package_comments(&mut self, package_comments: location::Comments) {
                self.package_comments = Some(package_comments);
            }
            pub(super) fn set_is_build_target(&mut self, is_build_target: bool) {
                self.is_build_target = is_build_target;
            }
            pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Hydrated<Key>, Error> {
                let Hydrate {
                    name,
                    syntax,
                    options,
                    package,
                    messages,
                    enums,
                    services,
                    extensions,
                    extension_blocks,
                    dependencies,
                    package_comments,
                    comments,
                    is_build_target,
                } = hydrate;
                self.set_name_and_path(name);
                self.syntax = Syntax::parse(&syntax.unwrap_or_default())?;
                self.package = package;
                self.messages = messages.into();
                self.enums = enums.into();
                self.services = services.into();
                self.extensions = extensions.into();
                self.extension_blocks = extension_blocks;
                self.dependencies = dependencies;
                self.package_comments = package_comments;
                self.comments = comments;
                self.is_build_target = is_build_target;
                self.hydrate_options(options);
                Ok((self.key, self.fqn.clone(), self.name.clone()))
            }
            /// Hydrates the data within the descriptor.
            ///
            /// Note: References and nested nodes are not hydrated.
            fn hydrate_options(&mut self, opts: FileOptions) {
                self.java_package = opts.java_package;
                self.java_outer_classname = opts.java_outer_classname;
                self.java_multiple_files = opts.java_multiple_files.unwrap_or(false);
                self.java_generate_equals_and_hash =
                    opts.java_generate_equals_and_hash.unwrap_or(false);
                self.java_string_check_utf8 = opts.java_string_check_utf8.unwrap_or(false);
                self.java_generic_services = opts.java_generic_services.unwrap_or(false);
                self.optimize_for = opts.optimize_for.map(Into::into);
                self.go_package = opts.go_package;
                self.cc_generic_services = opts.cc_generic_services.unwrap_or(false);
                self.py_generic_services = opts.py_generic_services.unwrap_or(false);
                self.php_generic_services = opts.php_generic_services.unwrap_or(false);
                self.deprecated = opts.deprecated.unwrap_or(false);
                self.cc_enable_arenas = opts.cc_enable_arenas.unwrap_or(false);
                self.objc_class_prefix = opts.objc_class_prefix;
                self.csharp_namespace = opts.csharp_namespace;
                self.swift_prefix = opts.swift_prefix;
                self.php_class_prefix = opts.php_class_prefix;
                self.php_namespace = opts.php_namespace;
                self.php_metadata_namespace = opts.php_metadata_namespace;
                self.ruby_package = opts.ruby_package;
                self.uninterpreted_options = opts
                    .uninterpreted_option
                    .into_iter()
                    .map(Into::into)
                    .collect();
                self.options_special_fields = opts.special_fields;
            }
            pub(crate) fn set_nodes_by_path(
                &mut self,
                mut nodes: HashMap<Box<[i32]>, super::node::Key>,
            ) {
                nodes.shrink_to_fit();
                self.nodes_by_path = nodes;
            }
            pub(crate) fn set_nodes_by_fqn(
                &mut self,
                mut nodes: HashMap<FullyQualifiedName, super::node::Key>,
            ) {
                nodes.shrink_to_fit();
                self.nodes_by_fqn = nodes;
            }
        }
        fn parse_syntax(syntax: &str) -> Result<Syntax, Error> {
            match syntax {
                "proto2" => Ok(Syntax::Proto2),
                "proto3" => Ok(Syntax::Proto3),
                _ => Err(Error::unsupported_syntax(syntax)),
            }
        }
    }
    pub mod location {
        use super::path;
        use crate::error::Error;
        use protobuf::descriptor::{source_code_info::Location as ProtoLoc, SourceCodeInfo};
        use std::iter::Peekable;
        /// Zero-based spans of a node.
        pub struct Span {
            pub start_line: i32,
            pub start_column: i32,
            pub end_line: i32,
            pub end_column: i32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Span {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Span",
                    "start_line",
                    &self.start_line,
                    "start_column",
                    &self.start_column,
                    "end_line",
                    &self.end_line,
                    "end_column",
                    &&self.end_column,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Span {
            #[inline]
            fn default() -> Span {
                Span {
                    start_line: ::core::default::Default::default(),
                    start_column: ::core::default::Default::default(),
                    end_line: ::core::default::Default::default(),
                    end_column: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Span {
            #[inline]
            fn clone(&self) -> Span {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Span {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Span {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Span {
            #[inline]
            fn eq(&self, other: &Span) -> bool {
                self.start_line == other.start_line
                    && self.start_column == other.start_column
                    && self.end_line == other.end_line
                    && self.end_column == other.end_column
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Span {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Span {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        impl Span {
            fn new(span: &[i32]) -> Result<Self, ()> {
                match span.len() {
                    3 => Ok(Self {
                        start_line: span[0],
                        start_column: span[1],
                        end_line: span[0],
                        end_column: span[2],
                    }),
                    4 => Ok(Self {
                        start_line: span[0],
                        start_column: span[1],
                        end_line: span[2],
                        end_column: span[3],
                    }),
                    _ => Err(()),
                }
            }
            pub fn start_line(&self) -> i32 {
                self.start_line
            }
            pub fn start_column(&self) -> i32 {
                self.start_column
            }
            pub fn end_line(&self) -> i32 {
                self.end_line
            }
            pub fn end_column(&self) -> i32 {
                self.end_column
            }
        }
        pub struct Comments {
            /// Any comment immediately preceding the node, without any
            /// whitespace between it and the comment.
            leading: Option<Box<str>>,
            trailing: Option<Box<str>>,
            leading_detached: Vec<Box<str>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Comments {
            #[inline]
            fn clone(&self) -> Comments {
                Comments {
                    leading: ::core::clone::Clone::clone(&self.leading),
                    trailing: ::core::clone::Clone::clone(&self.trailing),
                    leading_detached: ::core::clone::Clone::clone(&self.leading_detached),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Comments {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Comments",
                    "leading",
                    &self.leading,
                    "trailing",
                    &self.trailing,
                    "leading_detached",
                    &&self.leading_detached,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Comments {
            #[inline]
            fn default() -> Comments {
                Comments {
                    leading: ::core::default::Default::default(),
                    trailing: ::core::default::Default::default(),
                    leading_detached: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Comments {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Comments {
            #[inline]
            fn eq(&self, other: &Comments) -> bool {
                self.leading == other.leading
                    && self.trailing == other.trailing
                    && self.leading_detached == other.leading_detached
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Comments {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Comments {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Option<Box<str>>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Box<str>>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Box<str>>>;
            }
        }
        impl Comments {
            pub fn new_maybe(
                leading: Option<String>,
                trailing: Option<String>,
                leading_detacted: Vec<String>,
            ) -> Option<Self> {
                if leading.is_none() && trailing.is_none() && leading_detacted.is_empty() {
                    return None;
                }
                let leading = leading.map(String::into_boxed_str);
                let trailing = trailing.map(String::into_boxed_str);
                let leading_detached = leading_detacted
                    .into_iter()
                    .map(String::into_boxed_str)
                    .collect();
                Some(Self {
                    leading,
                    trailing,
                    leading_detached,
                })
            }
            /// Any comment immediately preceding the node, without any
            /// whitespace between it and the comment.
            pub fn leading(&self) -> Option<&str> {
                self.leading.as_deref()
            }
            /// Any comment immediately following the entity, without any
            /// whitespace between it and the comment. If the comment would be a
            /// leading comment for another entity, it won't be
            /// considered a trailing comment.
            pub fn trailing(&self) -> Option<&str> {
                self.trailing.as_deref()
            }
            /// Each comment block or line above the entity but seperated by
            /// whitespace.
            pub fn leading_detached(&self) -> &[Box<str>] {
                &self.leading_detached
            }
        }
        type Iter = Peekable<std::vec::IntoIter<ProtoLoc>>;
        fn iterate_next<T>(prefix: &[i32], locations: &mut Iter) -> Option<(ProtoLoc, T)>
        where
            T: From<i32>,
        {
            let peeked = locations.peek()?;
            let subpath = peeked.path.get(..prefix.len())?;
            if subpath != &prefix[..subpath.len()] || peeked.path.len() == prefix.len() {
                return None;
            }
            locations.next().and_then(|next| {
                let next_path = next.path.get(prefix.len()).map(|&n| T::from(n))?;
                Some((next, next_path))
            })
        }
        pub(super) struct Detail {
            pub(super) path: Box<[i32]>,
            pub(super) span: Span,
            pub(super) comments: Option<Comments>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Detail {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Detail",
                    "path",
                    &self.path,
                    "span",
                    &self.span,
                    "comments",
                    &&self.comments,
                )
            }
        }
        impl Detail {
            pub(super) fn new(loc: ProtoLoc) -> Result<Self, Error> {
                let span = Span::new(&loc.span).map_err(|()| Error::invalid_span(&loc))?;
                let comments = Comments::new_maybe(
                    loc.leading_comments,
                    loc.trailing_comments,
                    loc.leading_detached_comments,
                );
                let path = loc.path.into();
                Ok(Self {
                    path,
                    span,
                    comments,
                })
            }
        }
        pub(super) struct File {
            pub(super) syntax: Option<Detail>,
            pub(super) package: Option<Detail>,
            pub(super) dependencies: Vec<Detail>,
            pub(super) messages: Vec<Message>,
            pub(super) enums: Vec<Enum>,
            pub(super) services: Vec<Service>,
            pub(super) extensions: Vec<ExtensionDecl>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for File {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "syntax",
                    "package",
                    "dependencies",
                    "messages",
                    "enums",
                    "services",
                    "extensions",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.syntax,
                    &self.package,
                    &self.dependencies,
                    &self.messages,
                    &self.enums,
                    &self.services,
                    &&self.extensions,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "File", names, values)
            }
        }
        impl File {
            pub(super) fn new(info: SourceCodeInfo) -> Result<Self, Error> {
                let mut locations = info.location.into_iter().peekable();
                let mut package = None;
                let mut syntax = None;
                let mut messages = Vec::new();
                let mut enums = Vec::new();
                let mut services = Vec::new();
                let mut dependencies = Vec::new();
                let mut extensions = Vec::new();
                let Detail {
                    path,
                    span,
                    comments,
                } = Detail::new(locations.next().unwrap())?;
                while let Some(loc) = locations.next() {
                    match path::File::from_i32(loc.path[0]) {
                        path::File::Syntax => {
                            syntax = Some(Detail::new(loc)?);
                        }
                        path::File::Dependency => {
                            dependencies.push(Detail::new(loc)?);
                        }
                        path::File::Package => {
                            package = Some(Detail::new(loc)?);
                        }
                        path::File::Message => {
                            messages.push(Message::new(loc, &mut locations)?);
                        }
                        path::File::Enum => {
                            enums.push(Enum::new(loc, &mut locations)?);
                        }
                        path::File::Service => {
                            services.push(Service::new(loc, &mut locations)?);
                        }
                        path::File::Extension => {
                            extensions.push(ExtensionDecl::new(loc, &mut locations)?);
                        }
                        _ => continue,
                    }
                }
                Ok(Self {
                    syntax,
                    package,
                    dependencies,
                    messages,
                    enums,
                    services,
                    extensions,
                })
            }
        }
        pub(super) struct Message {
            pub(super) detail: Detail,
            pub(super) messages: Vec<Message>,
            pub(super) enums: Vec<Enum>,
            pub(super) extensions: Vec<ExtensionDecl>,
            pub(super) oneofs: Vec<Oneof>,
            pub(super) fields: Vec<Field>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Message {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "detail",
                    "messages",
                    "enums",
                    "extensions",
                    "oneofs",
                    "fields",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.detail,
                    &self.messages,
                    &self.enums,
                    &self.extensions,
                    &self.oneofs,
                    &&self.fields,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Message", names, values)
            }
        }
        impl Message {
            fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                let mut messages = Vec::new();
                let mut enums = Vec::new();
                let mut extensions = Vec::new();
                let mut oneofs = Vec::new();
                let mut fields = Vec::new();
                while let Some((loc, path)) = iterate_next(&detail.path, locations) {
                    match path {
                        path::Message::Field => {
                            fields.push(Field::new(loc, locations)?);
                        }
                        path::Message::Nested => {
                            messages.push(Self::new(loc, locations)?);
                        }
                        path::Message::Enum => {
                            enums.push(Enum::new(loc, locations)?);
                        }
                        path::Message::Extension => {
                            extensions.push(ExtensionDecl::new(loc, locations)?);
                        }
                        path::Message::Oneof => {
                            oneofs.push(Oneof::new(loc, locations)?);
                        }
                        path::Message::Unknown(_) => continue,
                    }
                }
                Ok(Self {
                    detail,
                    messages,
                    enums,
                    extensions,
                    oneofs,
                    fields,
                })
            }
        }
        pub(super) struct ExtensionDecl {
            pub(super) detail: Detail,
            pub(super) extensions: Vec<Field>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExtensionDecl {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ExtensionDecl",
                    "detail",
                    &self.detail,
                    "extensions",
                    &&self.extensions,
                )
            }
        }
        impl ExtensionDecl {
            fn new(
                node: ProtoLoc,
                locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
            ) -> Result<Self, Error> {
                let mut extensions = Vec::new();
                let detail = Detail::new(node)?;
                while let Some((next, _)) = iterate_next::<i32>(&detail.path, locations) {
                    extensions.push(Field::new(next, locations)?);
                }
                Ok(Self { detail, extensions })
            }
        }
        pub(super) struct Field {
            pub(super) detail: Detail,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Field {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Field",
                    "detail",
                    &&self.detail,
                )
            }
        }
        impl Field {
            fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                while iterate_next::<i32>(&detail.path, locations).is_some() {}
                Ok(Self { detail })
            }
        }
        pub(super) struct Oneof {
            pub(super) detail: Detail,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Oneof {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Oneof",
                    "detail",
                    &&self.detail,
                )
            }
        }
        impl Oneof {
            fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                while iterate_next::<i32>(&detail.path, locations).is_some() {}
                Ok(Self { detail })
            }
        }
        pub(super) struct Enum {
            pub(super) detail: Detail,
            pub(super) values: Vec<EnumValue>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Enum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Enum",
                    "detail",
                    &self.detail,
                    "values",
                    &&self.values,
                )
            }
        }
        impl Enum {
            fn new(
                node: ProtoLoc,
                locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
            ) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                let mut values = Vec::new();
                while let Some((next, next_path)) = iterate_next(&detail.path, locations) {
                    match next_path {
                        path::Enum::Value => {
                            values.push(EnumValue::new(next, locations)?);
                        }
                        path::Enum::Unknown(_) => continue,
                    }
                }
                Ok(Self { detail, values })
            }
        }
        pub(super) struct EnumValue {
            pub(super) detail: Detail,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for EnumValue {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "EnumValue",
                    "detail",
                    &&self.detail,
                )
            }
        }
        impl EnumValue {
            fn new(
                node: ProtoLoc,
                locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
            ) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                while iterate_next::<i32>(&detail.path, locations).is_some() {}
                Ok(Self { detail })
            }
        }
        pub(super) struct Service {
            pub(super) detail: Detail,
            pub(super) methods: Vec<Method>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Service {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Service",
                    "detail",
                    &self.detail,
                    "methods",
                    &&self.methods,
                )
            }
        }
        impl Service {
            fn new(
                node: ProtoLoc,
                locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
            ) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                let mut methods = Vec::new();
                while let Some((next, next_path)) = iterate_next(&detail.path, locations) {
                    match next_path {
                        path::Service::Method => {
                            methods.push(Method::new(next, locations)?);
                        }
                        path::Service::Mixin | path::Service::Unknown(_) => continue,
                    }
                }
                Ok(Self { detail, methods })
            }
        }
        pub(super) struct Method {
            pub(super) detail: Detail,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Method {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Method",
                    "detail",
                    &&self.detail,
                )
            }
        }
        impl Method {
            fn new(
                node: ProtoLoc,
                locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
            ) -> Result<Self, Error> {
                let detail = Detail::new(node)?;
                while iterate_next::<i32>(&detail.path, locations).is_some() {}
                Ok(Self { detail })
            }
        }
    }
    pub mod message {
        use super::{
            access::{self, NodeKeys},
            container, r#enum, extension, extension_block, field, file, impl_traits_and_methods,
            location::{self, Comments, Span},
            message, node, oneof, package,
            reference::{ReferenceInner, References},
            reserved::Reserved,
            resolve::Resolver,
            uninterpreted::UninterpretedOption,
            FullyQualifiedName, Hydrated, Set,
        };
        use protobuf::{
            descriptor::{descriptor_proto, MessageOptions},
            SpecialFields,
        };
        use std::iter;
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub(super) struct Hydrate {
            pub(super) name: Box<str>,
            pub(super) container: container::Key,
            pub(super) package: Option<package::Key>,
            pub(super) well_known: Option<WellKnownMessage>,
            pub(super) location: location::Detail,
            pub(super) options: protobuf::MessageField<MessageOptions>,
            pub(super) reserved_ranges: Vec<descriptor_proto::ReservedRange>,
            pub(super) reserved_names: Vec<String>,
            pub(super) extension_range: Vec<descriptor_proto::ExtensionRange>,
            pub(super) special_fields: protobuf::SpecialFields,
            pub(super) messages: Vec<Hydrated<message::Key>>,
            pub(super) enums: Vec<Hydrated<r#enum::Key>>,
            pub(super) fields: Vec<Hydrated<field::Key>>,
            pub(super) oneofs: Vec<Hydrated<oneof::Key>>,
            pub(super) extensions: Vec<Hydrated<extension::Key>>,
            pub(super) extension_blocks: Vec<extension_block::Key>,
        }
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            name: Box<str>,
            node_path: Box<[i32]>,
            span: Span,
            comments: Option<Comments>,
            container: container::Key,
            package: Option<package::Key>,
            file: file::Key,
            extensions: Set<extension::Key>,
            extension_blocks: Vec<extension_block::Key>,
            fields: Set<field::Key>,
            enums: Set<r#enum::Key>,
            messages: Set<message::Key>,
            oneofs: Set<oneof::Key>,
            real_oneofs: Set<oneof::Key>,
            synthetic_oneofs: Set<oneof::Key>,
            defined_extensions: Set<extension::Key>,
            applied_extensions: Set<extension::Key>,
            dependents: Set<file::Key>,
            referenced_by: Vec<ReferenceInner>,
            references: Vec<ReferenceInner>,
            extension_ranges: Vec<ExtensionRange>,
            reserved: Reserved,
            message_set_wire_format: bool,
            no_standard_descriptor_accessor: bool,
            deprecated: bool,
            map_entry: bool,
            uninterpreted_options: Vec<UninterpretedOption>,
            unknown_fields: protobuf::UnknownFields,
            special_fields: SpecialFields,
            options_special_fields: SpecialFields,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "name",
                    "node_path",
                    "span",
                    "comments",
                    "container",
                    "package",
                    "file",
                    "extensions",
                    "extension_blocks",
                    "fields",
                    "enums",
                    "messages",
                    "oneofs",
                    "real_oneofs",
                    "synthetic_oneofs",
                    "defined_extensions",
                    "applied_extensions",
                    "dependents",
                    "referenced_by",
                    "references",
                    "extension_ranges",
                    "reserved",
                    "message_set_wire_format",
                    "no_standard_descriptor_accessor",
                    "deprecated",
                    "map_entry",
                    "uninterpreted_options",
                    "unknown_fields",
                    "special_fields",
                    "options_special_fields",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.name,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.container,
                    &self.package,
                    &self.file,
                    &self.extensions,
                    &self.extension_blocks,
                    &self.fields,
                    &self.enums,
                    &self.messages,
                    &self.oneofs,
                    &self.real_oneofs,
                    &self.synthetic_oneofs,
                    &self.defined_extensions,
                    &self.applied_extensions,
                    &self.dependents,
                    &self.referenced_by,
                    &self.references,
                    &self.extension_ranges,
                    &self.reserved,
                    &self.message_set_wire_format,
                    &self.no_standard_descriptor_accessor,
                    &self.deprecated,
                    &self.map_entry,
                    &self.uninterpreted_options,
                    &self.unknown_fields,
                    &self.special_fields,
                    &&self.options_special_fields,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    name: ::core::clone::Clone::clone(&self.name),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    container: ::core::clone::Clone::clone(&self.container),
                    package: ::core::clone::Clone::clone(&self.package),
                    file: ::core::clone::Clone::clone(&self.file),
                    extensions: ::core::clone::Clone::clone(&self.extensions),
                    extension_blocks: ::core::clone::Clone::clone(&self.extension_blocks),
                    fields: ::core::clone::Clone::clone(&self.fields),
                    enums: ::core::clone::Clone::clone(&self.enums),
                    messages: ::core::clone::Clone::clone(&self.messages),
                    oneofs: ::core::clone::Clone::clone(&self.oneofs),
                    real_oneofs: ::core::clone::Clone::clone(&self.real_oneofs),
                    synthetic_oneofs: ::core::clone::Clone::clone(&self.synthetic_oneofs),
                    defined_extensions: ::core::clone::Clone::clone(&self.defined_extensions),
                    applied_extensions: ::core::clone::Clone::clone(&self.applied_extensions),
                    dependents: ::core::clone::Clone::clone(&self.dependents),
                    referenced_by: ::core::clone::Clone::clone(&self.referenced_by),
                    references: ::core::clone::Clone::clone(&self.references),
                    extension_ranges: ::core::clone::Clone::clone(&self.extension_ranges),
                    reserved: ::core::clone::Clone::clone(&self.reserved),
                    message_set_wire_format: ::core::clone::Clone::clone(
                        &self.message_set_wire_format,
                    ),
                    no_standard_descriptor_accessor: ::core::clone::Clone::clone(
                        &self.no_standard_descriptor_accessor,
                    ),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    map_entry: ::core::clone::Clone::clone(&self.map_entry),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    unknown_fields: ::core::clone::Clone::clone(&self.unknown_fields),
                    special_fields: ::core::clone::Clone::clone(&self.special_fields),
                    options_special_fields: ::core::clone::Clone::clone(
                        &self.options_special_fields,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    container: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    extensions: ::core::default::Default::default(),
                    extension_blocks: ::core::default::Default::default(),
                    fields: ::core::default::Default::default(),
                    enums: ::core::default::Default::default(),
                    messages: ::core::default::Default::default(),
                    oneofs: ::core::default::Default::default(),
                    real_oneofs: ::core::default::Default::default(),
                    synthetic_oneofs: ::core::default::Default::default(),
                    defined_extensions: ::core::default::Default::default(),
                    applied_extensions: ::core::default::Default::default(),
                    dependents: ::core::default::Default::default(),
                    referenced_by: ::core::default::Default::default(),
                    references: ::core::default::Default::default(),
                    extension_ranges: ::core::default::Default::default(),
                    reserved: ::core::default::Default::default(),
                    message_set_wire_format: ::core::default::Default::default(),
                    no_standard_descriptor_accessor: ::core::default::Default::default(),
                    deprecated: ::core::default::Default::default(),
                    map_entry: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    unknown_fields: ::core::default::Default::default(),
                    special_fields: ::core::default::Default::default(),
                    options_special_fields: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.name == other.name
                    && self.node_path == other.node_path
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.container == other.container
                    && self.package == other.package
                    && self.file == other.file
                    && self.extensions == other.extensions
                    && self.extension_blocks == other.extension_blocks
                    && self.fields == other.fields
                    && self.enums == other.enums
                    && self.messages == other.messages
                    && self.oneofs == other.oneofs
                    && self.real_oneofs == other.real_oneofs
                    && self.synthetic_oneofs == other.synthetic_oneofs
                    && self.defined_extensions == other.defined_extensions
                    && self.applied_extensions == other.applied_extensions
                    && self.dependents == other.dependents
                    && self.referenced_by == other.referenced_by
                    && self.references == other.references
                    && self.extension_ranges == other.extension_ranges
                    && self.reserved == other.reserved
                    && self.message_set_wire_format == other.message_set_wire_format
                    && self.no_standard_descriptor_accessor == other.no_standard_descriptor_accessor
                    && self.deprecated == other.deprecated
                    && self.map_entry == other.map_entry
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.unknown_fields == other.unknown_fields
                    && self.special_fields == other.special_fields
                    && self.options_special_fields == other.options_special_fields
            }
        }
        impl super::access::ReferencesMut for Inner {
            fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
                self.references
                    .iter_mut()
                    .chain(self.referenced_by.iter_mut())
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = node::Key> {
                iter::empty()
                    .chain(self.fields.iter().copied().map(node::Key::Field))
                    .chain(self.enums.iter().copied().map(node::Key::Enum))
                    .chain(self.messages.iter().copied().map(node::Key::Message))
                    .chain(self.oneofs.iter().copied().map(node::Key::Oneof))
                    .chain(
                        self.defined_extensions
                            .iter()
                            .copied()
                            .map(node::Key::Extension),
                    )
            }
        }
        impl Inner {
            pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Hydrated<Key> {
                let Hydrate {
                    name,
                    container,
                    package,
                    location,
                    options,
                    well_known,
                    reserved_ranges,
                    reserved_names,
                    extension_range,
                    special_fields,
                    messages,
                    enums,
                    fields,
                    oneofs,
                    extensions,
                    extension_blocks,
                } = hydrate;
                self.name = name;
                self.package = package;
                self.container = container;
                self.extension_ranges = extension_range.into_iter().map(Into::into).collect();
                self.special_fields = special_fields;
                self.messages = messages.into();
                self.enums = enums.into();
                self.fields = fields.into();
                self.oneofs = oneofs.into();
                self.extensions = extensions.into();
                self.extension_blocks = extension_blocks;
                self.hydrate_location(location);
                self.hydrate_options(options.unwrap_or_default());
                self.set_reserved(reserved_names, reserved_ranges);
                (self.key, self.fqn.clone(), self.name.clone())
            }
            fn hydrate_options(&mut self, opts: MessageOptions) {
                let MessageOptions {
                    message_set_wire_format,
                    no_standard_descriptor_accessor,
                    deprecated,
                    map_entry,
                    uninterpreted_option,
                    special_fields,
                } = opts;
                self.message_set_wire_format = message_set_wire_format.unwrap_or(false);
                self.no_standard_descriptor_accessor =
                    no_standard_descriptor_accessor.unwrap_or(false);
                self.deprecated = deprecated.unwrap_or(false);
                self.map_entry = map_entry.unwrap_or(false);
                self.uninterpreted_options =
                    uninterpreted_option.into_iter().map(Into::into).collect();
                self.options_special_fields = special_fields;
            }
        }
        pub struct Message<'ast>(Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Message<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Message<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Message<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Message<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Message<'ast> {}
        impl<'ast> PartialEq for Message<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Message<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Message<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Message<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Message<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Message<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Message<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Message<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Message<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Message<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Message<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl Inner {
            fn set_reserved<R>(&mut self, names: Vec<String>, ranges: Vec<R>)
            where
                R: Into<crate::ast::reserved::ReservedRange>,
            {
                self.reserved = crate::ast::reserved::Reserved {
                    names: names.into(),
                    ranges: ranges.into_iter().map(Into::into).collect(),
                };
            }
        }
        impl<'ast> Message<'ast> {
            pub fn reserved_names(&self) -> &[String] {
                &self.0.reserved.names
            }
            pub fn reserved_ranges(&self) -> &[crate::ast::reserved::ReservedRange] {
                &self.0.reserved.ranges
            }
            pub fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
        impl<'ast> crate::ast::access::Reserved for Message<'ast> {
            fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Message<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Message<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Message<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Message<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Message<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Message<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Message<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Message<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Message<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Message<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
        impl<'ast> Message<'ast> {
            pub fn references(&'ast self) -> References<'ast> {
                access::References::references(self)
            }
            pub fn referenced_by(&'ast self) -> References<'ast> {
                access::ReferencedBy::referenced_by(self)
            }
        }
        impl<'ast> access::References<'ast> for Message<'ast> {
            fn references(&'ast self) -> super::reference::References<'ast> {
                References::from_slice(&self.0.references, self.ast())
            }
        }
        impl<'ast> access::ReferencedBy<'ast> for Message<'ast> {
            fn referenced_by(&'ast self) -> super::reference::References<'ast> {
                References::from_slice(&self.0.referenced_by, self.ast())
            }
        }
        pub enum WellKnownMessage {
            /// Any contains an arbitrary serialized message along with a URL
            /// that describes the type of the serialized message.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Any>
            Any,
            /// Api is a light-weight descriptor for a protocol buffer service.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Api>
            Api,
            /// Wrapper message for bool.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.BoolValue>
            BoolValue,
            /// Wrapper message for bytes.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#bytesvalue>
            BytesValue,
            /// Wrapper message for double.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#doublevalue>
            DoubleValue,
            /// A Duration represents a signed, fixed-length span of time
            /// represented as a count of seconds and fractions of
            /// seconds at nanosecond resolution. It is independent
            /// of any calendar and concepts like "day" or "month". It is
            /// related to Timestamp in that the difference between two
            /// Timestamp values is a Duration and it can be added
            /// or subtracted from a Timestamp. Range
            /// is approximately +-10,000 years.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration>
            Duration,
            /// A generic empty message that you can re-use to avoid defining
            /// duplicated empty messages in your APIs. A typical
            /// example is to use it as the request or the response
            /// type of an API method. For Instance:
            ///
            /// ```protobuf
            /// service Foo {
            ///     rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
            /// }
            /// ```
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty>
            Empty,
            /// Enum type definition.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enum>
            Enum,
            /// Enum value definition.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#enumvalue>
            EnumValue,
            /// A single field of a message type.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#field>
            Field,
            FieldKind,
            /// FieldMask represents a set of symbolic field paths, for example:
            /// ```protobuf
            /// paths: "f.a"
            /// paths: "f.b.d"
            /// ```
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
            FieldMask,
            /// Wrapper message for float.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#floatvalue>
            FloatValue,
            /// Wrapper message for int32.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int32value>
            Int32Value,
            /// Wrapper message for int64.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#int64value>
            Int64Value,
            /// ListValue is a wrapper around a repeated field of values.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#listvalue>
            ListValue,
            /// Method represents a method of an api.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#method>
            Method,
            /// Declares an API to be included in this API. The including API
            /// must redeclare all the methods from the included
            /// API, but documentation and options are inherited as
            /// follows:
            ///
            /// If after comment and whitespace stripping, the documentation
            /// string of the redeclared method is empty, it will be
            /// inherited from the original method.
            ///
            /// Each annotation belonging to the service config (http,
            /// visibility) which is not set in the redeclared
            /// method will be inherited.
            ///
            /// If an http annotation is inherited, the path pattern will be
            /// modified as follows. Any version prefix will be
            /// replaced by the version of the including API plus
            /// the root path if specified.
            ///
            /// Example of a simple mixin:
            /// ```protobuf
            /// service AccessControl {
            ///   // Get the underlying ACL object.
            ///   rpc GetAcl(GetAclRequest) returns (Acl) {
            ///     option (google.api.http).get = "/v1/{resource=**}:getAcl";
            ///   }
            /// }
            ///
            /// package google.storage.v2;
            /// service Storage {
            ///   //       rpc GetAcl(GetAclRequest) returns (Acl);
            ///
            ///   // Get a data record.
            ///   rpc GetData(GetDataRequest) returns (Data) {
            ///     option (google.api.http).get = "/v2/{resource=**}";
            ///   }
            /// }
            /// ```
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Mixin>
            Mixin,
            /// A protocol buffer option, which can be attached to a message,
            /// field, enumeration, etc.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#option>
            Option,
            /// SourceContext represents information about the source of a
            /// protobuf element, like the file in which it is
            /// defined.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#sourcecontext>
            SourceContext,
            /// Wrapper message for string.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#stringvalue>
            StringValue,
            /// Struct represents a structured data value, consisting of fields
            /// which map to dynamically typed values. In some
            /// languages, Struct might be supported by a native
            /// representation. For example, in scripting
            /// languages like JS a struct is represented as an object. The
            /// details of that representation are described
            /// together with the proto support for the language.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct>
            Struct,
            /// A Timestamp represents a point in time independent of any time
            /// zone or calendar, represented as seconds and
            /// fractions of seconds at nanosecond resolution in UTC
            /// Epoch time. It is encoded using the Proleptic
            /// Gregorian Calendar which extends the Gregorian calendar
            /// backwards to year one. It is encoded assuming all
            /// minutes are 60 seconds long, i.e. leap seconds are
            /// "smeared" so that no leap second table is needed for
            /// interpretation. Range is from 0001-01-01T00:00:00Z to
            /// 9999-12-31T23:59:59.999999999Z. By restricting to that range, we
            /// ensure that we can convert to and from RFC 3339 date
            /// strings. See <https://www.ietf.org/rfc/rfc3339.txt.>
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#timestamp>
            Timestamp,
            /// A protocol buffer message type.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#type>
            Type,
            /// Wrapper message for uint32.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint32value>
            UInt32Value,
            /// Wrapper message for uint64.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#uint64value>
            UInt64Value,
            /// Value represents a dynamically typed value which can be either
            /// null, a number, a string, a boolean, a recursive
            /// struct value, or a list of values. A producer of
            /// value is expected to set one of that variants,
            /// absence of any variant indicates an error.
            ///
            /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#value>
            Value,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WellKnownMessage {
            #[inline]
            fn clone(&self) -> WellKnownMessage {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WellKnownMessage {}
        #[automatically_derived]
        impl ::core::fmt::Debug for WellKnownMessage {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        WellKnownMessage::Any => "Any",
                        WellKnownMessage::Api => "Api",
                        WellKnownMessage::BoolValue => "BoolValue",
                        WellKnownMessage::BytesValue => "BytesValue",
                        WellKnownMessage::DoubleValue => "DoubleValue",
                        WellKnownMessage::Duration => "Duration",
                        WellKnownMessage::Empty => "Empty",
                        WellKnownMessage::Enum => "Enum",
                        WellKnownMessage::EnumValue => "EnumValue",
                        WellKnownMessage::Field => "Field",
                        WellKnownMessage::FieldKind => "FieldKind",
                        WellKnownMessage::FieldMask => "FieldMask",
                        WellKnownMessage::FloatValue => "FloatValue",
                        WellKnownMessage::Int32Value => "Int32Value",
                        WellKnownMessage::Int64Value => "Int64Value",
                        WellKnownMessage::ListValue => "ListValue",
                        WellKnownMessage::Method => "Method",
                        WellKnownMessage::Mixin => "Mixin",
                        WellKnownMessage::Option => "Option",
                        WellKnownMessage::SourceContext => "SourceContext",
                        WellKnownMessage::StringValue => "StringValue",
                        WellKnownMessage::Struct => "Struct",
                        WellKnownMessage::Timestamp => "Timestamp",
                        WellKnownMessage::Type => "Type",
                        WellKnownMessage::UInt32Value => "UInt32Value",
                        WellKnownMessage::UInt64Value => "UInt64Value",
                        WellKnownMessage::Value => "Value",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WellKnownMessage {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WellKnownMessage {
            #[inline]
            fn eq(&self, other: &WellKnownMessage) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for WellKnownMessage {}
        #[automatically_derived]
        impl ::core::cmp::Eq for WellKnownMessage {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WellKnownMessage {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        impl WellKnownMessage {
            const ANY: &'static str = "Any";
            const API: &'static str = "Api";
            const BOOL_VALUE: &'static str = "BoolValue";
            const BYTES_VALUE: &'static str = "BytesValue";
            const DOUBLE_VALUE: &'static str = "DoubleValue";
            const DURATION: &'static str = "Duration";
            const EMPTY: &'static str = "Empty";
            const ENUM: &'static str = "Enum";
            const ENUM_VALUE: &'static str = "EnumValue";
            const FIELD: &'static str = "Field";
            const FIELD_KIND: &'static str = "FieldKind";
            const FIELD_MASK: &'static str = "FieldMask";
            const FLOAT_VALUE: &'static str = "FloatValue";
            const INT32_VALUE: &'static str = "Int32Value";
            const INT64_VALUE: &'static str = "Int64Value";
            const LIST_VALUE: &'static str = "ListValue";
            const METHOD: &'static str = "Method";
            const MIXIN: &'static str = "Mixin";
            const OPTION: &'static str = "Option";
            const SOURCE_CONTEXT: &'static str = "SourceContext";
            const STRING_VALUE: &'static str = "StringValue";
            const STRUCT: &'static str = "Struct";
            const TIMESTAMP: &'static str = "Timestamp";
            const TYPE: &'static str = "Type";
            const UINT32_VALUE: &'static str = "UInt32Value";
            const UINT64_VALUE: &'static str = "UInt64Value";
            const VALUE: &'static str = "Value";
            pub const fn as_str(self) -> &'static str {
                match self {
                    Self::Any => Self::ANY,
                    Self::Api => Self::API,
                    Self::BoolValue => Self::BOOL_VALUE,
                    Self::BytesValue => Self::BYTES_VALUE,
                    Self::DoubleValue => Self::DOUBLE_VALUE,
                    Self::Duration => Self::DURATION,
                    Self::Empty => Self::EMPTY,
                    Self::Enum => Self::ENUM,
                    Self::EnumValue => Self::ENUM_VALUE,
                    Self::Field => Self::FIELD,
                    Self::FieldKind => Self::FIELD_KIND,
                    Self::FieldMask => Self::FIELD_MASK,
                    Self::FloatValue => Self::FLOAT_VALUE,
                    Self::Int32Value => Self::INT32_VALUE,
                    Self::Int64Value => Self::INT64_VALUE,
                    Self::ListValue => Self::LIST_VALUE,
                    Self::Method => Self::METHOD,
                    Self::Mixin => Self::MIXIN,
                    Self::Option => Self::OPTION,
                    Self::SourceContext => Self::SOURCE_CONTEXT,
                    Self::StringValue => Self::STRING_VALUE,
                    Self::Struct => Self::STRUCT,
                    Self::Timestamp => Self::TIMESTAMP,
                    Self::Type => Self::TYPE,
                    Self::UInt32Value => Self::UINT32_VALUE,
                    Self::UInt64Value => Self::UINT64_VALUE,
                    Self::Value => Self::VALUE,
                }
            }
        }
        impl std::fmt::Display for WellKnownMessage {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fmt.write_str(self.as_str())
            }
        }
        impl std::str::FromStr for WellKnownMessage {
            type Err = ();
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    Self::ANY => Ok(Self::Any),
                    Self::API => Ok(Self::Api),
                    Self::BOOL_VALUE => Ok(Self::BoolValue),
                    Self::BYTES_VALUE => Ok(Self::BytesValue),
                    Self::DOUBLE_VALUE => Ok(Self::DoubleValue),
                    Self::DURATION => Ok(Self::Duration),
                    Self::EMPTY => Ok(Self::Empty),
                    Self::ENUM => Ok(Self::Enum),
                    Self::ENUM_VALUE => Ok(Self::EnumValue),
                    Self::FIELD => Ok(Self::Field),
                    Self::FIELD_KIND => Ok(Self::FieldKind),
                    Self::FIELD_MASK => Ok(Self::FieldMask),
                    Self::FLOAT_VALUE => Ok(Self::FloatValue),
                    Self::INT32_VALUE => Ok(Self::Int32Value),
                    Self::INT64_VALUE => Ok(Self::Int64Value),
                    Self::LIST_VALUE => Ok(Self::ListValue),
                    Self::METHOD => Ok(Self::Method),
                    Self::MIXIN => Ok(Self::Mixin),
                    Self::OPTION => Ok(Self::Option),
                    Self::SOURCE_CONTEXT => Ok(Self::SourceContext),
                    Self::STRING_VALUE => Ok(Self::StringValue),
                    Self::STRUCT => Ok(Self::Struct),
                    Self::TIMESTAMP => Ok(Self::Timestamp),
                    Self::TYPE => Ok(Self::Type),
                    Self::UINT32_VALUE => Ok(Self::UInt32Value),
                    Self::UINT64_VALUE => Ok(Self::UInt64Value),
                    Self::VALUE => Ok(Self::Value),
                    _ => Err(()),
                }
            }
        }
        pub struct ExtensionRange {
            pub start: i32,
            pub end: i32,
            pub uninterpreted_options: Vec<UninterpretedOption>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExtensionRange {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ExtensionRange",
                    "start",
                    &self.start,
                    "end",
                    &self.end,
                    "uninterpreted_options",
                    &&self.uninterpreted_options,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ExtensionRange {
            #[inline]
            fn clone(&self) -> ExtensionRange {
                ExtensionRange {
                    start: ::core::clone::Clone::clone(&self.start),
                    end: ::core::clone::Clone::clone(&self.end),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ExtensionRange {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ExtensionRange {
            #[inline]
            fn eq(&self, other: &ExtensionRange) -> bool {
                self.start == other.start
                    && self.end == other.end
                    && self.uninterpreted_options == other.uninterpreted_options
            }
        }
        impl ExtensionRange {
            pub fn start(&self) -> i32 {
                self.start
            }
            pub fn end(&self) -> i32 {
                self.end
            }
            pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
                &self.uninterpreted_options
            }
        }
        impl access::UninterpretedOptions for ExtensionRange {
            fn uninterpreted_options(&self) -> &[UninterpretedOption] {
                &self.uninterpreted_options
            }
        }
        impl From<descriptor_proto::ExtensionRange> for ExtensionRange {
            fn from(descriptor: descriptor_proto::ExtensionRange) -> Self {
                Self {
                    start: descriptor.start(),
                    end: descriptor.end(),
                    uninterpreted_options: descriptor
                        .options
                        .unwrap_or_default()
                        .uninterpreted_option
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    }
    pub mod method {
        use super::{
            access::NodeKeys,
            file, impl_traits_and_methods,
            location::{Comments, Span},
            message::{self, Message},
            package,
            reference::{ReferenceInner, References},
            resolve::Resolver,
            uninterpreted::UninterpretedOption,
            FullyQualifiedName,
        };
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            node_path: Box<[i32]>,
            span: Span,
            comments: Option<Comments>,
            package: Option<package::Key>,
            file: file::Key,
            name: Box<str>,
            uninterpreted_options: Vec<UninterpretedOption>,
            input: message::Key,
            output: message::Key,
            references: [ReferenceInner; 2],
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "node_path",
                    "span",
                    "comments",
                    "package",
                    "file",
                    "name",
                    "uninterpreted_options",
                    "input",
                    "output",
                    "references",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.package,
                    &self.file,
                    &self.name,
                    &self.uninterpreted_options,
                    &self.input,
                    &self.output,
                    &&self.references,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    input: ::core::default::Default::default(),
                    output: ::core::default::Default::default(),
                    references: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    package: ::core::clone::Clone::clone(&self.package),
                    file: ::core::clone::Clone::clone(&self.file),
                    name: ::core::clone::Clone::clone(&self.name),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    input: ::core::clone::Clone::clone(&self.input),
                    output: ::core::clone::Clone::clone(&self.output),
                    references: ::core::clone::Clone::clone(&self.references),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.node_path == other.node_path
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.package == other.package
                    && self.file == other.file
                    && self.name == other.name
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.input == other.input
                    && self.output == other.output
                    && self.references == other.references
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = super::node::Key> {
                std::iter::empty()
            }
        }
        impl Inner {
            pub(super) fn references_mut(
                &mut self,
            ) -> impl '_ + Iterator<Item = &'_ mut ReferenceInner> {
                self.references.iter_mut()
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
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Method<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Method<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Method<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Method<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Method<'ast> {}
        impl<'ast> PartialEq for Method<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Method<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Method<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Method<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Method<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Method<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Method<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Method<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Method<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Method<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Method<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Method<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Method<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Method<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Method<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Method<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Method<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Method<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Method<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Method<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Method<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
    }
    pub mod node {
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
        use std::fmt;
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
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Key::Package(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Package", &__self_0)
                    }
                    Key::File(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "File", &__self_0)
                    }
                    Key::Message(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                    }
                    Key::Enum(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Enum", &__self_0)
                    }
                    Key::EnumValue(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "EnumValue", &__self_0)
                    }
                    Key::Service(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Service", &__self_0)
                    }
                    Key::Method(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Method", &__self_0)
                    }
                    Key::Field(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Field", &__self_0)
                    }
                    Key::Oneof(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Oneof", &__self_0)
                    }
                    Key::Extension(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Extension", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<package::Key>;
                let _: ::core::clone::AssertParamIsClone<file::Key>;
                let _: ::core::clone::AssertParamIsClone<message::Key>;
                let _: ::core::clone::AssertParamIsClone<r#enum::Key>;
                let _: ::core::clone::AssertParamIsClone<enum_value::Key>;
                let _: ::core::clone::AssertParamIsClone<service::Key>;
                let _: ::core::clone::AssertParamIsClone<method::Key>;
                let _: ::core::clone::AssertParamIsClone<field::Key>;
                let _: ::core::clone::AssertParamIsClone<oneof::Key>;
                let _: ::core::clone::AssertParamIsClone<extension::Key>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Key::Package(__self_0), Key::Package(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::File(__self_0), Key::File(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::Message(__self_0), Key::Message(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::Enum(__self_0), Key::Enum(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::EnumValue(__self_0), Key::EnumValue(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Key::Service(__self_0), Key::Service(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::Method(__self_0), Key::Method(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::Field(__self_0), Key::Field(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::Oneof(__self_0), Key::Oneof(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Key::Extension(__self_0), Key::Extension(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<package::Key>;
                let _: ::core::cmp::AssertParamIsEq<file::Key>;
                let _: ::core::cmp::AssertParamIsEq<message::Key>;
                let _: ::core::cmp::AssertParamIsEq<r#enum::Key>;
                let _: ::core::cmp::AssertParamIsEq<enum_value::Key>;
                let _: ::core::cmp::AssertParamIsEq<service::Key>;
                let _: ::core::cmp::AssertParamIsEq<method::Key>;
                let _: ::core::cmp::AssertParamIsEq<field::Key>;
                let _: ::core::cmp::AssertParamIsEq<oneof::Key>;
                let _: ::core::cmp::AssertParamIsEq<extension::Key>;
            }
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
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Node<'ast> {
            #[inline]
            fn clone(&self) -> Node<'ast> {
                match self {
                    Node::Message(__self_0) => Node::Message(::core::clone::Clone::clone(__self_0)),
                    Node::Oneof(__self_0) => Node::Oneof(::core::clone::Clone::clone(__self_0)),
                    Node::Enum(__self_0) => Node::Enum(::core::clone::Clone::clone(__self_0)),
                    Node::EnumValue(__self_0) => {
                        Node::EnumValue(::core::clone::Clone::clone(__self_0))
                    }
                    Node::Service(__self_0) => Node::Service(::core::clone::Clone::clone(__self_0)),
                    Node::Method(__self_0) => Node::Method(::core::clone::Clone::clone(__self_0)),
                    Node::Field(__self_0) => Node::Field(::core::clone::Clone::clone(__self_0)),
                    Node::Extension(__self_0) => {
                        Node::Extension(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralPartialEq for Node<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::PartialEq for Node<'ast> {
            #[inline]
            fn eq(&self, other: &Node<'ast>) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Node::Message(__self_0), Node::Message(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Node::Oneof(__self_0), Node::Oneof(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Node::Enum(__self_0), Node::Enum(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Node::EnumValue(__self_0), Node::EnumValue(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Node::Service(__self_0), Node::Service(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Node::Method(__self_0), Node::Method(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Node::Field(__self_0), Node::Field(__arg1_0)) => *__self_0 == *__arg1_0,
                        (Node::Extension(__self_0), Node::Extension(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralEq for Node<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::Eq for Node<'ast> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Message<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Oneof<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Enum<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<EnumValue<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Service<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Method<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Field<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Extension<'ast>>;
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
    }
    pub mod oneof {
        use super::{
            access::NodeKeys,
            field, file, impl_traits_and_methods,
            location::{Comments, Span},
            package,
            resolve::Resolver,
            uninterpreted::UninterpretedOption,
            FullyQualifiedName,
        };
        pub struct Oneof<'ast>(Resolver<'ast, Key, Inner>);
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            name: Box<str>,
            package: Option<package::Key>,
            node_path: Box<[i32]>,
            span: Span,
            comments: Option<Comments>,
            file: file::Key,
            uninterpreted_options: Vec<UninterpretedOption>,
            fields: Vec<field::Key>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "name",
                    "package",
                    "node_path",
                    "span",
                    "comments",
                    "file",
                    "uninterpreted_options",
                    "fields",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.name,
                    &self.package,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.file,
                    &self.uninterpreted_options,
                    &&self.fields,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    fields: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    name: ::core::clone::Clone::clone(&self.name),
                    package: ::core::clone::Clone::clone(&self.package),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    file: ::core::clone::Clone::clone(&self.file),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    fields: ::core::clone::Clone::clone(&self.fields),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.name == other.name
                    && self.package == other.package
                    && self.node_path == other.node_path
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.file == other.file
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.fields == other.fields
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = super::node::Key> {
                self.fields.iter().copied().map(super::node::Key::Field)
            }
        }
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Oneof<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Oneof<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Oneof<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Oneof<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Oneof<'ast> {}
        impl<'ast> PartialEq for Oneof<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Oneof<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Oneof<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Oneof<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Oneof<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Oneof<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Oneof<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Oneof<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Oneof<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Oneof<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Oneof<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Oneof<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Oneof<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Oneof<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Oneof<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Oneof<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Oneof<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Oneof<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Oneof<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Oneof<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Oneof<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
    }
    pub mod package {
        use super::{
            access::NodeKeys,
            file::{self, File},
            impl_traits_and_methods, location, resolve, FullyQualifiedName,
        };
        use std::fmt::Debug;
        pub const WELL_KNOWN: &str = "google.protobuf";
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub struct CommentsInner {
            comments: location::Comments,
            defined_in: file::Key,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CommentsInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CommentsInner",
                    "comments",
                    &self.comments,
                    "defined_in",
                    &&self.defined_in,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CommentsInner {
            #[inline]
            fn clone(&self) -> CommentsInner {
                CommentsInner {
                    comments: ::core::clone::Clone::clone(&self.comments),
                    defined_in: ::core::clone::Clone::clone(&self.defined_in),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for CommentsInner {
            #[inline]
            fn default() -> CommentsInner {
                CommentsInner {
                    comments: ::core::default::Default::default(),
                    defined_in: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CommentsInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CommentsInner {
            #[inline]
            fn eq(&self, other: &CommentsInner) -> bool {
                self.comments == other.comments && self.defined_in == other.defined_in
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for CommentsInner {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CommentsInner {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<location::Comments>;
                let _: ::core::cmp::AssertParamIsEq<file::Key>;
            }
        }
        pub struct Comments<'ast> {
            pub comments: location::Comments,
            pub defined_in: File<'ast>,
        }
        impl<'ast> Comments<'ast> {
            pub fn defined_in(&self) -> File<'ast> {
                self.defined_in
            }
            pub fn comments(&self) -> &location::Comments {
                &self.comments
            }
        }
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            comments: Vec<CommentsInner>,
            name: Box<str>,
            is_well_known: bool,
            files: Vec<file::Key>,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.comments == other.comments
                    && self.name == other.name
                    && self.is_well_known == other.is_well_known
                    && self.files == other.files
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    is_well_known: ::core::default::Default::default(),
                    files: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    name: ::core::clone::Clone::clone(&self.name),
                    is_well_known: ::core::clone::Clone::clone(&self.is_well_known),
                    files: ::core::clone::Clone::clone(&self.files),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ =
                    &["key", "fqn", "comments", "name", "is_well_known", "files"];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.comments,
                    &self.name,
                    &self.is_well_known,
                    &&self.files,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        impl Inner {
            pub fn new(name: &str) -> Self {
                Self {
                    key: Key::default(),
                    name: name.into(),
                    is_well_known: name == WELL_KNOWN,
                    files: Vec::default(),
                    fqn: FullyQualifiedName::for_package(name),
                    comments: Vec::default(),
                }
            }
            pub(super) fn fqn(&self) -> &FullyQualifiedName {
                &self.fqn
            }
            pub(super) fn add_file(&mut self, file: file::Key) {
                self.files.push(file);
            }
            pub(super) fn add_comments(
                &mut self,
                comments: location::Comments,
                defined_in: file::Key,
            ) {
                self.comments.push(CommentsInner {
                    comments,
                    defined_in,
                });
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = super::node::Key> {
                self.files.iter().copied().map(Into::into)
            }
        }
        pub struct Package<'ast>(resolve::Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Package<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Package<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Package<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Package<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Package<'ast> {}
        impl<'ast> PartialEq for Package<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Package<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Package<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Package<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Package<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Package<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Package<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Package<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Package<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Package<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Package<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Package<'ast> {
            pub fn is_well_known(&self) -> bool {
                self.0.is_well_known
            }
        }
    }
    pub mod path {
        pub(super) fn new_file_path() -> Vec<i32> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([File::SYNTAX, File::PACKAGE]),
            )
        }
        pub(super) fn append(path: &[i32], kind: impl Into<i32>, index: i32) -> Vec<i32> {
            let mut path = path.to_vec();
            path.reserve(2);
            path.push(kind.into());
            path.push(index);
            path
        }
        pub(super) fn new(kind: impl Into<i32>, index: i32) -> Vec<i32> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([kind.into(), index]),
            )
        }
        #[repr(i32)]
        pub enum File {
            /// file name, relative to root of source tree
            Name = 1,
            /// FileDescriptorProto.package
            Package = 2,
            /// Names of files imported by this file.
            Dependency = 3,
            /// Indexes of the public imported files in the dependency list
            /// above.
            PublicDependency = 10,
            /// Indexes of the weak imported files in the dependency list.
            /// For Google-internal migration only. Do not use.
            WeakDependency = 11,
            Message = 4,
            /// FileDescriptorProto.enum_type
            Enum = 5,
            /// FileDescriptorProto.service
            Service = 6,
            /// FileDescriptorProto.extension
            Extension = 7,
            Options = 8,
            /// This field contains optional information about the original
            /// source code. You may safely remove this entire field
            /// without harming runtime functionality of the
            /// descriptors -- the information is needed only by
            /// development tools.
            SourceCodeInfo = 9,
            /// FileDescriptorProto.syntax
            Syntax = 12,
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for File {
            #[inline]
            fn clone(&self) -> File {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for File {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for File {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for File {
            #[inline]
            fn eq(&self, other: &File) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (File::Unknown(__self_0), File::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for File {}
        #[automatically_derived]
        impl ::core::cmp::Eq for File {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
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
        #[automatically_derived]
        impl ::core::clone::Clone for Message {
            #[inline]
            fn clone(&self) -> Message {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Message {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Message {
            #[inline]
            fn eq(&self, other: &Message) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Message::Unknown(__self_0), Message::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Message {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Message {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Message {}
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
        #[repr(i32)]
        pub enum Enum {
            /// EnumDescriptorProto.Value
            Value = 2,
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Enum {
            #[inline]
            fn clone(&self) -> Enum {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Enum {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Enum {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Enum {
            #[inline]
            fn eq(&self, other: &Enum) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Enum::Unknown(__self_0), Enum::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Enum {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Enum {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        impl From<i32> for Enum {
            fn from(value: i32) -> Self {
                Self::from_i32(value)
            }
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
        #[repr(i32)]
        pub enum Service {
            /// ServiceDescriptorProto.method
            Method = 2,
            Mixin = 6,
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Service {
            #[inline]
            fn clone(&self) -> Service {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Service {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Service {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Service {
            #[inline]
            fn eq(&self, other: &Service) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Service::Unknown(__self_0), Service::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Service {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Service {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
        }
        impl From<i32> for Service {
            fn from(value: i32) -> Self {
                Self::from_i32(value)
            }
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
        #[repr(i32)]
        pub enum Oneof {
            Name = 1,
            Options = 2,
            Unknown(i32),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Oneof {
            #[inline]
            fn clone(&self) -> Oneof {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Oneof {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Oneof {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Oneof {
            #[inline]
            fn eq(&self, other: &Oneof) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Oneof::Unknown(__self_0), Oneof::Unknown(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Oneof {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Oneof {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i32>;
            }
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
    }
    pub mod reference {
        use super::{
            r#enum::{self, Enum},
            extension::{self, Extension},
            field::{self, Field},
            message::{self, Message},
            method::{self, Method},
            Ast,
        };
        use either::Either;
        use std::{iter::Copied, option, slice};
        pub struct Reference<'ast> {
            /// The [`Field`], [`Extension`], or [`Method`] which references the
            /// [`Message`] or [`Enum`].
            pub referrer: Referrer<'ast>,
            /// The [`Message`] or [`Enum`] which is referenced by the
            /// [`Field`], [`Extension`], or [`Method`].
            pub referent: Referent<'ast>,
            /// Indicates wheter the reference is to a [`Message`] or [`Enum`]
            /// in an external file
            pub is_external: bool,
        }
        #[automatically_derived]
        impl<'ast> ::core::fmt::Debug for Reference<'ast> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Reference",
                    "referrer",
                    &self.referrer,
                    "referent",
                    &self.referent,
                    "is_external",
                    &&self.is_external,
                )
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Reference<'ast> {
            #[inline]
            fn clone(&self) -> Reference<'ast> {
                let _: ::core::clone::AssertParamIsClone<Referrer<'ast>>;
                let _: ::core::clone::AssertParamIsClone<Referent<'ast>>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                *self
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::Copy for Reference<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralPartialEq for Reference<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::PartialEq for Reference<'ast> {
            #[inline]
            fn eq(&self, other: &Reference<'ast>) -> bool {
                self.referrer == other.referrer
                    && self.referent == other.referent
                    && self.is_external == other.is_external
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralEq for Reference<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::Eq for Reference<'ast> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Referrer<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Referent<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        impl<'ast> Reference<'ast> {
            /// The [`Field`], [`Extension`], or [`Method`] which references the
            /// [`Message`] or [`Enum`].
            pub fn referrer(self) -> Referrer<'ast> {
                self.referrer
            }
            /// The [`Message`] or [`Enum`] which is referenced by the
            /// [`Field`], [`Extension`], or [`Method`].
            pub fn referent(self) -> Referent<'ast> {
                self.referent
            }
            /// Indicates wheter the reference is to a [`Message`] or [`Enum`]
            /// in an external file
            pub fn is_external(self) -> bool {
                self.is_external
            }
            fn from_inner(inner: ReferenceInner, ast: &'ast Ast) -> Self {
                Self {
                    referrer: Referrer::new(inner.referrer, ast),
                    referent: Referent::new(inner.referent, ast),
                    is_external: inner.is_external,
                }
            }
        }
        pub struct ReferenceInner {
            referrer: ReferrerKey,
            referent: ReferentKey,
            is_external: bool,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ReferenceInner {
            #[inline]
            fn clone(&self) -> ReferenceInner {
                let _: ::core::clone::AssertParamIsClone<ReferrerKey>;
                let _: ::core::clone::AssertParamIsClone<ReferentKey>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ReferenceInner {
            #[inline]
            fn default() -> ReferenceInner {
                ReferenceInner {
                    referrer: ::core::default::Default::default(),
                    referent: ::core::default::Default::default(),
                    is_external: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ReferenceInner {}
        #[automatically_derived]
        impl ::core::fmt::Debug for ReferenceInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ReferenceInner",
                    "referrer",
                    &self.referrer,
                    "referent",
                    &self.referent,
                    "is_external",
                    &&self.is_external,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ReferenceInner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ReferenceInner {
            #[inline]
            fn eq(&self, other: &ReferenceInner) -> bool {
                self.referrer == other.referrer
                    && self.referent == other.referent
                    && self.is_external == other.is_external
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ReferenceInner {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ReferenceInner {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<ReferrerKey>;
                let _: ::core::cmp::AssertParamIsEq<ReferentKey>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        pub(super) enum ReferentKey {
            Message(message::Key),
            Enum(r#enum::Key),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ReferentKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ReferentKey::Message(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                    }
                    ReferentKey::Enum(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Enum", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ReferentKey {
            #[inline]
            fn clone(&self) -> ReferentKey {
                let _: ::core::clone::AssertParamIsClone<message::Key>;
                let _: ::core::clone::AssertParamIsClone<r#enum::Key>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ReferentKey {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ReferentKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ReferentKey {
            #[inline]
            fn eq(&self, other: &ReferentKey) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (ReferentKey::Message(__self_0), ReferentKey::Message(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (ReferentKey::Enum(__self_0), ReferentKey::Enum(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ReferentKey {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ReferentKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<message::Key>;
                let _: ::core::cmp::AssertParamIsEq<r#enum::Key>;
            }
        }
        impl From<r#enum::Key> for ReferentKey {
            fn from(v: r#enum::Key) -> Self {
                Self::Enum(v)
            }
        }
        impl From<message::Key> for ReferentKey {
            fn from(v: message::Key) -> Self {
                Self::Message(v)
            }
        }
        /// The [`Message`] or [`Enum`] which is referenced by the [`Field`],
        /// [`Extension`], or [`Method`].
        ///
        /// [`Referent`] is returne from [`Field::referent`],
        /// [`Extension::referent`], [`Method::input_referent`], and
        /// [`Method::output_referent`]
        pub enum Referent<'ast> {
            Message(Message<'ast>),
            Enum(Enum<'ast>),
        }
        #[automatically_derived]
        impl<'ast> ::core::fmt::Debug for Referent<'ast> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Referent::Message(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                    }
                    Referent::Enum(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Enum", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Referent<'ast> {
            #[inline]
            fn clone(&self) -> Referent<'ast> {
                let _: ::core::clone::AssertParamIsClone<Message<'ast>>;
                let _: ::core::clone::AssertParamIsClone<Enum<'ast>>;
                *self
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::Copy for Referent<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralPartialEq for Referent<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::PartialEq for Referent<'ast> {
            #[inline]
            fn eq(&self, other: &Referent<'ast>) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Referent::Message(__self_0), Referent::Message(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Referent::Enum(__self_0), Referent::Enum(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralEq for Referent<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::Eq for Referent<'ast> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Message<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Enum<'ast>>;
            }
        }
        impl<'ast> Referent<'ast> {
            fn new(key: impl Into<ReferentKey>, ast: &'ast Ast) -> Self {
                match key.into() {
                    ReferentKey::Message(key) => Self::Message(Message::new(key, ast)),
                    ReferentKey::Enum(key) => Self::Enum(Enum::new(key, ast)),
                }
            }
        }
        impl<'ast> From<(r#enum::Key, &'ast Ast)> for Referent<'ast> {
            fn from((key, ast): (r#enum::Key, &'ast Ast)) -> Self {
                Self::Enum((key, ast).into())
            }
        }
        impl<'ast> From<(message::Key, &'ast Ast)> for Referent<'ast> {
            fn from((key, ast): (message::Key, &'ast Ast)) -> Self {
                Self::Message((key, ast).into())
            }
        }
        /// The [`Field`], [`Extension`], or [`Method`] which references a
        /// [`Message`] or [`Enum`].
        pub enum Referrer<'ast> {
            Field(Field<'ast>),
            Extension(Extension<'ast>),
            Method(Method<'ast>),
        }
        #[automatically_derived]
        impl<'ast> ::core::fmt::Debug for Referrer<'ast> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Referrer::Field(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Field", &__self_0)
                    }
                    Referrer::Extension(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Extension", &__self_0)
                    }
                    Referrer::Method(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Method", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::clone::Clone for Referrer<'ast> {
            #[inline]
            fn clone(&self) -> Referrer<'ast> {
                let _: ::core::clone::AssertParamIsClone<Field<'ast>>;
                let _: ::core::clone::AssertParamIsClone<Extension<'ast>>;
                let _: ::core::clone::AssertParamIsClone<Method<'ast>>;
                *self
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::Copy for Referrer<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralPartialEq for Referrer<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::PartialEq for Referrer<'ast> {
            #[inline]
            fn eq(&self, other: &Referrer<'ast>) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Referrer::Field(__self_0), Referrer::Field(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Referrer::Extension(__self_0), Referrer::Extension(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Referrer::Method(__self_0), Referrer::Method(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl<'ast> ::core::marker::StructuralEq for Referrer<'ast> {}
        #[automatically_derived]
        impl<'ast> ::core::cmp::Eq for Referrer<'ast> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Field<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Extension<'ast>>;
                let _: ::core::cmp::AssertParamIsEq<Method<'ast>>;
            }
        }
        impl<'ast> Referrer<'ast> {
            pub(super) fn new(key: impl Into<ReferrerKey>, ast: &'ast Ast) -> Self {
                match key.into() {
                    ReferrerKey::Field(key) => Self::Field(Field::new(key, ast)),
                    ReferrerKey::Extension(key) => Self::Extension(Extension::new(key, ast)),
                    ReferrerKey::Method(key) => Self::Method(Method::new(key, ast)),
                }
            }
        }
        pub(super) enum ReferrerKey {
            Field(field::Key),
            Extension(extension::Key),
            Method(method::Key),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ReferrerKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ReferrerKey::Field(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Field", &__self_0)
                    }
                    ReferrerKey::Extension(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Extension", &__self_0)
                    }
                    ReferrerKey::Method(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Method", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ReferrerKey {
            #[inline]
            fn clone(&self) -> ReferrerKey {
                let _: ::core::clone::AssertParamIsClone<field::Key>;
                let _: ::core::clone::AssertParamIsClone<extension::Key>;
                let _: ::core::clone::AssertParamIsClone<method::Key>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ReferrerKey {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ReferrerKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ReferrerKey {
            #[inline]
            fn eq(&self, other: &ReferrerKey) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (ReferrerKey::Field(__self_0), ReferrerKey::Field(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (ReferrerKey::Extension(__self_0), ReferrerKey::Extension(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (ReferrerKey::Method(__self_0), ReferrerKey::Method(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ReferrerKey {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ReferrerKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<field::Key>;
                let _: ::core::cmp::AssertParamIsEq<extension::Key>;
                let _: ::core::cmp::AssertParamIsEq<method::Key>;
            }
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
        impl From<method::Key> for ReferrerKey {
            fn from(key: method::Key) -> Self {
                Self::Method(key)
            }
        }
        impl<'ast> Referrer<'ast> {
            /// Returns `true` if the referrer is [`Field`].
            ///
            /// [`Field`]: Referrer::Field
            #[must_use]
            pub fn is_field(&self) -> bool {
                match self {
                    Self::Field(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_field(&self) -> Option<&Field<'ast>> {
                if let Self::Field(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            /// Returns `true` if the referrer is [`Extension`].
            ///
            /// [`Extension`]: Referrer::Extension
            #[must_use]
            pub fn is_extension(&self) -> bool {
                match self {
                    Self::Extension(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_extension(&self) -> Option<&Extension<'ast>> {
                if let Self::Extension(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            /// Returns `true` if the referrer is [`Method`].
            ///
            /// [`Method`]: Referrer::Method
            #[must_use]
            pub fn is_method(&self) -> bool {
                match self {
                    Self::Method(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_method(&self) -> Option<&Method<'ast>> {
                if let Self::Method(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
        }
        pub struct References<'ast> {
            ast: &'ast Ast,
            inner:
                Either<Copied<slice::Iter<'ast, ReferenceInner>>, option::IntoIter<ReferenceInner>>,
        }
        impl<'ast> References<'ast> {
            pub(crate) fn from_option(opt: Option<ReferenceInner>, ast: &'ast Ast) -> Self {
                Self {
                    ast,
                    inner: Either::Right(opt.into_iter()),
                }
            }
            pub(crate) fn from_slice(slice: &'ast [ReferenceInner], ast: &'ast Ast) -> Self {
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
    }
    pub mod reserved {
        use protobuf::descriptor;
        pub struct Reserved {
            pub names: Box<[String]>,
            pub ranges: Box<[ReservedRange]>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Reserved {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Reserved",
                    "names",
                    &self.names,
                    "ranges",
                    &&self.ranges,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Reserved {
            #[inline]
            fn clone(&self) -> Reserved {
                Reserved {
                    names: ::core::clone::Clone::clone(&self.names),
                    ranges: ::core::clone::Clone::clone(&self.ranges),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Reserved {
            #[inline]
            fn default() -> Reserved {
                Reserved {
                    names: ::core::default::Default::default(),
                    ranges: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Reserved {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Reserved {
            #[inline]
            fn eq(&self, other: &Reserved) -> bool {
                self.names == other.names && self.ranges == other.ranges
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Reserved {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Reserved {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Box<[String]>>;
                let _: ::core::cmp::AssertParamIsEq<Box<[ReservedRange]>>;
            }
        }
        impl Reserved {
            #[must_use]
            pub fn names(&self) -> &[String] {
                &self.names
            }
            #[must_use]
            pub fn ranges(&self) -> &[ReservedRange] {
                &self.ranges
            }
        }
        pub struct ReservedRange {
            pub start: Option<i32>,
            pub end: Option<i32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ReservedRange {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ReservedRange",
                    "start",
                    &self.start,
                    "end",
                    &&self.end,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ReservedRange {
            #[inline]
            fn clone(&self) -> ReservedRange {
                ReservedRange {
                    start: ::core::clone::Clone::clone(&self.start),
                    end: ::core::clone::Clone::clone(&self.end),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ReservedRange {
            #[inline]
            fn default() -> ReservedRange {
                ReservedRange {
                    start: ::core::default::Default::default(),
                    end: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ReservedRange {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ReservedRange {
            #[inline]
            fn eq(&self, other: &ReservedRange) -> bool {
                self.start == other.start && self.end == other.end
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ReservedRange {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ReservedRange {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
                let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
            }
        }
        impl From<descriptor::descriptor_proto::ReservedRange> for ReservedRange {
            fn from(range: descriptor::descriptor_proto::ReservedRange) -> Self {
                Self {
                    start: range.start,
                    end: range.end,
                }
            }
        }
        impl From<descriptor::enum_descriptor_proto::EnumReservedRange> for ReservedRange {
            fn from(range: descriptor::enum_descriptor_proto::EnumReservedRange) -> Self {
                Self {
                    start: range.start,
                    end: range.end,
                }
            }
        }
        impl ReservedRange {
            #[must_use]
            pub fn start(&self) -> i32 {
                self.start.unwrap_or(0)
            }
            #[must_use]
            pub fn end(&self) -> i32 {
                self.end.unwrap_or(0)
            }
        }
    }
    pub mod service {
        use super::{
            access::NodeKeys, file, impl_traits_and_methods, location, node, package, resolve,
            uninterpreted::UninterpretedOption, FullyQualifiedName,
        };
        #[repr(transparent)]
        pub(super) struct Key(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for Key {}
        #[automatically_derived]
        impl ::core::clone::Clone for Key {
            #[inline]
            fn clone(&self) -> Key {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Key {
            #[inline]
            fn default() -> Key {
                Key(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Key {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Key {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Key {
            #[inline]
            fn eq(&self, other: &Key) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Key {
            #[inline]
            fn cmp(&self, other: &Key) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Key {
            #[inline]
            fn partial_cmp(&self, other: &Key) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Key {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Key {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Key", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for Key {
            fn from(k: ::slotmap::KeyData) -> Self {
                Key(k)
            }
        }
        unsafe impl ::slotmap::Key for Key {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        pub(super) struct Inner {
            key: Key,
            fqn: FullyQualifiedName,
            node_path: Box<[i32]>,
            span: location::Span,
            comments: Option<location::Comments>,
            name: Box<str>,
            package: Option<package::Key>,
            file: file::Key,
            uninterpreted_options: Vec<UninterpretedOption>,
            methods: Vec<super::method::Key>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Inner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "key",
                    "fqn",
                    "node_path",
                    "span",
                    "comments",
                    "name",
                    "package",
                    "file",
                    "uninterpreted_options",
                    "methods",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.key,
                    &self.fqn,
                    &self.node_path,
                    &self.span,
                    &self.comments,
                    &self.name,
                    &self.package,
                    &self.file,
                    &self.uninterpreted_options,
                    &&self.methods,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Inner {
            #[inline]
            fn clone(&self) -> Inner {
                Inner {
                    key: ::core::clone::Clone::clone(&self.key),
                    fqn: ::core::clone::Clone::clone(&self.fqn),
                    node_path: ::core::clone::Clone::clone(&self.node_path),
                    span: ::core::clone::Clone::clone(&self.span),
                    comments: ::core::clone::Clone::clone(&self.comments),
                    name: ::core::clone::Clone::clone(&self.name),
                    package: ::core::clone::Clone::clone(&self.package),
                    file: ::core::clone::Clone::clone(&self.file),
                    uninterpreted_options: ::core::clone::Clone::clone(&self.uninterpreted_options),
                    methods: ::core::clone::Clone::clone(&self.methods),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Inner {
            #[inline]
            fn default() -> Inner {
                Inner {
                    key: ::core::default::Default::default(),
                    fqn: ::core::default::Default::default(),
                    node_path: ::core::default::Default::default(),
                    span: ::core::default::Default::default(),
                    comments: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    package: ::core::default::Default::default(),
                    file: ::core::default::Default::default(),
                    uninterpreted_options: ::core::default::Default::default(),
                    methods: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Inner {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Inner {
            #[inline]
            fn eq(&self, other: &Inner) -> bool {
                self.key == other.key
                    && self.fqn == other.fqn
                    && self.node_path == other.node_path
                    && self.span == other.span
                    && self.comments == other.comments
                    && self.name == other.name
                    && self.package == other.package
                    && self.file == other.file
                    && self.uninterpreted_options == other.uninterpreted_options
                    && self.methods == other.methods
            }
        }
        impl NodeKeys for Inner {
            fn keys(&self) -> impl Iterator<Item = node::Key> {
                self.methods.iter().copied().map(Into::into)
            }
        }
        pub struct Service<'ast>(resolve::Resolver<'ast, Key, Inner>);
        impl crate::ast::access::Key for Inner {
            type Key = Key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl Inner {
            pub(super) fn set_key(&mut self, key: Key) {
                self.key = key;
            }
        }
        impl<'ast> Service<'ast> {
            pub(super) fn new(key: Key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
        impl<'ast> Service<'ast> {
            pub(crate) fn key(self) -> Key {
                self.0.key
            }
        }
        impl<'ast> Service<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for Service<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<'ast> Copy for Service<'ast> {}
        impl<'ast> PartialEq for Service<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for Service<'ast> {}
        impl<'ast> crate::ast::resolve::Resolve<Inner> for Service<'ast> {
            fn resolve(&self) -> &Inner {
                crate::ast::resolve::Resolve::resolve(&self.0)
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Service<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> Service<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::resolve::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for Inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
        impl<'ast> From<(Key, &'ast crate::ast::Ast)> for Service<'ast> {
            fn from((key, ast): (Key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::resolve::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::resolve::Resolver<'ast, Key, Inner>> for Service<'ast> {
            fn from(resolver: crate::ast::resolve::Resolver<'ast, Key, Inner>) -> Self {
                Self(resolver)
            }
        }
        impl<'ast> ::std::fmt::Display for Service<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }
        impl<'ast> ::std::fmt::Debug for Service<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::resolve::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl From<crate::ast::FullyQualifiedName> for Inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for Inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
        impl Inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for Service<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> Service<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> crate::ast::access::File<'ast> for Service<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> Service<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> crate::ast::access::Package<'ast> for Service<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl<'ast> Service<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
        impl Inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
        impl<'ast> crate::ast::access::NodePath for Service<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> Service<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl Inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
        impl<'ast> crate::ast::access::Span for Service<'ast> {
            fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl<'ast> Service<'ast> {
            pub fn span(&self) -> crate::ast::location::Span {
                self.0.span
            }
        }
        impl Inner {
            pub(super) fn set_span(&mut self, span: crate::ast::location::Span) {
                self.span = span;
            }
        }
        impl<'ast> crate::ast::access::Comments for Service<'ast> {
            fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> Service<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::location::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl Inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::location::Comments) {
                self.comments = Some(comments);
            }
        }
        impl Inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
        impl Inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
        impl Inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Detail) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
    }
    pub mod uninterpreted {
        use itertools::Itertools;
        use protobuf::descriptor;
        use std::{fmt, ops::Deref};
        ///  a dot-separated name.
        ///
        ///  E.g.,`{ ["foo", false], ["bar.baz", true], ["qux", false] }`
        /// represents  `"foo.(bar.baz).qux"`.
        pub struct NamePart {
            pub value: Box<str>,
            pub formatted_value: Box<str>,
            pub is_extension: bool,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for NamePart {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for NamePart {
            #[inline]
            fn eq(&self, other: &NamePart) -> bool {
                self.value == other.value
                    && self.formatted_value == other.formatted_value
                    && self.is_extension == other.is_extension
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for NamePart {}
        #[automatically_derived]
        impl ::core::cmp::Eq for NamePart {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Box<str>>;
                let _: ::core::cmp::AssertParamIsEq<Box<str>>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for NamePart {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.value, state);
                ::core::hash::Hash::hash(&self.formatted_value, state);
                ::core::hash::Hash::hash(&self.is_extension, state)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NamePart {
            #[inline]
            fn clone(&self) -> NamePart {
                NamePart {
                    value: ::core::clone::Clone::clone(&self.value),
                    formatted_value: ::core::clone::Clone::clone(&self.formatted_value),
                    is_extension: ::core::clone::Clone::clone(&self.is_extension),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NamePart {
            #[inline]
            fn default() -> NamePart {
                NamePart {
                    value: ::core::default::Default::default(),
                    formatted_value: ::core::default::Default::default(),
                    is_extension: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NamePart {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "NamePart",
                    "value",
                    &self.value,
                    "formatted_value",
                    &self.formatted_value,
                    "is_extension",
                    &&self.is_extension,
                )
            }
        }
        impl NamePart {
            #[must_use]
            pub fn value(&self) -> &str {
                &self.value
            }
            /// true if a segment represents an extension (denoted with
            /// parentheses in  options specs in .proto files).
            #[must_use]
            pub const fn is_extension(&self) -> bool {
                self.is_extension
            }
            /// Returns the formatted value of the `NamePart`
            ///
            /// If `is_extension` is `true`, the formatted value will be wrapped
            /// in parentheses.
            #[must_use]
            pub fn formatted_value(&self) -> &str {
                &self.formatted_value
            }
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.value
            }
        }
        impl AsRef<str> for NamePart {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
        impl fmt::Display for NamePart {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.is_extension {
                    f.write_fmt(format_args!("({0})", self.value()))
                } else {
                    f.write_fmt(format_args!("{0}", self.value()))
                }
            }
        }
        impl From<descriptor::uninterpreted_option::NamePart> for NamePart {
            fn from(part: descriptor::uninterpreted_option::NamePart) -> Self {
                let is_extension = part.is_extension.unwrap_or(false);
                let value: Box<str> = part.name_part.unwrap_or_default().into();
                let formatted_value = if is_extension {
                    {
                        let res = ::alloc::fmt::format(format_args!("({0})", &value));
                        res
                    }
                    .into()
                } else {
                    value.clone()
                };
                Self {
                    value,
                    formatted_value,
                    is_extension,
                }
            }
        }
        impl From<&descriptor::uninterpreted_option::NamePart> for NamePart {
            fn from(part: &descriptor::uninterpreted_option::NamePart) -> Self {
                Self::from(part.clone())
            }
        }
        pub struct NameParts {
            parts: Vec<NamePart>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NameParts {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "NameParts",
                    "parts",
                    &&self.parts,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NameParts {
            #[inline]
            fn clone(&self) -> NameParts {
                NameParts {
                    parts: ::core::clone::Clone::clone(&self.parts),
                }
            }
        }
        impl std::fmt::Display for NameParts {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(format_args!("{0}", self.formatted()))
            }
        }
        impl<'a> std::iter::IntoIterator for &'a NameParts {
            type Item = &'a NamePart;
            type IntoIter = std::slice::Iter<'a, NamePart>;
            fn into_iter(self) -> Self::IntoIter {
                self.parts.iter()
            }
        }
        impl NameParts {
            pub fn iter(&self) -> std::slice::Iter<'_, NamePart> {
                self.parts.iter()
            }
            #[must_use]
            pub fn get(&self, idx: usize) -> Option<&NamePart> {
                self.parts.get(idx)
            }
            #[must_use]
            pub fn len(&self) -> usize {
                self.parts.len()
            }
            #[must_use]
            pub fn is_empty(&self) -> bool {
                self.parts.is_empty()
            }
            #[must_use]
            pub fn contains(&self, part: &str) -> bool {
                self.parts.iter().any(|p| p.value() == part)
            }
            #[must_use]
            pub fn formatted(&self) -> String {
                self.iter().map(NamePart::formatted_value).join(".")
            }
        }
        pub struct UninterpretedOptions {
            options: Vec<UninterpretedOption>,
        }
        impl Deref for UninterpretedOptions {
            type Target = [UninterpretedOption];
            fn deref(&self) -> &Self::Target {
                &self.options
            }
        }
        /// A message representing an option that parser does not recognize.
        pub struct UninterpretedOption {
            name: Box<[NamePart]>,
            value: UninterpretedValue,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UninterpretedOption {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UninterpretedOption",
                    "name",
                    &self.name,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UninterpretedOption {
            #[inline]
            fn clone(&self) -> UninterpretedOption {
                UninterpretedOption {
                    name: ::core::clone::Clone::clone(&self.name),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UninterpretedOption {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UninterpretedOption {
            #[inline]
            fn eq(&self, other: &UninterpretedOption) -> bool {
                self.name == other.name && self.value == other.value
            }
        }
        pub enum UninterpretedValue {
            Identifier(String),
            PositiveInt(u64),
            NegativeInt(i64),
            Double(f64),
            String(Vec<u8>),
            Aggregate(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UninterpretedValue {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    UninterpretedValue::Identifier(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Identifier",
                            &__self_0,
                        )
                    }
                    UninterpretedValue::PositiveInt(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PositiveInt",
                            &__self_0,
                        )
                    }
                    UninterpretedValue::NegativeInt(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "NegativeInt",
                            &__self_0,
                        )
                    }
                    UninterpretedValue::Double(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Double", &__self_0)
                    }
                    UninterpretedValue::String(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "String", &__self_0)
                    }
                    UninterpretedValue::Aggregate(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Aggregate", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UninterpretedValue {
            #[inline]
            fn clone(&self) -> UninterpretedValue {
                match self {
                    UninterpretedValue::Identifier(__self_0) => {
                        UninterpretedValue::Identifier(::core::clone::Clone::clone(__self_0))
                    }
                    UninterpretedValue::PositiveInt(__self_0) => {
                        UninterpretedValue::PositiveInt(::core::clone::Clone::clone(__self_0))
                    }
                    UninterpretedValue::NegativeInt(__self_0) => {
                        UninterpretedValue::NegativeInt(::core::clone::Clone::clone(__self_0))
                    }
                    UninterpretedValue::Double(__self_0) => {
                        UninterpretedValue::Double(::core::clone::Clone::clone(__self_0))
                    }
                    UninterpretedValue::String(__self_0) => {
                        UninterpretedValue::String(::core::clone::Clone::clone(__self_0))
                    }
                    UninterpretedValue::Aggregate(__self_0) => {
                        UninterpretedValue::Aggregate(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for UninterpretedValue {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for UninterpretedValue {
            #[inline]
            fn eq(&self, other: &UninterpretedValue) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            UninterpretedValue::Identifier(__self_0),
                            UninterpretedValue::Identifier(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            UninterpretedValue::PositiveInt(__self_0),
                            UninterpretedValue::PositiveInt(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            UninterpretedValue::NegativeInt(__self_0),
                            UninterpretedValue::NegativeInt(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            UninterpretedValue::Double(__self_0),
                            UninterpretedValue::Double(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            UninterpretedValue::String(__self_0),
                            UninterpretedValue::String(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            UninterpretedValue::Aggregate(__self_0),
                            UninterpretedValue::Aggregate(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        impl UninterpretedValue {
            /// Returns `true` if the uninterpreted option value is
            /// [`Identifier`].
            ///
            /// [`Identifier`]: UninterpretedOptionValue::Identifier
            #[must_use]
            pub fn is_identifier(&self) -> bool {
                match self {
                    Self::Identifier(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_identifier(&self) -> Option<&String> {
                if let Self::Identifier(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            /// Returns `true` if the uninterpreted option value is
            /// [`PositiveInt`].
            ///
            /// [`PositiveInt`]: UninterpretedOptionValue::PositiveInt
            #[must_use]
            pub fn is_positive_int(&self) -> bool {
                match self {
                    Self::PositiveInt(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_positive_int(&self) -> Option<&u64> {
                if let Self::PositiveInt(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_positive_int(self) -> Result<u64, Self> {
                if let Self::PositiveInt(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            pub fn try_into_identifier(self) -> Result<String, Self> {
                if let Self::Identifier(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            /// Returns `true` if the uninterpreted option value is
            /// [`NegativeInt`].
            ///
            /// [`NegativeInt`]: UninterpretedOptionValue::NegativeInt
            #[must_use]
            pub fn is_negative_int(&self) -> bool {
                match self {
                    Self::NegativeInt(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_negative_int(&self) -> Option<&i64> {
                if let Self::NegativeInt(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_negative_int(self) -> Result<i64, Self> {
                if let Self::NegativeInt(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            /// Returns `true` if the uninterpreted option value is [`Double`].
            ///
            /// [`Double`]: UninterpretedOptionValue::Double
            #[must_use]
            pub fn is_double(&self) -> bool {
                match self {
                    Self::Double(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_double(&self) -> Option<&f64> {
                if let Self::Double(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_double(self) -> Result<f64, Self> {
                if let Self::Double(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            /// Returns `true` if the uninterpreted option value is [`String`].
            ///
            /// [`String`]: UninterpretedOptionValue::String
            #[must_use]
            pub fn is_string(&self) -> bool {
                match self {
                    Self::String(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_string(&self) -> Option<&Vec<u8>> {
                if let Self::String(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_string(self) -> Result<Vec<u8>, Self> {
                if let Self::String(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
            /// Returns `true` if the uninterpreted option value is
            /// [`Aggregate`].
            ///
            /// [`Aggregate`]: UninterpretedOptionValue::Aggregate
            #[must_use]
            pub fn is_aggregate(&self) -> bool {
                match self {
                    Self::Aggregate(..) => true,
                    _ => false,
                }
            }
            #[must_use]
            pub fn as_aggregate(&self) -> Option<&String> {
                if let Self::Aggregate(v) = self {
                    Some(v)
                } else {
                    None
                }
            }
            pub fn try_into_aggregate(self) -> Result<String, Self> {
                if let Self::Aggregate(v) = self {
                    Ok(v)
                } else {
                    Err(self)
                }
            }
        }
        impl From<descriptor::UninterpretedOption> for UninterpretedOption {
            fn from(option: descriptor::UninterpretedOption) -> Self {
                let descriptor::UninterpretedOption {
                    name,
                    identifier_value,
                    negative_int_value,
                    double_value,
                    string_value,
                    aggregate_value,
                    positive_int_value,
                    special_fields: _,
                } = option;
                let name = name.into_iter().map(Into::into).collect::<Box<[_]>>();
                let value = if let Some(value) = identifier_value {
                    UninterpretedValue::Identifier(value)
                } else if let Some(value) = positive_int_value {
                    UninterpretedValue::PositiveInt(value)
                } else if let Some(value) = negative_int_value {
                    UninterpretedValue::NegativeInt(value)
                } else if let Some(value) = double_value {
                    UninterpretedValue::Double(value)
                } else if let Some(value) = string_value {
                    UninterpretedValue::String(value)
                } else if let Some(value) = aggregate_value {
                    UninterpretedValue::Aggregate(value)
                } else {
                    UninterpretedValue::PositiveInt(0)
                };
                Self { name, value }
            }
        }
        impl UninterpretedOption {
            #[must_use]
            pub fn name(&self) -> &[NamePart] {
                self.name.as_ref()
            }
            #[must_use]
            pub const fn value(&self) -> &UninterpretedValue {
                &self.value
            }
        }
    }
    mod hydrate {
        use super::{
            container, r#enum, enum_value, extension, extension_block, field,
            file::{self, DependencyInner},
            location, message, method, node, oneof, package, service, Ast, EnumTable,
            EnumValueTable, ExtensionDeclTable, ExtensionTable, FieldTable, FileTable,
            FullyQualifiedName, Hydrated, MessageTable, MethodTable, OneofTable, PackageTable,
            ServiceTable,
        };
        use crate::{error::Error, to_i32, HashMap, Mutex};
        use ahash::HashMapExt;
        use protobuf::descriptor::{
            DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
            FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto,
            ServiceDescriptorProto,
        };
        use std::{
            collections::HashMap,
            iter,
            ops::{Deref, DerefMut},
            str::FromStr,
        };
        type Nodes = Box<dyn Send + Sync + Iterator<Item = (FullyQualifiedName, node::Key)>>;
        pub(crate) fn run(
            file_descriptors: Vec<FileDescriptorProto>,
            targets: &[String],
        ) -> Result<super::Ast, Error> {
            Hydrate::new(file_descriptors, targets).map(Into::into)
        }
        pub(super) struct Hydrate {
            packages: Mutex<PackageTable>,
            files: Mutex<FileTable>,
            messages: Mutex<MessageTable>,
            enums: Mutex<EnumTable>,
            enum_values: Mutex<EnumValueTable>,
            services: Mutex<ServiceTable>,
            methods: Mutex<MethodTable>,
            fields: Mutex<FieldTable>,
            oneofs: Mutex<OneofTable>,
            extensions: Mutex<ExtensionTable>,
            extension_blocks: Mutex<ExtensionDeclTable>,
            well_known_package: package::Key,
        }
        #[automatically_derived]
        impl ::core::default::Default for Hydrate {
            #[inline]
            fn default() -> Hydrate {
                Hydrate {
                    packages: ::core::default::Default::default(),
                    files: ::core::default::Default::default(),
                    messages: ::core::default::Default::default(),
                    enums: ::core::default::Default::default(),
                    enum_values: ::core::default::Default::default(),
                    services: ::core::default::Default::default(),
                    methods: ::core::default::Default::default(),
                    fields: ::core::default::Default::default(),
                    oneofs: ::core::default::Default::default(),
                    extensions: ::core::default::Default::default(),
                    extension_blocks: ::core::default::Default::default(),
                    well_known_package: ::core::default::Default::default(),
                }
            }
        }
        impl Into<Ast> for Hydrate {
            fn into(self) -> Ast {
                Ast {
                    packages: self.packages.take(),
                    files: self.files.take(),
                    messages: self.messages.take(),
                    enums: self.enums.take(),
                    enum_values: self.enum_values.take(),
                    services: self.services.take(),
                    methods: self.methods.take(),
                    fields: self.fields.take(),
                    oneofs: self.oneofs.take(),
                    extensions: self.extensions.take(),
                    extension_blocks: self.extension_blocks.take(),
                    well_known_package: self.well_known_package,
                    nodes: HashMap::default(),
                }
            }
        }
        impl Hydrate {
            fn package_key(&self, fqn: FullyQualifiedName) -> package::Key {
                self.packages.lock().get_or_insert_key(fqn)
            }
            fn file_key(&self, fqn: FullyQualifiedName) -> file::Key {
                self.files.lock().get_or_insert_key(fqn)
            }
            fn message_key(&self, fqn: FullyQualifiedName) -> message::Key {
                self.messages.lock().get_or_insert_key(fqn)
            }
            fn enum_key(&self, fqn: FullyQualifiedName) -> r#enum::Key {
                self.enums.lock().get_or_insert_key(fqn)
            }
            fn enum_value_key(&self, fqn: FullyQualifiedName) -> enum_value::Key {
                self.enum_values.lock().get_or_insert_key(fqn)
            }
            fn service_key(&self, fqn: FullyQualifiedName) -> service::Key {
                self.services.lock().get_or_insert_key(fqn)
            }
            fn method_key(&self, fqn: FullyQualifiedName) -> method::Key {
                self.methods.lock().get_or_insert_key(fqn)
            }
            fn field_key(&self, fqn: FullyQualifiedName) -> field::Key {
                self.fields.lock().get_or_insert_key(fqn)
            }
            fn oneof_key(&self, fqn: FullyQualifiedName) -> oneof::Key {
                self.oneofs.lock().get_or_insert_key(fqn)
            }
            fn extension_key(&self, fqn: FullyQualifiedName) -> extension::Key {
                self.extensions.lock().get_or_insert_key(fqn)
            }
            fn extension_block_key(&self, fqn: FullyQualifiedName) -> extension_block::Key {
                self.extension_blocks.lock().get_or_insert_key(fqn)
            }
        }
        impl Hydrate {
            fn new(
                file_descriptors: Vec<FileDescriptorProto>,
                targets: &[String],
            ) -> Result<Self, Error> {
                Self::default().init(file_descriptors, targets)
            }
            fn init(
                mut self,
                file_descriptors: Vec<FileDescriptorProto>,
                targets: &[String],
            ) -> Result<Self, Error> {
                let well_known = self.hydrate_package(Some("google.protobuf".to_string()));
                let len = file_descriptors.len();
                fn fold_identity() -> (Vec<Hydrated<file::Key>>, Nodes) {
                    (Vec::new(), Box::new(iter::empty()))
                }
                fn reduce_identity() -> (
                    Vec<Hydrated<file::Key>>,
                    HashMap<FullyQualifiedName, node::Key>,
                ) {
                    (Vec::new(), HashMap::default())
                }
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_file(
                &self,
                descriptor: FileDescriptorProto,
                targets: &[String],
            ) -> Result<(Hydrated<file::Key>, Nodes), Error> {
                let name = descriptor.name.unwrap();
                let locations =
                    location::File::new(descriptor.source_code_info.unwrap_or_else(|| {
                        ::core::panicking::panic_fmt(format_args!(
                            "source_code_info not found on FileDescriptorProto for \"{0}\"",
                            name
                        ));
                    }))?;
                let is_build_target = targets.iter().any(|t| t == &name);
                let (package, package_fqn) = self.hydrate_package(descriptor.package);
                let fqn = FullyQualifiedName::new(&name, package_fqn);
                let key = self.file_key(fqn.clone());
                let (messages, message_nodes) = self.hydrate_messages(
                    descriptor.message_type,
                    locations.messages,
                    fqn.clone(),
                    key.into(),
                    key,
                    package,
                )?;
                let (enums, enum_nodes) = self.hydrate_enums(
                    descriptor.enum_type,
                    locations.enums,
                    key.into(),
                    fqn.clone(),
                    key,
                    package,
                );
                let (services, service_nodes) = self.hydrate_services(
                    descriptor.service,
                    locations.services,
                    fqn.clone(),
                    key,
                    package,
                )?;
                let (extension_blocks, extensions, extension_nodes) = self.hydrate_extensions(
                    descriptor.extension,
                    locations.extensions,
                    key.into(),
                    fqn,
                    key,
                    package,
                )?;
                let dependencies = self.hydrate_dependencies(
                    key,
                    descriptor.dependency,
                    descriptor.public_dependency,
                    descriptor.weak_dependency,
                );
                let file = &mut self.files.lock()[key];
                let (key, fqn, name) = file.hydrate(file::Hydrate {
                    name: name.into_boxed_str(),
                    syntax: descriptor.syntax,
                    options: descriptor.options.unwrap_or_default(),
                    package,
                    messages,
                    enums,
                    services,
                    extensions,
                    extension_blocks,
                    dependencies,
                    package_comments: locations.package.and_then(|loc| loc.comments),
                    comments: locations.syntax.and_then(|loc| loc.comments),
                    is_build_target,
                })?;
                let nodes = Box::new(
                    iter::once((fqn.clone(), key.into()))
                        .chain(message_nodes)
                        .chain(enum_nodes)
                        .chain(service_nodes)
                        .chain(extension_nodes),
                );
                Ok(((key, fqn, name), nodes))
            }
            fn hydrate_package(
                &self,
                package: Option<String>,
            ) -> (Option<package::Key>, Option<FullyQualifiedName>) {
                let Some(package) = package else {
                    return (None, None);
                };
                let is_well_known = package == package::WELL_KNOWN;
                if package.is_empty() {
                    return (None, None);
                }
                let fqn = FullyQualifiedName::for_package(&package);
                let (key, pkg) = self.package_key(fqn.clone());
                pkg.set_name(package);
                (Some(key), Some(fqn))
            }
            fn hydrate_dependencies(
                &self,
                dependent: file::Key,
                dependencies_by_fqn: Vec<String>,
                public_dependencies: Vec<i32>,
                weak_dependencies: Vec<i32>,
            ) -> file::DependenciesInner {
                let mut all = Vec::with_capacity(dependencies_by_fqn.len());
                let mut weak = Vec::with_capacity(weak_dependencies.len());
                let mut public = Vec::with_capacity(public_dependencies.len());
                for (i, dependency) in dependencies_by_fqn.into_iter().enumerate() {
                    let index = to_i32(i);
                    let is_weak = weak_dependencies.contains(&index);
                    let is_public = public_dependencies.contains(&index);
                    let fqn = FullyQualifiedName::from(dependency);
                    let (dependency_key, dependency_file) =
                        self.files.lock().get_or_insert(fqn.clone());
                    let dep = DependencyInner {
                        is_used: bool::default(),
                        is_public,
                        is_weak,
                        dependent,
                        dependency: dependency_key,
                    };
                    dependency_file.add_dependent(dep.into());
                    all.push(dep);
                    if is_public {
                        public.push(dep);
                    }
                    if is_weak {
                        weak.push(dep);
                    }
                }
                file::DependenciesInner {
                    all,
                    public,
                    weak,
                    unusued: Vec::default(),
                }
            }
            fn hydrate_messages(
                &self,
                descriptors: Vec<DescriptorProto>,
                locations: Vec<location::Message>,
                container_fqn: FullyQualifiedName,
                container: container::Key,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Vec<Hydrated<message::Key>>, Nodes), Error> {
                assert_message_locations(&container_fqn, &locations, &descriptors);
                let mut messages = Vec::with_capacity(descriptors.len());
                let mut all_nodes = Vec::with_capacity(descriptors.len());
                for (descriptor, location) in descriptors.into_iter().zip(locations) {
                    let fqn =
                        FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                    let ((key, fqn, name), nodes) =
                        self.hydrate_message(descriptor, fqn, location, container, file, package)?;
                    all_nodes.push(iter::once((fqn.clone(), key.into())).chain(nodes));
                    messages.push((key, fqn, name));
                }
                Ok((messages, Box::new(all_nodes.into_iter().flatten())))
            }
            fn is_well_known(&self, package: Option<package::Key>) -> bool {
                if let Some(package) = package {
                    return package == self.well_known_package;
                }
                false
            }
            #[allow(clippy::too_many_arguments)]
            fn hydrate_message(
                &mut self,
                descriptor: DescriptorProto,
                fqn: FullyQualifiedName,
                location: location::Message,
                container: container::Key,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Hydrated<message::Key>, Nodes), Error> {
                let name = descriptor.name.unwrap_or_default().into_boxed_str();
                let key = self.messages.lock().get_or_insert_key(fqn.clone());
                let well_known = if self.is_well_known(package) {
                    message::WellKnownMessage::from_str(&name).ok()
                } else {
                    None
                };
                let (extension_blocks, extensions, extension_nodes) = self.hydrate_extensions(
                    descriptor.extension,
                    location.extensions,
                    key.into(),
                    fqn.clone(),
                    file,
                    package,
                )?;
                let (messages, message_nodes) = self.hydrate_messages(
                    descriptor.nested_type,
                    location.messages,
                    fqn.clone(),
                    key.into(),
                    file,
                    package,
                )?;
                let (enums, enum_nodes) = self.hydrate_enums(
                    descriptor.enum_type,
                    location.enums,
                    key.into(),
                    fqn.clone(),
                    file,
                    package,
                );
                let (oneofs, oneof_nodes) = self.hydrate_oneofs(
                    descriptor.oneof_decl,
                    location.oneofs,
                    key,
                    fqn.clone(),
                    file,
                    package,
                )?;
                let (fields, field_nodes) = self.hydrate_fields(
                    descriptor.field,
                    location.fields,
                    key.into(),
                    fqn,
                    file,
                    package,
                )?;
                let location = location.detail;
                let (key, fqn, name) = self.messages.lock()[key].hydrate(message::Hydrate {
                    name,
                    container,
                    fields,
                    location,
                    messages,
                    oneofs,
                    enums,
                    well_known,
                    extension_blocks,
                    extensions,
                    package,
                    options: descriptor.options,
                    reserved_names: descriptor.reserved_name,
                    reserved_ranges: descriptor.reserved_range,
                    special_fields: descriptor.special_fields,
                    extension_range: descriptor.extension_range,
                });
                let nodes = iter::once((fqn.clone(), key.into()))
                    .chain(message_nodes)
                    .chain(enum_nodes)
                    .chain(oneof_nodes)
                    .chain(field_nodes)
                    .chain(extension_nodes);
                Ok(((key, fqn, name), Box::new(nodes)))
            }
            fn hydrate_enums(
                &self,
                descriptors: Vec<EnumDescriptorProto>,
                locations: Vec<location::Enum>,
                container: container::Key,
                container_fqn: FullyQualifiedName,
                file: file::Key,
                package: Option<package::Key>,
            ) -> (Vec<Hydrated<r#enum::Key>>, Nodes) {
                assert_enum_locations(&container_fqn, &locations, &descriptors);
                let mut enums = Vec::with_capacity(descriptors.len());
                let mut iters = Vec::new();
                for (descriptor, location) in descriptors.into_iter().zip(locations) {
                    let fqn =
                        FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                    let ((key, fqn, name), nodes) = self.hydrate_enum(
                        descriptor,
                        fqn.clone(),
                        location,
                        container,
                        file,
                        package,
                    );
                    iters.push(iter::once((fqn.clone(), key.into())).chain(nodes));
                    enums.push((key, fqn.clone(), name));
                }
                (enums, Box::new(iters.into_iter().flatten()))
            }
            fn hydrate_enum(
                &self,
                descriptor: EnumDescriptorProto,
                fqn: FullyQualifiedName,
                location: location::Enum,
                container: container::Key,
                file: file::Key,
                package: Option<package::Key>,
            ) -> (Hydrated<r#enum::Key>, Nodes) {
                let name = descriptor.name.clone().unwrap_or_default().into_boxed_str();
                let key = self.enums.lock().get_or_insert_key(fqn.clone());
                let (values, values_iter) = self.hydrate_enum_values(
                    descriptor.value,
                    location.values,
                    key,
                    fqn,
                    file,
                    package,
                );
                let well_known = if self.is_well_known(package) {
                    r#enum::WellKnownEnum::from_str(&name).ok()
                } else {
                    None
                };
                let (key, fqn, name) = self.enums.lock()[key].hydrate(r#enum::Hydrate {
                    name,
                    values,
                    container,
                    location: location.detail,
                    options: descriptor.options,
                    reserved_names: descriptor.reserved_name,
                    reserved_ranges: descriptor.reserved_range,
                    special_fields: descriptor.special_fields,
                    well_known,
                });
                let nodes = Box::new(iter::once((fqn.clone(), key.into())).chain(values_iter));
                ((key, fqn, name), nodes)
            }
            fn hydrate_enum_values(
                &self,
                descriptors: Vec<EnumValueDescriptorProto>,
                locations: Vec<location::EnumValue>,
                r#enum: r#enum::Key,
                enum_fqn: FullyQualifiedName,
                file: file::Key,
                package: Option<package::Key>,
            ) -> (Vec<Hydrated<enum_value::Key>>, Nodes) {
                assert_enum_value_locations(&enum_fqn, &locations, &descriptors);
                let mut values = Vec::with_capacity(descriptors.len());
                for (descriptor, location) in descriptors.into_iter().zip(locations) {
                    let fqn = FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone()));
                    let (key, fqn, name) = self.hydrate_enum_value(
                        descriptor,
                        fqn.clone(),
                        location,
                        r#enum,
                        file,
                        package,
                    );
                    values.push((key, fqn, name));
                }
                let nodes = Box::new(
                    values
                        .clone()
                        .into_iter()
                        .map(|(key, fqn, _)| (fqn, key.into())),
                );
                (values, nodes)
            }
            fn hydrate_enum_value(
                &self,
                descriptor: EnumValueDescriptorProto,
                fqn: FullyQualifiedName,
                location: location::EnumValue,
                r#enum: r#enum::Key,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Hydrated<enum_value::Key> {
                let mut enum_values = self.enum_values.lock();
                let key = enum_values.get_or_insert_key(fqn);
                enum_values[key].hydrate(enum_value::Hydrate {
                    name: descriptor.name().into(),
                    number: descriptor.number(),
                    location: location.detail,
                    options: descriptor.options,
                    special_fields: descriptor.special_fields,
                    r#enum,
                    file,
                    package,
                })
            }
            fn hydrate_services(
                &self,
                descriptors: Vec<ServiceDescriptorProto>,
                locations: Vec<location::Service>,
                container_fqn: FullyQualifiedName,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Vec<Hydrated<service::Key>>, Nodes), Error> {
                assert_service_locations(&container_fqn, &locations, &descriptors);
                let mut services = Vec::with_capacity(descriptors.len());
                let mut iters = Vec::with_capacity(descriptors.len());
                for (descriptor, location) in descriptors.into_iter().zip(locations) {
                    let fqn =
                        FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
                    let ((key, fqn, name), nodes) =
                        self.hydrate_service(descriptor, fqn.clone(), location, file, package)?;
                    iters.push(iter::once((fqn.clone(), key.into())).chain(nodes));
                    services.push((key, fqn.clone(), name));
                }
                let nodes = Box::new(iters.into_iter().flatten());
                Ok((services, nodes))
            }
            fn hydrate_service(
                &self,
                descriptor: ServiceDescriptorProto,
                fqn: FullyQualifiedName,
                location: location::Service,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Hydrated<service::Key>, Nodes), Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_methods(
                &self,
                descriptors: Vec<MethodDescriptorProto>,
                locations: Vec<location::Method>,
                container_fqn: FullyQualifiedName,
                container: container::Key,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Vec<Hydrated<method::Key>>, Nodes), Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_method(&self) -> Result<Hydrated<method::Key>, Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_fields(
                &self,
                descriptors: Vec<FieldDescriptorProto>,
                locations: Vec<location::Field>,
                container: container::Key,
                container_fqn: FullyQualifiedName,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Vec<Hydrated<field::Key>>, Nodes), Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_field(&self) -> Result<Hydrated<field::Key>, Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_oneofs(
                &self,
                descriptors: Vec<OneofDescriptorProto>,
                locations: Vec<location::Oneof>,
                message: message::Key,
                message_fqn: FullyQualifiedName,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<(Vec<Hydrated<oneof::Key>>, Nodes), Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_oneof(&self) -> Result<Hydrated<oneof::Key>, Error> {
                ::core::panicking::panic("not yet implemented")
            }
            fn hydrate_extensions(
                &mut self,
                descriptors: Vec<FieldDescriptorProto>,
                locations: Vec<location::ExtensionDecl>,
                container: container::Key,
                container_fqn: FullyQualifiedName,
                file: file::Key,
                package: Option<package::Key>,
            ) -> Result<
                (
                    Vec<extension_block::Key>,
                    Vec<Hydrated<extension::Key>>,
                    Nodes,
                ),
                Error,
            > {
                ::core::panicking::panic("not yet implemented")
            }
        }
        fn assert_enum_locations(
            container_fqn: &str,
            locations: &[location::Enum],
            descriptors: &[EnumDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for enums in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                container_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_enum_value_locations(
            enum_fqn: &str,
            locations: &[location::EnumValue],
            descriptors: &[EnumValueDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for enum values in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                enum_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_message_locations(
            container_fqn: &str,
            locations: &[location::Message],
            descriptors: &[DescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for messages in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                container_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_oneof_locations(
            message_fqn: &str,
            locations: &[location::Oneof],
            descriptors: &[OneofDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for oneofs in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                message_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_service_locations(
            container_fqn: &str,
            locations: &[location::Service],
            descriptors: &[ServiceDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for services in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                container_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_method_locations(
            service_fqn: &str,
            locations: &[location::Method],
            descriptors: &[MethodDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for methods in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                service_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_field_locations(
            message_fqn: &str,
            locations: &[location::Field],
            descriptors: &[FieldDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for fields in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                message_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_extension_locations(
            container_fqn: &str,
            locations: &[location::ExtensionDecl],
            descriptors: &[FieldDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of locations for extensions in \"{2}\", expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len(),
                                container_fqn
                            )),
                        );
                    }
                }
            };
        }
        fn assert_file_locations(
            locations: &[location::File],
            descriptors: &[FileDescriptorProto],
        ) {
            match (&locations.len(), &descriptors.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(format_args!(
                                "invalid number of file locations for files , expected: {0}, found: {1}",
                                descriptors.len(),
                                locations.len()
                            )),
                        );
                    }
                }
            };
        }
    }
    mod resolve {
        use super::Ast;
        use std::fmt;
        #[doc(hidden)]
        pub(super) trait Get<K, T> {
            fn get(&self, key: K) -> &T;
        }
        pub(super) trait Resolve<T> {
            fn resolve(&self) -> &T;
        }
        pub(super) struct Resolver<'ast, K, I> {
            pub(super) ast: &'ast Ast,
            pub(super) key: K,
            pub(super) marker: std::marker::PhantomData<I>,
        }
        impl<'ast, K, I> Resolver<'ast, K, I> {
            pub(super) const fn new(key: K, ast: &'ast Ast) -> Self {
                Self {
                    ast,
                    key,
                    marker: std::marker::PhantomData,
                }
            }
        }
        impl<'ast, K, I> Clone for Resolver<'ast, K, I>
        where
            K: Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    ast: self.ast,
                    key: self.key.clone(),
                    marker: std::marker::PhantomData,
                }
            }
        }
        impl<'ast, K, I> From<(K, &'ast Ast)> for Resolver<'ast, K, I> {
            fn from((key, ast): (K, &'ast Ast)) -> Self {
                Self {
                    ast,
                    key,
                    marker: std::marker::PhantomData,
                }
            }
        }
        impl<'ast, K, I> Copy for Resolver<'ast, K, I> where K: Copy {}
        impl<'ast, K, I> PartialEq for Resolver<'ast, K, I>
        where
            K: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.key == other.key
            }
        }
        impl<'ast, K, I> Eq for Resolver<'ast, K, I> where K: Eq {}
        use super::{
            r#enum, enum_value, extension, extension_block, field, file, message, method, oneof,
            package, service,
        };
        impl Get<package::Key, package::Inner> for Ast {
            fn get(&self, key: package::Key) -> &package::Inner {
                &self.packages[key]
            }
        }
        impl<'ast> Resolve<package::Inner> for Resolver<'ast, package::Key, package::Inner> {
            fn resolve(&self) -> &package::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, package::Key, package::Inner> {
            type Target = package::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, package::Key, package::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<file::Key, file::Inner> for Ast {
            fn get(&self, key: file::Key) -> &file::Inner {
                &self.files[key]
            }
        }
        impl<'ast> Resolve<file::Inner> for Resolver<'ast, file::Key, file::Inner> {
            fn resolve(&self) -> &file::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, file::Key, file::Inner> {
            type Target = file::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, file::Key, file::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<message::Key, message::Inner> for Ast {
            fn get(&self, key: message::Key) -> &message::Inner {
                &self.messages[key]
            }
        }
        impl<'ast> Resolve<message::Inner> for Resolver<'ast, message::Key, message::Inner> {
            fn resolve(&self) -> &message::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, message::Key, message::Inner> {
            type Target = message::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, message::Key, message::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<r#enum::Key, r#enum::Inner> for Ast {
            fn get(&self, key: r#enum::Key) -> &r#enum::Inner {
                &self.enums[key]
            }
        }
        impl<'ast> Resolve<r#enum::Inner> for Resolver<'ast, r#enum::Key, r#enum::Inner> {
            fn resolve(&self) -> &r#enum::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, r#enum::Key, r#enum::Inner> {
            type Target = r#enum::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, r#enum::Key, r#enum::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<enum_value::Key, enum_value::Inner> for Ast {
            fn get(&self, key: enum_value::Key) -> &enum_value::Inner {
                &self.enum_values[key]
            }
        }
        impl<'ast> Resolve<enum_value::Inner> for Resolver<'ast, enum_value::Key, enum_value::Inner> {
            fn resolve(&self) -> &enum_value::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, enum_value::Key, enum_value::Inner> {
            type Target = enum_value::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, enum_value::Key, enum_value::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<oneof::Key, oneof::Inner> for Ast {
            fn get(&self, key: oneof::Key) -> &oneof::Inner {
                &self.oneofs[key]
            }
        }
        impl<'ast> Resolve<oneof::Inner> for Resolver<'ast, oneof::Key, oneof::Inner> {
            fn resolve(&self) -> &oneof::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, oneof::Key, oneof::Inner> {
            type Target = oneof::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, oneof::Key, oneof::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<service::Key, service::Inner> for Ast {
            fn get(&self, key: service::Key) -> &service::Inner {
                &self.services[key]
            }
        }
        impl<'ast> Resolve<service::Inner> for Resolver<'ast, service::Key, service::Inner> {
            fn resolve(&self) -> &service::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, service::Key, service::Inner> {
            type Target = service::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, service::Key, service::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<method::Key, method::Inner> for Ast {
            fn get(&self, key: method::Key) -> &method::Inner {
                &self.methods[key]
            }
        }
        impl<'ast> Resolve<method::Inner> for Resolver<'ast, method::Key, method::Inner> {
            fn resolve(&self) -> &method::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, method::Key, method::Inner> {
            type Target = method::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, method::Key, method::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<field::Key, field::Inner> for Ast {
            fn get(&self, key: field::Key) -> &field::Inner {
                &self.fields[key]
            }
        }
        impl<'ast> Resolve<field::Inner> for Resolver<'ast, field::Key, field::Inner> {
            fn resolve(&self) -> &field::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, field::Key, field::Inner> {
            type Target = field::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, field::Key, field::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<extension::Key, extension::Inner> for Ast {
            fn get(&self, key: extension::Key) -> &extension::Inner {
                &self.extensions[key]
            }
        }
        impl<'ast> Resolve<extension::Inner> for Resolver<'ast, extension::Key, extension::Inner> {
            fn resolve(&self) -> &extension::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, extension::Key, extension::Inner> {
            type Target = extension::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, extension::Key, extension::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
        impl Get<extension_block::Key, extension_block::Inner> for Ast {
            fn get(&self, key: extension_block::Key) -> &extension_block::Inner {
                &self.extension_blocks[key]
            }
        }
        impl<'ast> Resolve<extension_block::Inner>
            for Resolver<'ast, extension_block::Key, extension_block::Inner>
        {
            fn resolve(&self) -> &extension_block::Inner {
                Get::get(self.ast, self.key.clone())
            }
        }
        impl<'ast> ::std::ops::Deref for Resolver<'ast, extension_block::Key, extension_block::Inner> {
            type Target = extension_block::Inner;
            fn deref(&self) -> &Self::Target {
                self.resolve()
            }
        }
        impl<'ast> fmt::Debug for Resolver<'ast, extension_block::Key, extension_block::Inner> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self.resolve(), f)
            }
        }
    }
    use crate::{
        ast::file::{DependencyInner, DependentInner},
        error::Error,
        to_i32, HashMap, HashSet,
    };
    use ahash::HashMapExt;
    use protobuf::descriptor::{
        DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
        FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
    };
    use slotmap::SlotMap;
    use std::{
        fmt,
        iter::once,
        ops::{Deref, Index, IndexMut},
        str::FromStr,
        sync::Arc,
    };
    trait FromFqn {
        fn from_fqn(fqn: FullyQualifiedName) -> Self;
    }
    struct Table<K, V>
    where
        K: slotmap::Key,
    {
        map: SlotMap<K, V>,
        lookup: HashMap<FullyQualifiedName, K>,
        order: Vec<K>,
    }
    #[automatically_derived]
    impl<K: ::core::fmt::Debug, V: ::core::fmt::Debug> ::core::fmt::Debug for Table<K, V>
    where
        K: slotmap::Key,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Table",
                "map",
                &self.map,
                "lookup",
                &self.lookup,
                "order",
                &&self.order,
            )
        }
    }
    #[automatically_derived]
    impl<K: ::core::clone::Clone, V: ::core::clone::Clone> ::core::clone::Clone for Table<K, V>
    where
        K: slotmap::Key,
    {
        #[inline]
        fn clone(&self) -> Table<K, V> {
            Table {
                map: ::core::clone::Clone::clone(&self.map),
                lookup: ::core::clone::Clone::clone(&self.lookup),
                order: ::core::clone::Clone::clone(&self.order),
            }
        }
    }
    impl<K, V> Table<K, V>
    where
        K: slotmap::Key,
    {
        fn with_capacity(len: usize) -> Self {
            Self {
                map: SlotMap::with_capacity_and_key(len),
                lookup: HashMap::with_capacity(len),
                order: Vec::with_capacity(len),
            }
        }
    }
    impl<K, V> Default for Table<K, V>
    where
        K: slotmap::Key,
    {
        fn default() -> Self {
            Self {
                map: SlotMap::with_key(),
                lookup: HashMap::default(),
                order: Vec::default(),
            }
        }
    }
    impl<K, V> Table<K, V>
    where
        K: slotmap::Key,
        V: access::FullyQualifiedName,
    {
        fn get(&self, key: K) -> Option<&V> {
            self.map.get(key)
        }
        fn get_mut(&mut self, key: K) -> Option<&mut V> {
            self.map.get_mut(key)
        }
        fn iter(&self) -> impl Iterator<Item = (K, &V)> {
            self.order.iter().map(move |key| (*key, &self.map[*key]))
        }
        fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
            self.map.iter_mut()
        }
        fn keys(&self) -> impl '_ + Iterator<Item = K> {
            self.order.iter().copied()
        }
        fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
            self.lookup.get(fqn).map(|key| &self.map[*key])
        }
        fn get_mut_by_fqn(&mut self, fqn: &FullyQualifiedName) -> Option<&mut V> {
            self.lookup.get(fqn).map(|key| &mut self.map[*key])
        }
    }
    impl<K, V> Index<K> for Table<K, V>
    where
        K: slotmap::Key,
    {
        type Output = V;
        fn index(&self, key: K) -> &Self::Output {
            &self.map[key]
        }
    }
    impl<K, V> IndexMut<K> for Table<K, V>
    where
        K: slotmap::Key,
    {
        fn index_mut(&mut self, key: K) -> &mut Self::Output {
            &mut self.map[key]
        }
    }
    impl<K, V> Table<K, V>
    where
        K: slotmap::Key,
        V: From<FullyQualifiedName> + access::Key<Key = K>,
    {
        pub fn new() -> Self {
            Self {
                map: SlotMap::with_key(),
                lookup: HashMap::default(),
                order: Vec::new(),
            }
        }
        fn get_or_insert_key(&mut self, fqn: FullyQualifiedName) -> K {
            self.get_or_insert(fqn).0
        }
        fn get_or_insert(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
            let key = *self
                .lookup
                .entry(fqn.clone())
                .or_insert_with(|| self.map.insert(fqn.into()));
            let value = &mut self.map[key];
            if value.key() != key {
                value.set_key(key);
            }
            (key, value)
        }
    }
    type PackageTable = Table<package::Key, package::Inner>;
    type FileTable = Table<file::Key, file::Inner>;
    type MessageTable = Table<message::Key, message::Inner>;
    type EnumTable = Table<r#enum::Key, r#enum::Inner>;
    type EnumValueTable = Table<enum_value::Key, enum_value::Inner>;
    type ServiceTable = Table<service::Key, service::Inner>;
    type MethodTable = Table<method::Key, method::Inner>;
    type FieldTable = Table<field::Key, field::Inner>;
    type OneofTable = Table<oneof::Key, oneof::Inner>;
    type ExtensionTable = Table<extension::Key, extension::Inner>;
    type ExtensionDeclTable = Table<extension_block::Key, extension_block::Inner>;
    struct Set<K> {
        set: Vec<K>,
        by_name: HashMap<Box<str>, K>,
    }
    #[automatically_derived]
    impl<K: ::core::fmt::Debug> ::core::fmt::Debug for Set<K> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Set",
                "set",
                &self.set,
                "by_name",
                &&self.by_name,
            )
        }
    }
    #[automatically_derived]
    impl<K: ::core::clone::Clone> ::core::clone::Clone for Set<K> {
        #[inline]
        fn clone(&self) -> Set<K> {
            Set {
                set: ::core::clone::Clone::clone(&self.set),
                by_name: ::core::clone::Clone::clone(&self.by_name),
            }
        }
    }
    impl<K> Default for Set<K> {
        fn default() -> Self {
            Self {
                set: Vec::default(),
                by_name: HashMap::default(),
            }
        }
    }
    impl<K> Set<K>
    where
        K: Copy,
    {
        fn from_vec(hydrated: Vec<Hydrated<K>>) -> Self {
            let mut set = Vec::with_capacity(hydrated.len());
            let mut by_name = HashMap::with_capacity(hydrated.len());
            for (key, _, name) in hydrated {
                set.push(key);
                by_name.insert(name, key);
            }
            Self { set, by_name }
        }
    }
    impl<K> Set<K>
    where
        K: slotmap::Key + Copy,
    {
        fn by_name(&self, name: &str) -> Option<K> {
            self.by_name.get(name).copied()
        }
        fn get(&self, index: usize) -> Option<K> {
            self.set.get(index).copied()
        }
        fn from_slice(hydrated: &[Hydrated<K>]) -> Self {
            let mut set = Vec::with_capacity(hydrated.len());
            let mut by_name = HashMap::with_capacity(hydrated.len());
            for (key, _, name) in hydrated {
                set.push(*key);
                by_name.insert(name.clone(), *key);
            }
            Self { set, by_name }
        }
    }
    impl<K> From<Vec<Hydrated<K>>> for Set<K>
    where
        K: Copy,
    {
        fn from(v: Vec<Hydrated<K>>) -> Self {
            Self::from_vec(v)
        }
    }
    impl Index<usize> for Set<package::Key> {
        type Output = package::Key;
        fn index(&self, index: usize) -> &Self::Output {
            &self.set[index]
        }
    }
    impl<K> PartialEq for Set<K>
    where
        K: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.set == other.set
        }
    }
    impl<K> Eq for Set<K> where K: PartialEq {}
    impl<K> Deref for Set<K> {
        type Target = [K];
        fn deref(&self) -> &Self::Target {
            &self.set
        }
    }
    /// (key, fqn, name)
    type Hydrated<K> = (K, FullyQualifiedName, Box<str>);
    pub struct Ast {
        packages: PackageTable,
        files: FileTable,
        messages: MessageTable,
        enums: EnumTable,
        enum_values: EnumValueTable,
        services: ServiceTable,
        methods: MethodTable,
        fields: FieldTable,
        oneofs: OneofTable,
        extensions: ExtensionTable,
        extension_blocks: ExtensionDeclTable,
        nodes: HashMap<FullyQualifiedName, node::Key>,
        well_known_package: package::Key,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Ast {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "packages",
                "files",
                "messages",
                "enums",
                "enum_values",
                "services",
                "methods",
                "fields",
                "oneofs",
                "extensions",
                "extension_blocks",
                "nodes",
                "well_known_package",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.packages,
                &self.files,
                &self.messages,
                &self.enums,
                &self.enum_values,
                &self.services,
                &self.methods,
                &self.fields,
                &self.oneofs,
                &self.extensions,
                &self.extension_blocks,
                &self.nodes,
                &&self.well_known_package,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Ast", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Ast {
        #[inline]
        fn default() -> Ast {
            Ast {
                packages: ::core::default::Default::default(),
                files: ::core::default::Default::default(),
                messages: ::core::default::Default::default(),
                enums: ::core::default::Default::default(),
                enum_values: ::core::default::Default::default(),
                services: ::core::default::Default::default(),
                methods: ::core::default::Default::default(),
                fields: ::core::default::Default::default(),
                oneofs: ::core::default::Default::default(),
                extensions: ::core::default::Default::default(),
                extension_blocks: ::core::default::Default::default(),
                nodes: ::core::default::Default::default(),
                well_known_package: ::core::default::Default::default(),
            }
        }
    }
    impl Ast {
        fn new(
            file_descriptors: Vec<FileDescriptorProto>,
            targets: &[String],
        ) -> Result<Self, Error> {
            hydrate::run(file_descriptors, targets)
        }
    }
    pub enum WellKnownType {
        Enum(r#enum::WellKnownEnum),
        Message(message::WellKnownMessage),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WellKnownType {
        #[inline]
        fn clone(&self) -> WellKnownType {
            let _: ::core::clone::AssertParamIsClone<r#enum::WellKnownEnum>;
            let _: ::core::clone::AssertParamIsClone<message::WellKnownMessage>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for WellKnownType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for WellKnownType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                WellKnownType::Enum(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Enum", &__self_0)
                }
                WellKnownType::Message(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Message", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for WellKnownType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for WellKnownType {
        #[inline]
        fn eq(&self, other: &WellKnownType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (WellKnownType::Enum(__self_0), WellKnownType::Enum(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (WellKnownType::Message(__self_0), WellKnownType::Message(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for WellKnownType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for WellKnownType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<r#enum::WellKnownEnum>;
            let _: ::core::cmp::AssertParamIsEq<message::WellKnownMessage>;
        }
    }
    impl WellKnownType {
        pub const PACKAGE: &'static str = "google.protobuf";
    }
    pub struct FullyQualifiedName(Box<str>);
    #[automatically_derived]
    impl ::core::fmt::Debug for FullyQualifiedName {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FullyQualifiedName", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FullyQualifiedName {
        #[inline]
        fn clone(&self) -> FullyQualifiedName {
            FullyQualifiedName(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FullyQualifiedName {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FullyQualifiedName {
        #[inline]
        fn eq(&self, other: &FullyQualifiedName) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FullyQualifiedName {}
    #[automatically_derived]
    impl ::core::cmp::Eq for FullyQualifiedName {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Box<str>>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for FullyQualifiedName {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for FullyQualifiedName {
        #[inline]
        fn partial_cmp(
            &self,
            other: &FullyQualifiedName,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for FullyQualifiedName {
        #[inline]
        fn cmp(&self, other: &FullyQualifiedName) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    impl AsRef<str> for FullyQualifiedName {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl From<Box<str>> for FullyQualifiedName {
        fn from(value: Box<str>) -> Self {
            Self(value)
        }
    }
    impl From<&str> for FullyQualifiedName {
        fn from(value: &str) -> Self {
            Self(value.into())
        }
    }
    impl Deref for FullyQualifiedName {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Default for FullyQualifiedName {
        fn default() -> Self {
            Self("".into())
        }
    }
    impl From<String> for FullyQualifiedName {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
    impl FullyQualifiedName {
        pub fn new(value: impl AsRef<str>, container: Option<Self>) -> Self {
            let value = value.as_ref();
            if value.is_empty() {
                if let Some(fqn) = container {
                    return fqn;
                }
                return Self::default();
            }
            let container = container.unwrap_or_default();
            if container.is_empty() {
                return Self(value.into());
            }
            Self(
                {
                    let res = ::alloc::fmt::format(format_args!("{0}.{1}", container, value));
                    res
                }
                .into(),
            )
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn as_str(&self) -> &str {
            &self.0
        }
        fn push(&mut self, value: &str) {
            if value.is_empty() {
                return;
            }
            let mut existing = self.0.to_string();
            if !self.0.is_empty() {
                existing.push('.');
            }
            existing.push_str(value);
            self.0 = existing.into();
        }
        fn for_package(package: &str) -> Self {
            if package.is_empty() {
                return Self::default();
            }
            if package.starts_with('.') {
                Self(package.into())
            } else {
                Self(
                    {
                        let res = ::alloc::fmt::format(format_args!(".{0}", package));
                        res
                    }
                    .into(),
                )
            }
        }
    }
    impl fmt::Display for FullyQualifiedName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(format_args!("{0}", self.0))
        }
    }
    use self::file::Hydrate;
    use impl_access_file;
    use impl_access_fqn;
    use impl_access_name;
    use impl_access_package;
    use impl_access_reserved;
    use impl_base_traits_and_methods;
    use impl_clone_copy;
    use impl_comments;
    use impl_eq;
    use impl_fmt;
    use impl_from_fqn;
    use impl_from_key_and_ast;
    use impl_key;
    use impl_node_path;
    use impl_resolve;
    use impl_set_uninterpreted_options;
    use impl_span;
    use impl_traits_and_methods;
    use inner_method_file;
    use inner_method_hydrate_location;
    use inner_method_package;
    use node_method_ast;
    use node_method_key;
    use node_method_new;
}
pub mod error {
    use snafu::Snafu;
    pub enum Error {
        #[snafu(display(
            "Unsupported or invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""
        ))]
        UnsupportedSyntax { value: String },
        #[snafu(display(
            "Group field types are deprecated and not supported. Use an embedded message instead."
        ))]
        GroupNotSupported,
        # [snafu (display ("Invalid span: {:?}; path: {:?}; expected a span length of either 3 or 4, found {}" , span , path , span . len ()))]
        InvalidSpan { span: Vec<i32>, path: Vec<i32> },
        #[snafu(display("Missing source code info for {:?}", path))]
        MissingSourceCodeInfo { path: String },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::UnsupportedSyntax { value: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnsupportedSyntax",
                        "value",
                        &__self_0,
                    )
                }
                Error::GroupNotSupported => {
                    ::core::fmt::Formatter::write_str(f, "GroupNotSupported")
                }
                Error::InvalidSpan {
                    span: __self_0,
                    path: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "InvalidSpan",
                    "span",
                    __self_0,
                    "path",
                    &__self_1,
                ),
                Error::MissingSourceCodeInfo { path: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MissingSourceCodeInfo",
                        "path",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Error {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Error {
        #[inline]
        fn eq(&self, other: &Error) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        Error::UnsupportedSyntax { value: __self_0 },
                        Error::UnsupportedSyntax { value: __arg1_0 },
                    ) => *__self_0 == *__arg1_0,
                    (
                        Error::InvalidSpan {
                            span: __self_0,
                            path: __self_1,
                        },
                        Error::InvalidSpan {
                            span: __arg1_0,
                            path: __arg1_1,
                        },
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (
                        Error::MissingSourceCodeInfo { path: __self_0 },
                        Error::MissingSourceCodeInfo { path: __arg1_0 },
                    ) => *__self_0 == *__arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Error {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Error {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Vec<i32>>;
            let _: ::core::cmp::AssertParamIsEq<Vec<i32>>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Error {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Error::UnsupportedSyntax { value: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Error::InvalidSpan {
                    span: __self_0,
                    path: __self_1,
                } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state)
                }
                Error::MissingSourceCodeInfo { path: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                _ => {}
            }
        }
    }
    ///SNAFU context selector for the `Error::UnsupportedSyntax` variant
    struct UnsupportedSyntaxSnafu<__T0> {
        #[allow(missing_docs)]
        value: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for UnsupportedSyntaxSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "UnsupportedSyntaxSnafu",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for UnsupportedSyntaxSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for UnsupportedSyntaxSnafu<__T0> {
        #[inline]
        fn clone(&self) -> UnsupportedSyntaxSnafu<__T0> {
            UnsupportedSyntaxSnafu {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<__T0> UnsupportedSyntaxSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> Error
        where
            __T0: ::core::convert::Into<String>,
        {
            Error::UnsupportedSyntax {
                value: ::core::convert::Into::into(self.value),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, Error>
        where
            __T0: ::core::convert::Into<String>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<Error> for UnsupportedSyntaxSnafu<__T0>
    where
        Error: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> Error {
            Error::UnsupportedSyntax {
                value: ::core::convert::Into::into(self.value),
            }
        }
    }
    ///SNAFU context selector for the `Error::GroupNotSupported` variant
    struct GroupNotSupportedSnafu;
    #[automatically_derived]
    impl ::core::fmt::Debug for GroupNotSupportedSnafu {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "GroupNotSupportedSnafu")
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for GroupNotSupportedSnafu {}
    #[automatically_derived]
    impl ::core::clone::Clone for GroupNotSupportedSnafu {
        #[inline]
        fn clone(&self) -> GroupNotSupportedSnafu {
            *self
        }
    }
    impl GroupNotSupportedSnafu {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> Error {
            Error::GroupNotSupported {}
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, Error> {
            ::core::result::Result::Err(self.build())
        }
    }
    impl ::snafu::IntoError<Error> for GroupNotSupportedSnafu
    where
        Error: ::snafu::Error + ::snafu::ErrorCompat,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> Error {
            Error::GroupNotSupported {}
        }
    }
    ///SNAFU context selector for the `Error::InvalidSpan` variant
    struct InvalidSpanSnafu<__T0, __T1> {
        #[allow(missing_docs)]
        span: __T0,
        #[allow(missing_docs)]
        path: __T1,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for InvalidSpanSnafu<__T0, __T1>
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "InvalidSpanSnafu",
                "span",
                &self.span,
                "path",
                &&self.path,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for InvalidSpanSnafu<__T0, __T1>
    {
    }
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for InvalidSpanSnafu<__T0, __T1>
    {
        #[inline]
        fn clone(&self) -> InvalidSpanSnafu<__T0, __T1> {
            InvalidSpanSnafu {
                span: ::core::clone::Clone::clone(&self.span),
                path: ::core::clone::Clone::clone(&self.path),
            }
        }
    }
    impl<__T0, __T1> InvalidSpanSnafu<__T0, __T1> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> Error
        where
            __T0: ::core::convert::Into<Vec<i32>>,
            __T1: ::core::convert::Into<Vec<i32>>,
        {
            Error::InvalidSpan {
                span: ::core::convert::Into::into(self.span),
                path: ::core::convert::Into::into(self.path),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, Error>
        where
            __T0: ::core::convert::Into<Vec<i32>>,
            __T1: ::core::convert::Into<Vec<i32>>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0, __T1> ::snafu::IntoError<Error> for InvalidSpanSnafu<__T0, __T1>
    where
        Error: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<Vec<i32>>,
        __T1: ::core::convert::Into<Vec<i32>>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> Error {
            Error::InvalidSpan {
                span: ::core::convert::Into::into(self.span),
                path: ::core::convert::Into::into(self.path),
            }
        }
    }
    ///SNAFU context selector for the `Error::MissingSourceCodeInfo` variant
    struct MissingSourceCodeInfoSnafu<__T0> {
        #[allow(missing_docs)]
        path: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for MissingSourceCodeInfoSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "MissingSourceCodeInfoSnafu",
                "path",
                &&self.path,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for MissingSourceCodeInfoSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for MissingSourceCodeInfoSnafu<__T0> {
        #[inline]
        fn clone(&self) -> MissingSourceCodeInfoSnafu<__T0> {
            MissingSourceCodeInfoSnafu {
                path: ::core::clone::Clone::clone(&self.path),
            }
        }
    }
    impl<__T0> MissingSourceCodeInfoSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> Error
        where
            __T0: ::core::convert::Into<String>,
        {
            Error::MissingSourceCodeInfo {
                path: ::core::convert::Into::into(self.path),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, Error>
        where
            __T0: ::core::convert::Into<String>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<Error> for MissingSourceCodeInfoSnafu<__T0>
    where
        Error: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> Error {
            Error::MissingSourceCodeInfo {
                path: ::core::convert::Into::into(self.path),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for Error {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            # [allow (unused_variables)] match * self { Error :: UnsupportedSyntax { ref value } => { __snafu_display_formatter . write_fmt (format_args ! ("Unsupported or invalid syntax: {0:?}; expected either \"proto2\" or \"proto3\"" , value)) } Error :: GroupNotSupported { } => { __snafu_display_formatter . write_fmt (format_args ! ("Group field types are deprecated and not supported. Use an embedded message instead.")) } Error :: InvalidSpan { ref path , ref span } => { __snafu_display_formatter . write_fmt (format_args ! ("Invalid span: {0:?}; path: {1:?}; expected a span length of either 3 or 4, found {2}" , span , path , span . len ())) } Error :: MissingSourceCodeInfo { ref path } => { __snafu_display_formatter . write_fmt (format_args ! ("Missing source code info for {0:?}" , path)) } }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for Error
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Error::UnsupportedSyntax { .. } => "Error :: UnsupportedSyntax",
                Error::GroupNotSupported { .. } => "Error :: GroupNotSupported",
                Error::InvalidSpan { .. } => "Error :: InvalidSpan",
                Error::MissingSourceCodeInfo { .. } => "Error :: MissingSourceCodeInfo",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                Error::UnsupportedSyntax { .. } => ::core::option::Option::None,
                Error::GroupNotSupported { .. } => ::core::option::Option::None,
                Error::InvalidSpan { .. } => ::core::option::Option::None,
                Error::MissingSourceCodeInfo { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                Error::UnsupportedSyntax { .. } => ::core::option::Option::None,
                Error::GroupNotSupported { .. } => ::core::option::Option::None,
                Error::InvalidSpan { .. } => ::core::option::Option::None,
                Error::MissingSourceCodeInfo { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for Error {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Error::UnsupportedSyntax { .. } => ::core::option::Option::None,
                Error::GroupNotSupported { .. } => ::core::option::Option::None,
                Error::InvalidSpan { .. } => ::core::option::Option::None,
                Error::MissingSourceCodeInfo { .. } => ::core::option::Option::None,
            }
        }
    }
    impl Error {
        pub(crate) fn unsupported_syntax(v: impl ToString) -> Self {
            Self::UnsupportedSyntax {
                value: v.to_string(),
            }
        }
        pub(crate) fn invalid_span(loc: &protobuf::descriptor::source_code_info::Location) -> Self {
            Self::InvalidSpan {
                span: loc.span.clone(),
                path: loc.path.clone(),
            }
        }
    }
}
pub mod generator {
    pub struct Version {
        pub major: u32,
        pub minor: u32,
        pub patch: Option<u32>,
        pub prerelease: Option<String>,
        pub build_metadata: Option<String>,
        pub prefix: Option<String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Version {
        #[inline]
        fn clone(&self) -> Version {
            Version {
                major: ::core::clone::Clone::clone(&self.major),
                minor: ::core::clone::Clone::clone(&self.minor),
                patch: ::core::clone::Clone::clone(&self.patch),
                prerelease: ::core::clone::Clone::clone(&self.prerelease),
                build_metadata: ::core::clone::Clone::clone(&self.build_metadata),
                prefix: ::core::clone::Clone::clone(&self.prefix),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Version {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "major",
                "minor",
                "patch",
                "prerelease",
                "build_metadata",
                "prefix",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.major,
                &self.minor,
                &self.patch,
                &self.prerelease,
                &self.build_metadata,
                &&self.prefix,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Version", names, values)
        }
    }
    pub trait Input {
        type Parameter;
        fn files(&self) -> &[protobuf::descriptor::FileDescriptorProto];
        fn protoc_version(&self) -> Option<Version>;
    }
}
type HashMap<K, V> = ahash::HashMap<K, V>;
type HashSet<V> = ahash::HashSet<V>;
fn to_i32<T>(value: T) -> i32
where
    T: TryInto<i32>,
    T::Error: Display,
{
    value.try_into().unwrap_or_else(|err| {
        ::core::panicking::panic_fmt(format_args!("value cannot be converted to i32: {0}", err));
    })
}
struct Mutex<T>(mutex::Inner<T>);
#[automatically_derived]
impl<T: ::core::default::Default> ::core::default::Default for Mutex<T> {
    #[inline]
    fn default() -> Mutex<T> {
        Mutex(::core::default::Default::default())
    }
}
#[automatically_derived]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Mutex<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Mutex", &&self.0)
    }
}
impl<T> Mutex<T> {
    fn new(t: T) -> Self {
        Self(mutex::Inner::new(t))
    }
    fn lock(&self) -> mutex::Guard<'_, T> {
        self.0.lock().expect("mutex poisoned")
    }
}
impl<T> Mutex<T>
where
    T: Default,
{
    fn take(self) -> T {
        let mut lock = self.lock();
        let guard = lock.deref_mut();
        std::mem::take(guard)
    }
}
mod mutex {
    #[cfg(not(feature = "rayon"))]
    pub(super) use fake::{Guard, Inner};
    #[cfg(not(feature = "rayon"))]
    mod fake {
        use std::ops::{Deref, DerefMut};
        pub(crate) struct Guard<'lock, T>(pub &'lock mut T);
        impl<'lock, T> DerefMut for Guard<'lock, T> {
            fn deref_mut(&mut self) -> &mut T {
                self.0
            }
        }
        impl<'lock, T> Deref for Guard<'lock, T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        pub(crate) struct Inner<T>(pub T);
        #[automatically_derived]
        impl<T: ::core::default::Default> ::core::default::Default for Inner<T> {
            #[inline]
            fn default() -> Inner<T> {
                Inner(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Inner<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Inner", &&self.0)
            }
        }
        impl<T> Inner<T> {
            pub(crate) fn new(t: T) -> Self {
                Self(t)
            }
            pub(crate) fn lock(&mut self) -> Result<Guard<'_, T>, ()> {
                Ok(Guard(&mut self.0))
            }
        }
    }
}
