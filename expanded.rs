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
    clippy::struct_excessive_bools
)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use ahash::AHasher;
use std::hash::BuildHasherDefault;
pub mod ast {
    use crate::{
        ast,
        r#enum::{self, Enum, WellKnownEnum},
        enum_value::{self, EnumValue},
        extension::{self, Extension},
        field::{self, Field},
        file::{self, File},
        message::{self, Message, WellKnownMessage},
        method::{self, Method},
        oneof::{self, Oneof},
        package::{self, Package},
        service::{self, Service},
        HashMap,
    };
    use slotmap::SlotMap;
    use std::{borrow::Cow, fmt, ops::Deref};
    mod hydrate {}
    #[doc(hidden)]
    pub(crate) trait Get<K, T> {
        fn get(&self, key: K) -> &T;
    }
    pub(crate) trait Access<T> {
        fn access(&self) -> &T;
    }
    pub(crate) enum Key {
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
                    (Key::EnumValue(__self_0), Key::EnumValue(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Key::Service(__self_0), Key::Service(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Key::Method(__self_0), Key::Method(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Key::Field(__self_0), Key::Field(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Key::Oneof(__self_0), Key::Oneof(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Key::Extension(__self_0), Key::Extension(__arg1_0)) => *__self_0 == *__arg1_0,
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
    pub struct Ast {
        packages: SlotMap<package::Key, package::Inner>,
        files: SlotMap<file::Key, file::Inner>,
        messages: SlotMap<message::Key, message::Inner>,
        enums: SlotMap<r#enum::Key, r#enum::Inner>,
        enum_values: SlotMap<enum_value::Key, enum_value::Inner>,
        services: SlotMap<service::Key, service::Inner>,
        methods: SlotMap<method::Key, method::Inner>,
        fields: SlotMap<field::Key, field::Inner>,
        oneofs: SlotMap<oneof::Key, oneof::Inner>,
        extensions: SlotMap<extension::Key, extension::Inner>,
        nodes: HashMap<FullyQualifiedName, Key>,
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
                "nodes",
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
                &&self.nodes,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Ast", names, values)
        }
    }
    pub(crate) struct Accessor<'ast, K, I> {
        ast: &'ast Ast,
        key: K,
        marker: std::marker::PhantomData<I>,
    }
    impl<'ast, K, I> Accessor<'ast, K, I> {
        pub(crate) fn new(key: K, ast: &'ast Ast) -> Self {
            Self {
                ast,
                key,
                marker: std::marker::PhantomData,
            }
        }
    }
    impl<'ast, K, I> Clone for Accessor<'ast, K, I>
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
    impl<'ast, K, I> From<(K, &'ast Ast)> for Accessor<'ast, K, I> {
        fn from((key, ast): (K, &'ast Ast)) -> Self {
            Self {
                ast,
                key,
                marker: std::marker::PhantomData,
            }
        }
    }
    impl<'ast, K, I> Copy for Accessor<'ast, K, I> where K: Copy {}
    impl<'ast, K, I> PartialEq for Accessor<'ast, K, I>
    where
        K: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }
    impl<'ast, K, I> Eq for Accessor<'ast, K, I> where K: Eq {}
    impl Get<package::Key, package::Inner> for Ast {
        fn get(&self, key: package::Key) -> &package::Inner {
            &self.packages[key]
        }
    }
    impl<'ast> Access<package::Inner> for Accessor<'ast, package::Key, package::Inner> {
        fn access(&self) -> &package::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, package::Key, package::Inner> {
        type Target = package::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, package::Key, package::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<file::Key, file::Inner> for Ast {
        fn get(&self, key: file::Key) -> &file::Inner {
            &self.files[key]
        }
    }
    impl<'ast> Access<file::Inner> for Accessor<'ast, file::Key, file::Inner> {
        fn access(&self) -> &file::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, file::Key, file::Inner> {
        type Target = file::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, file::Key, file::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<message::Key, message::Inner> for Ast {
        fn get(&self, key: message::Key) -> &message::Inner {
            &self.messages[key]
        }
    }
    impl<'ast> Access<message::Inner> for Accessor<'ast, message::Key, message::Inner> {
        fn access(&self) -> &message::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, message::Key, message::Inner> {
        type Target = message::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, message::Key, message::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<r#enum::Key, r#enum::Inner> for Ast {
        fn get(&self, key: r#enum::Key) -> &r#enum::Inner {
            &self.enums[key]
        }
    }
    impl<'ast> Access<r#enum::Inner> for Accessor<'ast, r#enum::Key, r#enum::Inner> {
        fn access(&self) -> &r#enum::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, r#enum::Key, r#enum::Inner> {
        type Target = r#enum::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, r#enum::Key, r#enum::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<oneof::Key, oneof::Inner> for Ast {
        fn get(&self, key: oneof::Key) -> &oneof::Inner {
            &self.oneofs[key]
        }
    }
    impl<'ast> Access<oneof::Inner> for Accessor<'ast, oneof::Key, oneof::Inner> {
        fn access(&self) -> &oneof::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, oneof::Key, oneof::Inner> {
        type Target = oneof::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, oneof::Key, oneof::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<service::Key, service::Inner> for Ast {
        fn get(&self, key: service::Key) -> &service::Inner {
            &self.services[key]
        }
    }
    impl<'ast> Access<service::Inner> for Accessor<'ast, service::Key, service::Inner> {
        fn access(&self) -> &service::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, service::Key, service::Inner> {
        type Target = service::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, service::Key, service::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<method::Key, method::Inner> for Ast {
        fn get(&self, key: method::Key) -> &method::Inner {
            &self.methods[key]
        }
    }
    impl<'ast> Access<method::Inner> for Accessor<'ast, method::Key, method::Inner> {
        fn access(&self) -> &method::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, method::Key, method::Inner> {
        type Target = method::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, method::Key, method::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<field::Key, field::Inner> for Ast {
        fn get(&self, key: field::Key) -> &field::Inner {
            &self.fields[key]
        }
    }
    impl<'ast> Access<field::Inner> for Accessor<'ast, field::Key, field::Inner> {
        fn access(&self) -> &field::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, field::Key, field::Inner> {
        type Target = field::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, field::Key, field::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    impl Get<extension::Key, extension::Inner> for Ast {
        fn get(&self, key: extension::Key) -> &extension::Inner {
            &self.extensions[key]
        }
    }
    impl<'ast> Access<extension::Inner> for Accessor<'ast, extension::Key, extension::Inner> {
        fn access(&self) -> &extension::Inner {
            self.ast.get(self.key.clone())
        }
    }
    impl<'ast> Deref for Accessor<'ast, extension::Key, extension::Inner> {
        type Target = extension::Inner;
        fn deref(&self) -> &Self::Target {
            self.access()
        }
    }
    impl<'ast> fmt::Debug for Accessor<'ast, extension::Key, extension::Inner> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Accessor")
                .field(&self.key)
                .field(self.access())
                .finish()
        }
    }
    pub enum Kind {
        Package,
        File,
        Message,
        Oneof,
        Enum,
        EnumValue,
        Service,
        Method,
        Field,
        Extension,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Kind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Kind::Package => "Package",
                    Kind::File => "File",
                    Kind::Message => "Message",
                    Kind::Oneof => "Oneof",
                    Kind::Enum => "Enum",
                    Kind::EnumValue => "EnumValue",
                    Kind::Service => "Service",
                    Kind::Method => "Method",
                    Kind::Field => "Field",
                    Kind::Extension => "Extension",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Kind {
        #[inline]
        fn clone(&self) -> Kind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Kind {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Kind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Kind {
        #[inline]
        fn eq(&self, other: &Kind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Kind {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Kind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Kind {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    impl fmt::Display for Kind {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Package => fmt.write_fmt(format_args!("Package")),
                Self::File => fmt.write_fmt(format_args!("File")),
                Self::Message => fmt.write_fmt(format_args!("Message")),
                Self::Oneof => fmt.write_fmt(format_args!("Oneof")),
                Self::Enum => fmt.write_fmt(format_args!("Enum")),
                Self::EnumValue => fmt.write_fmt(format_args!("EnumValue")),
                Self::Service => fmt.write_fmt(format_args!("Service")),
                Self::Method => fmt.write_fmt(format_args!("Method")),
                Self::Field => fmt.write_fmt(format_args!("Field")),
                Self::Extension => fmt.write_fmt(format_args!("Extension")),
            }
        }
    }
    pub enum WellKnownType {
        Enum(WellKnownEnum),
        Message(WellKnownMessage),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for WellKnownType {
        #[inline]
        fn clone(&self) -> WellKnownType {
            let _: ::core::clone::AssertParamIsClone<WellKnownEnum>;
            let _: ::core::clone::AssertParamIsClone<WellKnownMessage>;
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
            let _: ::core::cmp::AssertParamIsEq<WellKnownEnum>;
            let _: ::core::cmp::AssertParamIsEq<WellKnownMessage>;
        }
    }
    impl WellKnownType {
        pub const PACKAGE: &'static str = "google.protobuf";
    }
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
    #[automatically_derived]
    impl<'ast> ::core::clone::Clone for Node<'ast> {
        #[inline]
        fn clone(&self) -> Node<'ast> {
            match self {
                Node::Package(__self_0) => Node::Package(::core::clone::Clone::clone(__self_0)),
                Node::File(__self_0) => Node::File(::core::clone::Clone::clone(__self_0)),
                Node::Message(__self_0) => Node::Message(::core::clone::Clone::clone(__self_0)),
                Node::Oneof(__self_0) => Node::Oneof(::core::clone::Clone::clone(__self_0)),
                Node::Enum(__self_0) => Node::Enum(::core::clone::Clone::clone(__self_0)),
                Node::EnumValue(__self_0) => Node::EnumValue(::core::clone::Clone::clone(__self_0)),
                Node::Service(__self_0) => Node::Service(::core::clone::Clone::clone(__self_0)),
                Node::Method(__self_0) => Node::Method(::core::clone::Clone::clone(__self_0)),
                Node::Field(__self_0) => Node::Field(::core::clone::Clone::clone(__self_0)),
                Node::Extension(__self_0) => Node::Extension(::core::clone::Clone::clone(__self_0)),
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
                    (Node::Package(__self_0), Node::Package(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Node::File(__self_0), Node::File(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Node::Message(__self_0), Node::Message(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Node::Oneof(__self_0), Node::Oneof(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Node::Enum(__self_0), Node::Enum(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Node::EnumValue(__self_0), Node::EnumValue(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::Service(__self_0), Node::Service(__arg1_0)) => *__self_0 == *__arg1_0,
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
            let _: ::core::cmp::AssertParamIsEq<Package<'ast>>;
            let _: ::core::cmp::AssertParamIsEq<File<'ast>>;
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
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl Node<'_> {
        pub const fn kind(&self) -> Kind {
            match self {
                Self::Package(_) => Kind::Package,
                Self::File(_) => Kind::File,
                Self::Message(_) => Kind::Message,
                Self::Oneof(_) => Kind::Oneof,
                Self::Enum(_) => Kind::Enum,
                Self::EnumValue(_) => Kind::EnumValue,
                Self::Service(_) => Kind::Service,
                Self::Method(_) => Kind::Method,
                Self::Field(_) => Kind::Field,
                Self::Extension(_) => Kind::Extension,
            }
        }
    }
    pub(crate) struct Nodes<K> {
        fqn_lookup: HashMap<FullyQualifiedName, usize>,
        name_lookup: HashMap<String, usize>,
        list: Vec<K>,
    }
    #[automatically_derived]
    impl<K: ::core::fmt::Debug> ::core::fmt::Debug for Nodes<K> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Nodes",
                "fqn_lookup",
                &self.fqn_lookup,
                "name_lookup",
                &self.name_lookup,
                "list",
                &&self.list,
            )
        }
    }
    #[automatically_derived]
    impl<K: ::core::clone::Clone> ::core::clone::Clone for Nodes<K> {
        #[inline]
        fn clone(&self) -> Nodes<K> {
            Nodes {
                fqn_lookup: ::core::clone::Clone::clone(&self.fqn_lookup),
                name_lookup: ::core::clone::Clone::clone(&self.name_lookup),
                list: ::core::clone::Clone::clone(&self.list),
            }
        }
    }
    #[automatically_derived]
    impl<K> ::core::marker::StructuralPartialEq for Nodes<K> {}
    #[automatically_derived]
    impl<K: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Nodes<K> {
        #[inline]
        fn eq(&self, other: &Nodes<K>) -> bool {
            self.fqn_lookup == other.fqn_lookup
                && self.name_lookup == other.name_lookup
                && self.list == other.list
        }
    }
    impl Default for Nodes<ast::Key> {
        fn default() -> Self {
            Self {
                fqn_lookup: HashMap::default(),
                list: Vec::new(),
                name_lookup: HashMap::default(),
            }
        }
    }
    impl<T> Nodes<T>
    where
        T: Fqn,
    {
        pub fn insert(&mut self, node: T) {
            if self.fqn_lookup.contains_key(node.fully_qualified_name()) {
                return;
            }
            self.fqn_lookup.insert(node.fqn().clone(), self.list.len());
            self.list.push(node);
        }
    }
    impl<T> Nodes<T>
    where
        T: Clone,
    {
        pub fn get(&self, fqn: &FullyQualifiedName) -> Option<T> {
            self.fqn_lookup.get(fqn).map(|i| self.list[*i].clone())
        }
    }
    impl<T> Nodes<T> {
        pub fn new() -> Self {
            Self {
                fqn_lookup: HashMap::default(),
                list: Vec::new(),
                name_lookup: HashMap::default(),
            }
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.list.iter()
        }
    }
    impl Deref for Nodes<ast::Key> {
        type Target = [ast::Key];
        fn deref(&self) -> &Self::Target {
            &self.list
        }
    }
    impl<'ast> AsRef<[Node<'ast>]> for Nodes<Node<'ast>> {
        fn as_ref(&self) -> &[Node<'ast>] {
            &self.list
        }
    }
    /// A trait implemented by all nodes that have a [`FullyQualifiedName`].
    pub trait Fqn {
        /// Returns the [`FullyQualifiedName`] of the node.
        fn fully_qualified_name(&self) -> &FullyQualifiedName;
        /// Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the node.
        fn fqn(&self) -> &FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    pub struct FullyQualifiedName(String);
    #[automatically_derived]
    impl ::core::fmt::Debug for FullyQualifiedName {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FullyQualifiedName", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for FullyQualifiedName {
        #[inline]
        fn default() -> FullyQualifiedName {
            FullyQualifiedName(::core::default::Default::default())
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
            let _: ::core::cmp::AssertParamIsEq<String>;
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
    impl FullyQualifiedName {
        pub fn new(value: impl AsRef<str>, container: Option<FullyQualifiedName>) -> Self {
            let value = value.as_ref();
            if value.is_empty() {
                if let Some(fqn) = container {
                    return fqn;
                }
                return Self::default();
            }
            Self({
                let res = ::alloc::fmt::format(format_args!(
                    "{0}.{1}",
                    container.unwrap_or_default(),
                    &value
                ));
                res
            })
        }
        pub fn as_str(&self) -> &str {
            &self.0
        }
        pub(crate) fn push(&mut self, value: impl AsRef<str>) {
            let value = value.as_ref();
            if value.is_empty() {
                return;
            }
            self.0.push('.');
            self.0.push_str(value);
        }
    }
    impl AsRef<str> for FullyQualifiedName {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl fmt::Display for FullyQualifiedName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(format_args!("{0}", self.0))
        }
    }
    /// A message representing an option that parser does not recognize.
    pub struct UninterpretedOption {
        name: Vec<NamePart>,
        identifier_value: Option<String>,
        positive_int_value: Option<u64>,
        negative_int_value: Option<i64>,
        double_value: Option<f64>,
        string_value: Option<Vec<u8>>,
        aggregate_value: Option<String>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UninterpretedOption {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "name",
                "identifier_value",
                "positive_int_value",
                "negative_int_value",
                "double_value",
                "string_value",
                "aggregate_value",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.name,
                &self.identifier_value,
                &self.positive_int_value,
                &self.negative_int_value,
                &self.double_value,
                &self.string_value,
                &&self.aggregate_value,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "UninterpretedOption",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UninterpretedOption {
        #[inline]
        fn clone(&self) -> UninterpretedOption {
            UninterpretedOption {
                name: ::core::clone::Clone::clone(&self.name),
                identifier_value: ::core::clone::Clone::clone(&self.identifier_value),
                positive_int_value: ::core::clone::Clone::clone(&self.positive_int_value),
                negative_int_value: ::core::clone::Clone::clone(&self.negative_int_value),
                double_value: ::core::clone::Clone::clone(&self.double_value),
                string_value: ::core::clone::Clone::clone(&self.string_value),
                aggregate_value: ::core::clone::Clone::clone(&self.aggregate_value),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UninterpretedOption {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UninterpretedOption {
        #[inline]
        fn eq(&self, other: &UninterpretedOption) -> bool {
            self.name == other.name
                && self.identifier_value == other.identifier_value
                && self.positive_int_value == other.positive_int_value
                && self.negative_int_value == other.negative_int_value
                && self.double_value == other.double_value
                && self.string_value == other.string_value
                && self.aggregate_value == other.aggregate_value
        }
    }
    impl UninterpretedOption {
        #[must_use]
        pub fn name(&self) -> &[NamePart] {
            self.name.as_ref()
        }
        #[must_use]
        pub const fn identifier_value(&self) -> Option<&String> {
            self.identifier_value.as_ref()
        }
        #[must_use]
        pub const fn negative_int_value(&self) -> Option<i64> {
            self.negative_int_value
        }
        #[must_use]
        pub const fn double_value(&self) -> Option<f64> {
            self.double_value
        }
        #[must_use]
        pub fn string_value(&self) -> Option<&[u8]> {
            self.string_value.as_deref()
        }
        #[must_use]
        pub fn aggregate_value(&self) -> Option<&str> {
            self.aggregate_value.as_deref()
        }
    }
    ///  a dot-separated name.
    ///
    ///  E.g.,`{ ["foo", false], ["bar.baz", true], ["qux", false] }` represents
    ///  `"foo.(bar.baz).qux"`.
    pub struct NamePart {
        value: String,
        is_extension: bool,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NamePart {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NamePart {
        #[inline]
        fn eq(&self, other: &NamePart) -> bool {
            self.value == other.value && self.is_extension == other.is_extension
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
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for NamePart {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.value, state);
            ::core::hash::Hash::hash(&self.is_extension, state)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NamePart {
        #[inline]
        fn clone(&self) -> NamePart {
            NamePart {
                value: ::core::clone::Clone::clone(&self.value),
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
                is_extension: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NamePart {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "NamePart",
                "value",
                &self.value,
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
        /// true if a segment represents an extension (denoted with parentheses
        /// in  options specs in .proto files).
        #[must_use]
        pub const fn is_extension(&self) -> bool {
            self.is_extension
        }
        /// Returns the formatted value of the `NamePart`
        ///
        /// If `is_extension` is `true`, the formatted value will be wrapped in
        /// parentheses.
        #[must_use]
        pub fn formatted_value(&self) -> Cow<'_, str> {
            if self.is_extension {
                Cow::Owned({
                    let res = ::alloc::fmt::format(format_args!("({0})", self.value()));
                    res
                })
            } else {
                Cow::Borrowed(self.value())
            }
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
    impl From<protobuf::descriptor::uninterpreted_option::NamePart> for NamePart {
        fn from(part: protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
            Self {
                is_extension: part.is_extension.unwrap_or(false),
                value: part.name_part.unwrap_or_default(),
            }
        }
    }
    impl From<&protobuf::descriptor::uninterpreted_option::NamePart> for NamePart {
        fn from(part: &protobuf::descriptor::uninterpreted_option::NamePart) -> Self {
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
            self.parts.iter().any(|p| p.value == part)
        }
        #[must_use]
        pub fn formatted(&self) -> String {
            itertools::join(self.iter().map(|v| v.formatted_value()), ".")
        }
    }
    pub(crate) use impl_access;
    pub(crate) use impl_copy_clone;
    pub(crate) use impl_eq;
    pub(crate) use impl_fmt;
    pub(crate) use impl_fqn;
    pub(crate) use impl_from_key_and_ast;
    pub(crate) use impl_traits;
}
pub mod container {
    use crate::{file::File, message::Message};
    pub enum Container<'ast> {
        Message(Message<'ast>),
        File(File<'ast>),
    }
}
pub mod r#enum {
    use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};
    use std::fmt;
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Inner", "fqn", &&self.fqn)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Inner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<FullyQualifiedName>;
        }
    }
    pub struct Enum<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Enum<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Enum<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Enum<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Enum<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Enum<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Enum<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Enum<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
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
        /// NullValue is a singleton enumeration to represent the null value for
        /// the Value type union.
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
    impl fmt::Display for WellKnownEnum {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str(self.as_str())
        }
    }
    impl std::str::FromStr for WellKnownEnum {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<WellKnownEnum, Self::Err> {
            match s {
                Self::FIELD_CARDINALITY => Ok(WellKnownEnum::FieldCardinality),
                Self::FIELD_KIND => Ok(WellKnownEnum::FieldKind),
                Self::NULL_VALUE => Ok(WellKnownEnum::NullValue),
                Self::SYNTAX => Ok(WellKnownEnum::Syntax),
                _ => Err(()),
            }
        }
    }
}
pub mod enum_value {
    use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Inner", "fqn", &&self.fqn)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Inner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<FullyQualifiedName>;
        }
    }
    pub struct EnumValue<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for EnumValue<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> EnumValue<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for EnumValue<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for EnumValue<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for EnumValue<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for EnumValue<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for EnumValue<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
}
pub mod error {
    use snafu::Snafu;
    pub enum Error {
        #[snafu(display("Invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""))]
        InvalidSyntax { value: String },
        #[snafu(display(
            "Group field types are deprecated and not supported. Use an embedded message instead."
        ))]
        GroupNotSupported,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::InvalidSyntax { value: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidSyntax",
                        "value",
                        &__self_0,
                    )
                }
                Error::GroupNotSupported => {
                    ::core::fmt::Formatter::write_str(f, "GroupNotSupported")
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
                        Error::InvalidSyntax { value: __self_0 },
                        Error::InvalidSyntax { value: __arg1_0 },
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
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Error {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Error::InvalidSyntax { value: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                _ => {}
            }
        }
    }
    ///SNAFU context selector for the `Error::InvalidSyntax` variant
    struct InvalidSyntaxSnafu<__T0> {
        #[allow(missing_docs)]
        value: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for InvalidSyntaxSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "InvalidSyntaxSnafu",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for InvalidSyntaxSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for InvalidSyntaxSnafu<__T0> {
        #[inline]
        fn clone(&self) -> InvalidSyntaxSnafu<__T0> {
            InvalidSyntaxSnafu {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<__T0> InvalidSyntaxSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> Error
        where
            __T0: ::core::convert::Into<String>,
        {
            Error::InvalidSyntax {
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
    impl<__T0> ::snafu::IntoError<Error> for InvalidSyntaxSnafu<__T0>
    where
        Error: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> Error {
            Error::InvalidSyntax {
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
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for Error {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Error::InvalidSyntax { ref value } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "Invalid syntax: {0:?}; expected either \"proto2\" or \"proto3\"",
                                value
                            ),
                        )
                }
                Error::GroupNotSupported {} => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "Group field types are deprecated and not supported. Use an embedded message instead."
                            ),
                        )
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for Error
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Error::InvalidSyntax { .. } => "Error :: InvalidSyntax",
                Error::GroupNotSupported { .. } => "Error :: GroupNotSupported",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                Error::InvalidSyntax { .. } => ::core::option::Option::None,
                Error::GroupNotSupported { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                Error::InvalidSyntax { .. } => ::core::option::Option::None,
                Error::GroupNotSupported { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for Error {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Error::InvalidSyntax { .. } => ::core::option::Option::None,
                Error::GroupNotSupported { .. } => ::core::option::Option::None,
            }
        }
    }
    impl Error {
        pub(crate) fn invalid_syntax(v: String) -> Self {
            Self::InvalidSyntax { value: v }
        }
    }
}
pub mod extension {
    use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Inner", "fqn", &&self.fqn)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Inner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<FullyQualifiedName>;
        }
    }
    pub struct Extension<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Extension<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Extension<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Extension<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Extension<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Extension<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Extension<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Extension<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
}
pub mod field {
    use crate::{
        ast::{impl_traits, Access, Accessor, Ast, FullyQualifiedName, Get, UninterpretedOption},
        r#enum::{self, Enum},
        error::Error,
        message::{self, Message},
    };
    use ::std::vec::Vec;
    use protobuf::{
        descriptor::{field_descriptor_proto, field_options::CType as ProtobufCType},
        EnumOrUnknown,
    };
    use std::fmt;
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
                    (Label::Unkown(__self_0), Label::Unkown(__arg1_0)) => *__self_0 == *__arg1_0,
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
                    (CType::Unknown(__self_0), CType::Unknown(__arg1_0)) => *__self_0 == *__arg1_0,
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
    impl fmt::Debug for Map<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Map")
                .field("key", &self.key)
                .field("value", &self.value)
                .finish()
        }
    }
    impl Clone for Map<'_> {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Copy for Map<'_> {}
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
                Type::Repeated(__self_0) => Type::Repeated(::core::clone::Clone::clone(__self_0)),
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
    impl Copy for Type<'_> {}
    enum TypeInner {
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
    enum ValueInner {
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
    impl ValueInner {
        fn access_with<'ast>(&self, ast: &'ast Ast) -> Value<'ast> {
            match *self {
                ValueInner::Scalar(s) => Value::Scalar(s),
                ValueInner::Enum(key) => (key, ast).into(),
                ValueInner::Message(key) => (key, ast).into(),
                ValueInner::Unknown(u) => Value::Unknown(u),
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
                    (Value::Scalar(__self_0), Value::Scalar(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Value::Enum(__self_0), Value::Enum(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Value::Message(__self_0), Value::Message(__arg1_0)) => *__self_0 == *__arg1_0,
                    (Value::Unknown(__self_0), Value::Unknown(__arg1_0)) => *__self_0 == *__arg1_0,
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
    impl<'ast> From<(message::Key, &'ast Ast)> for Value<'ast> {
        fn from((key, ast): (message::Key, &'ast Ast)) -> Self {
            Self::from((key, ast))
        }
    }
    impl<'ast> From<(r#enum::Key, &'ast Ast)> for Value<'ast> {
        fn from((key, ast): (r#enum::Key, &'ast Ast)) -> Self {
            Self::from((key, ast))
        }
    }
    impl<'ast> Clone for Value<'ast> {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<'ast> Copy for Value<'ast> {}
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
    impl Value<'_> {
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
        pub const fn as_unknown(&self) -> Option<&i32> {
            if let Self::Unknown(v) = self {
                Some(v)
            } else {
                None
            }
        }
    }
    impl ValueInner {
        pub(crate) fn new(
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
        name: String,
        number: i32,
        label: Option<Label>,
        ///  If type_name is set, this need not be set.  If both this and
        /// type_name  are set, this must be one of TYPE_ENUM,
        /// TYPE_MESSAGE or TYPE_GROUP.
        field_type: TypeInner,
        ///  For message and enum types, this is the name of the type.  If the
        /// name  starts with a '.', it is fully-qualified.  Otherwise,
        /// C++-like scoping  rules are used to find the type (i.e.
        /// first the nested types within this  message are searched,
        /// then within the parent, on up to the root  namespace).
        type_name: Option<String>,
        ///  For extensions, this is the name of the type being extended.  It is
        ///  resolved in the same manner as type_name.
        extendee: Option<String>,
        ///  For numeric types, contains the original text representation of the
        /// value.  For booleans, "true" or "false".
        ///  For strings, contains the default text contents (not escaped in any
        /// way).  For bytes, contains the C escaped value.  All bytes >= 128
        /// are escaped.  TODO(kenton):  Base-64 encode?
        default_value: Option<String>,
        ///  If set, gives the index of a oneof in the containing type's
        /// oneof_decl  list.  This field is a member of that oneof.
        oneof_index: Option<i32>,
        ///  JSON name of this field. The value is set by protocol compiler. If
        /// the  user has set a "json_name" option on this field, that
        /// option's value  will be used. Otherwise, it's deduced from
        /// the field's name by converting  it to camelCase.
        json_name: Option<String>,
        ///  The ctype option instructs the C++ code generator to use a
        /// different  representation of the field than it normally
        /// would.  See the specific  options below.  This option is not
        /// yet implemented in the open source  release -- sorry, we'll
        /// try to include it in a future version!
        ctype: Option<CType>,
        ///  The packed option can be enabled for repeated primitive fields to
        /// enable  a more efficient representation on the wire. Rather than
        /// repeatedly  writing the tag and type for each element, the entire
        /// array is encoded as  a single length-delimited blob. In proto3, only
        /// explicit setting it to  false will avoid using packed encoding.
        packed: bool,
        ///  The jstype option determines the JavaScript type used for values of
        /// the  field.  The option is permitted only for 64 bit
        /// integral and fixed types  (int64, uint64, sint64, fixed64,
        /// sfixed64).  A field with jstype JS_STRING  is represented as
        /// JavaScript string, which avoids loss of precision that  can
        /// happen when a large value is converted to a floating point
        /// JavaScript.  Specifying JS_NUMBER for the jstype causes the
        /// generated JavaScript code to  use the JavaScript "number"
        /// type.  The behavior of the default option  JS_NORMAL is
        /// implementation dependent.
        ///
        ///  This option is an enum to permit additional types to be added, e.g.
        ///  goog.math.Integer.
        jstype: Option<JsType>,
        ///  Should this field be parsed lazily?  Lazy applies only to
        /// message-type  fields.  It means that when the outer message
        /// is initially parsed, the  inner message's contents will not
        /// be parsed but instead stored in encoded  form.  The inner
        /// message will actually be parsed when it is first accessed.
        ///
        ///  This is only a hint.  Implementations are free to choose whether to
        /// use  eager or lazy parsing regardless of the value of this
        /// option.  However,  setting this option true suggests that
        /// the protocol author believes that  using lazy parsing on
        /// this field is worth the additional bookkeeping  overhead
        /// typically needed to implement it.
        ///
        ///  This option does not affect the public interface of any generated
        /// code;  all method signatures remain the same.  Furthermore,
        /// thread-safety of the  interface is not affected by this
        /// option; const methods remain safe to  call from multiple
        /// threads concurrently, while non-const methods continue  to
        /// require exclusive access.
        ///
        ///
        ///  Note that implementations may choose not to check required fields
        /// within  a lazy sub-message.  That is, calling IsInitialized() on the
        /// outer message  may return true even if the inner message has missing
        /// required fields.  This is necessary because otherwise the inner
        /// message would have to be  parsed in order to perform the check,
        /// defeating the purpose of lazy  parsing.  An implementation which
        /// chooses not to check required fields  must be consistent about it.
        /// That is, for any particular sub-message, the  implementation must
        /// either *always* check its required fields, or *never*  check its
        /// required fields, regardless of whether or not the message has
        ///  been parsed.
        lazy: bool,
        ///  Is this field deprecated?
        ///  Depending on the target platform, this can emit Deprecated
        /// annotations  for accessors, or it will be completely
        /// ignored; in the very least, this  is a formalization for
        /// deprecating fields.
        deprecated: bool,
        ///  For Google-internal migration only. Do not use.
        weak: bool,
        ///  The parser stores options it doesn't recognize here. See above.
        uninterpreted_option: Vec<UninterpretedOption>,
        ///  If true, this is a proto3 "optional". When a proto3 field is
        /// optional, it  tracks presence regardless of field type.
        ///
        ///  When proto3_optional is true, this field must be belong to a oneof
        /// to  signal to old proto3 clients that presence is tracked
        /// for this field. This  oneof is known as a "synthetic" oneof,
        /// and this field must be its sole  member (each proto3
        /// optional field gets its own synthetic oneof). Synthetic
        /// oneofs exist in the descriptor only, and do not generate any
        /// API. Synthetic  oneofs must be ordered after all "real"
        /// oneofs.
        ///
        ///  For message fields, proto3_optional doesn't create any semantic
        /// change,  since non-repeated message fields always track
        /// presence. However it still  indicates the semantic detail of
        /// whether the user wrote "optional" or not.  This can be
        /// useful for round-tripping the .proto file. For consistency
        /// we  give message fields a synthetic oneof also, even though
        /// it is not required  to track presence. This is especially
        /// important because the parser can't  tell if a field is a
        /// message or an enum, so it must always create a  synthetic oneof.
        ///
        ///  Proto2 optional fields do not set this flag, because they already
        /// indicate  optional with `LABEL_OPTIONAL`.
        pub proto3_optional: Option<bool>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "fqn",
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
                "uninterpreted_option",
                "proto3_optional",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.fqn,
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
                &self.uninterpreted_option,
                &&self.proto3_optional,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
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
                uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
                proto3_optional: ::core::clone::Clone::clone(&self.proto3_optional),
            }
        }
    }
    pub struct Field<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Field<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Field<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Field<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Field<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Field<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Field<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Field<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
}
pub mod file {
    use crate::{
        ast::{impl_traits, Accessor, Ast, FullyQualifiedName, Get, Nodes, UninterpretedOption},
        error::Error,
        message, package, HashSet,
    };
    use std::{
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
    pub struct File<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for File<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> File<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for File<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for File<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for File<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for File<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for File<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
    impl<'ast> File<'ast> {
        #[must_use]
        pub fn name(&self) -> &str {
            self.0.name.as_ref()
        }
        #[must_use]
        pub fn path(&self) -> &Path {
            self.0.path.as_ref()
        }
        #[must_use]
        pub fn package(&self) -> &str {
            self.0.pkg_name.as_ref()
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
        /// annotations  for everything in the file, or it will be
        /// completely ignored; in the very  least, this is a
        /// formalization for deprecating files.
        #[must_use]
        pub fn deprecated(&self) -> bool {
            self.0.deprecated
        }
        ///  Enables the use of arenas for the proto messages in this file. This
        /// applies  only to generated classes for C++.
        #[must_use]
        pub fn cc_enable_arenas(&self) -> bool {
            self.0.cc_enable_arenas
        }
        ///  Sets the objective c class prefix which is prepended to all
        /// objective c  generated classes from this .proto. There is no
        /// default.
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
        /// CamelCase it  replacing '.' with underscore and use that to
        /// prefix the types/symbols  defined. When this options is
        /// provided, they will use this value instead  to prefix the
        /// types/symbols defined.
        #[must_use]
        pub fn swift_prefix(&self) -> Option<&str> {
            self.0.swift_prefix.as_deref()
        }
        ///  Sets the php class prefix which is prepended to all php generated
        /// classes  from this .proto. Default is empty.
        #[must_use]
        pub fn php_class_prefix(&self) -> Option<&str> {
            self.0.php_class_prefix.as_deref()
        }
        ///  Use this option to change the namespace of php generated classes.
        /// Default  is empty. When this option is empty, the package name will
        /// be used for  determining the namespace.
        #[must_use]
        pub fn php_namespace(&self) -> Option<&str> {
            self.0.php_namespace.as_deref()
        }
        ///  Use this option to change the namespace of php generated metadata
        /// classes.  Default is empty. When this option is empty, the proto
        /// file name will be  used for determining the namespace.
        #[must_use]
        pub fn php_metadata_namespace(&self) -> Option<&str> {
            self.0.php_metadata_namespace.as_deref()
        }
        ///  Use this option to change the package of ruby generated classes.
        /// Default  is empty. When this option is not set, the package name
        /// will be used for  determining the ruby package.
        #[must_use]
        pub fn ruby_package(&self) -> Option<&str> {
            self.0.ruby_package.as_deref()
        }
        ///  The parser stores options it doesn't recognize here.
        ///  See the documentation for the "Options" section above.
        #[must_use]
        pub fn uninterpreted_option(&self) -> &[UninterpretedOption] {
            &self.0.uninterpreted_option
        }
    }
    /// Syntax of the proto file. Lorem ipsum dolor sit amet, consectetur
    /// adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim
    /// sit amet, adipiscing nec, ultricies sed, dolor.
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
    impl Default for Syntax {
        fn default() -> Self {
            Self::Proto2
        }
    }
    impl Syntax {
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
    }
    impl FromStr for Syntax {
        type Err = Error;
        fn from_str(v: &str) -> Result<Self, Self::Err> {
            match &*v.to_lowercase() {
                "proto2" | "" => Ok(Self::Proto2),
                "proto3" => Ok(Self::Proto3),
                _ => Err(Error::invalid_syntax(v.to_string())),
            }
        }
    }
    impl TryFrom<&str> for Syntax {
        type Error = Error;
        fn try_from(v: &str) -> Result<Self, Self::Error> {
            Self::from_str(v)
        }
    }
    impl TryFrom<String> for Syntax {
        type Error = Error;
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
                OptimizeMode::LiteRuntime => ::core::fmt::Formatter::write_str(f, "LiteRuntime"),
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
    #[doc(hidden)]
    pub(crate) struct Inner {
        name: String,
        path: PathBuf,
        pkg_name: String,
        pkg: Option<package::Key>,
        fqn: FullyQualifiedName,
        messages: Nodes<message::Key>,
        is_build_target: bool,
        used_imports: HashSet<Key>,
        syntax: Syntax,
        ///  Sets the Java package where classes generated from this .proto will
        /// be  placed.  By default, the proto package is used, but this
        /// is often  inappropriate because proto packages do not
        /// normally start with backwards  domain names.
        java_package: Option<String>,
        ///  Controls the name of the wrapper Java class generated for the
        /// .proto file.  That class will always contain the .proto
        /// file's getDescriptor() method as  well as any top-level
        /// extensions defined in the .proto file.  If
        /// java_multiple_files is disabled, then all the other classes
        /// from the  .proto file will be nested inside the
        /// single wrapper outer class.
        java_outer_classname: Option<String>,
        ///  If enabled, then the Java code generator will generate a separate
        /// .java  file for each top-level message, enum, and service
        /// defined in the .proto  file.  Thus, these types will *not*
        /// be nested inside the wrapper class  named by
        /// java_outer_classname.  However, the wrapper class will still
        /// be  generated to contain the file's getDescriptor()
        /// method as well as any  top-level extensions defined in the file.
        java_multiple_files: bool,
        ///  This option does nothing.
        java_generate_equals_and_hash: bool,
        ///  If set true, then the Java2 code generator will generate code that
        ///  throws an exception whenever an attempt is made to assign a
        /// non-UTF-8  byte sequence to a string field.
        ///  Message reflection will do the same.
        ///  However, an extension field still accepts non-UTF-8 byte sequences.
        ///  This option has no effect on when used with the lite runtime.
        java_string_check_utf8: bool,
        optimize_for: Option<OptimizeMode>,
        ///  Sets the Go package where structs generated from this .proto will
        /// be  placed. If omitted, the Go package will be derived from
        /// the following:
        ///    - The basename of the package import path, if provided.
        ///    - Otherwise, the package statement in the .proto file, if
        ///      present.
        ///    - Otherwise, the basename of the .proto file, without extension.
        go_package: Option<String>,
        ///  Should generic services be generated in each language?  "Generic"
        /// services  are not specific to any particular RPC system.  They are
        /// generated by the  main code generators in each language (without
        /// additional plugins).  Generic services were the only kind of service
        /// generation supported by  early versions of google.protobuf.
        ///
        ///  Generic services are now considered deprecated in favor of using
        /// plugins  that generate code specific to your particular RPC system.
        /// Therefore,  these default to false.  Old code which depends on
        /// generic services should  explicitly set them to true.
        cc_generic_services: bool,
        java_generic_services: bool,
        py_generic_services: bool,
        php_generic_services: bool,
        ///  Is this file deprecated?
        ///  Depending on the target platform, this can emit Deprecated
        /// annotations  for everything in the file, or it will be
        /// completely ignored; in the very  least, this is a
        /// formalization for deprecating files.
        deprecated: bool,
        ///  Enables the use of arenas for the proto messages in this file. This
        /// applies  only to generated classes for C++.
        cc_enable_arenas: bool,
        ///  Sets the objective c class prefix which is prepended to all
        /// objective c  generated classes from this .proto. There is no
        /// default.
        objc_class_prefix: Option<String>,
        ///  Namespace for generated classes; defaults to the package.
        csharp_namespace: Option<String>,
        ///  By default Swift generators will take the proto package and
        /// CamelCase it  replacing '.' with underscore and use that to
        /// prefix the types/symbols  defined. When this options is
        /// provided, they will use this value instead  to prefix the
        /// types/symbols defined.
        swift_prefix: Option<String>,
        ///  Sets the php class prefix which is prepended to all php generated
        /// classes  from this .proto. Default is empty.
        php_class_prefix: Option<String>,
        ///  Use this option to change the namespace of php generated classes.
        /// Default  is empty. When this option is empty, the package name will
        /// be used for  determining the namespace.
        php_namespace: Option<String>,
        ///  Use this option to change the namespace of php generated metadata
        /// classes.  Default is empty. When this option is empty, the proto
        /// file name will be  used for determining the namespace.
        php_metadata_namespace: Option<String>,
        ///  Use this option to change the package of ruby generated classes.
        /// Default  is empty. When this option is not set, the package name
        /// will be used for  determining the ruby package.
        ruby_package: Option<String>,
        ///  The parser stores options it doesn't recognize here.
        ///  See the documentation for the "Options" section above.
        uninterpreted_option: Vec<UninterpretedOption>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "name",
                "path",
                "pkg_name",
                "pkg",
                "fqn",
                "messages",
                "is_build_target",
                "used_imports",
                "syntax",
                "java_package",
                "java_outer_classname",
                "java_multiple_files",
                "java_generate_equals_and_hash",
                "java_string_check_utf8",
                "optimize_for",
                "go_package",
                "cc_generic_services",
                "java_generic_services",
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
                "uninterpreted_option",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.name,
                &self.path,
                &self.pkg_name,
                &self.pkg,
                &self.fqn,
                &self.messages,
                &self.is_build_target,
                &self.used_imports,
                &self.syntax,
                &self.java_package,
                &self.java_outer_classname,
                &self.java_multiple_files,
                &self.java_generate_equals_and_hash,
                &self.java_string_check_utf8,
                &self.optimize_for,
                &self.go_package,
                &self.cc_generic_services,
                &self.java_generic_services,
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
                &&self.uninterpreted_option,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                name: ::core::clone::Clone::clone(&self.name),
                path: ::core::clone::Clone::clone(&self.path),
                pkg_name: ::core::clone::Clone::clone(&self.pkg_name),
                pkg: ::core::clone::Clone::clone(&self.pkg),
                fqn: ::core::clone::Clone::clone(&self.fqn),
                messages: ::core::clone::Clone::clone(&self.messages),
                is_build_target: ::core::clone::Clone::clone(&self.is_build_target),
                used_imports: ::core::clone::Clone::clone(&self.used_imports),
                syntax: ::core::clone::Clone::clone(&self.syntax),
                java_package: ::core::clone::Clone::clone(&self.java_package),
                java_outer_classname: ::core::clone::Clone::clone(&self.java_outer_classname),
                java_multiple_files: ::core::clone::Clone::clone(&self.java_multiple_files),
                java_generate_equals_and_hash: ::core::clone::Clone::clone(
                    &self.java_generate_equals_and_hash,
                ),
                java_string_check_utf8: ::core::clone::Clone::clone(&self.java_string_check_utf8),
                optimize_for: ::core::clone::Clone::clone(&self.optimize_for),
                go_package: ::core::clone::Clone::clone(&self.go_package),
                cc_generic_services: ::core::clone::Clone::clone(&self.cc_generic_services),
                java_generic_services: ::core::clone::Clone::clone(&self.java_generic_services),
                py_generic_services: ::core::clone::Clone::clone(&self.py_generic_services),
                php_generic_services: ::core::clone::Clone::clone(&self.php_generic_services),
                deprecated: ::core::clone::Clone::clone(&self.deprecated),
                cc_enable_arenas: ::core::clone::Clone::clone(&self.cc_enable_arenas),
                objc_class_prefix: ::core::clone::Clone::clone(&self.objc_class_prefix),
                csharp_namespace: ::core::clone::Clone::clone(&self.csharp_namespace),
                swift_prefix: ::core::clone::Clone::clone(&self.swift_prefix),
                php_class_prefix: ::core::clone::Clone::clone(&self.php_class_prefix),
                php_namespace: ::core::clone::Clone::clone(&self.php_namespace),
                php_metadata_namespace: ::core::clone::Clone::clone(&self.php_metadata_namespace),
                ruby_package: ::core::clone::Clone::clone(&self.ruby_package),
                uninterpreted_option: ::core::clone::Clone::clone(&self.uninterpreted_option),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.name == other.name
                && self.path == other.path
                && self.pkg_name == other.pkg_name
                && self.pkg == other.pkg
                && self.fqn == other.fqn
                && self.messages == other.messages
                && self.is_build_target == other.is_build_target
                && self.used_imports == other.used_imports
                && self.syntax == other.syntax
                && self.java_package == other.java_package
                && self.java_outer_classname == other.java_outer_classname
                && self.java_multiple_files == other.java_multiple_files
                && self.java_generate_equals_and_hash == other.java_generate_equals_and_hash
                && self.java_string_check_utf8 == other.java_string_check_utf8
                && self.optimize_for == other.optimize_for
                && self.go_package == other.go_package
                && self.cc_generic_services == other.cc_generic_services
                && self.java_generic_services == other.java_generic_services
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
                && self.uninterpreted_option == other.uninterpreted_option
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
pub mod location {
    pub struct Comments {
        /// Any comment immediately preceding the node, without any
        /// whitespace between it and the comment.
        pub(crate) leading: Option<String>,
        pub(crate) trailing: Option<String>,
        pub(crate) leading_detached: Vec<String>,
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
    impl Comments {
        /// Any comment immediately preceding the node, without any
        /// whitespace between it and the comment.
        pub fn leading(&self) -> Option<&str> {
            self.leading.as_deref()
        }
        /// Any comment immediately following the entity, without any
        /// whitespace between it and the comment. If the comment would be a
        /// leading comment for another entity, it won't be considered a
        /// trailing comment.
        pub fn trailing(&self) -> Option<&str> {
            self.trailing.as_deref()
        }
        /// Each comment block or line above the entity but seperated by
        /// whitespace.
        pub fn leading_detached(&self) -> &[String] {
            &self.leading_detached
        }
    }
    pub(crate) struct Location {
        path: Vec<i32>,
        ///  Always has exactly three or four elements: start line, start
        /// column,  end line (optional, otherwise assumed same as start
        /// line), end column.  These are packed into a single field for
        /// efficiency.  Note that line  and column numbers are
        /// zero-based -- typically you will want to add  1 to each
        /// before displaying to a user.
        span: Vec<i32>,
    }
    impl Location {
        pub fn path(&self) -> &[i32] {
            &self.path
        }
        ///  Always has exactly three or four elements: start line, start
        /// column,  end line (optional, otherwise assumed same as start
        /// line), end column.  These are packed into a single field for
        /// efficiency.  Note that line  and column numbers are
        /// zero-based -- typically you will want to add  1 to each
        /// before displaying to a user.
        pub fn span(&self) -> &[i32] {
            &self.span
        }
    }
    #[repr(i32)]
    pub(crate) enum FileDescriptorPath {
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
        MessageType = 4,
        /// FileDescriptorProto.enum_type
        EnumType = 5,
        /// FileDescriptorProto.service
        Service = 6,
        /// FileDescriptorProto.extension
        Extension = 7,
        Options = 8,
        /// This field contains optional information about the original source
        /// code. You may safely remove this entire field without
        /// harming runtime functionality of the descriptors -- the
        /// information is needed only by development tools.
        SourceCodeInfo = 9,
        /// FileDescriptorProto.syntax
        Syntax = 12,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FileDescriptorPath {
        #[inline]
        fn clone(&self) -> FileDescriptorPath {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FileDescriptorPath {}
    #[automatically_derived]
    impl ::core::fmt::Debug for FileDescriptorPath {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    FileDescriptorPath::Name => "Name",
                    FileDescriptorPath::Package => "Package",
                    FileDescriptorPath::Dependency => "Dependency",
                    FileDescriptorPath::PublicDependency => "PublicDependency",
                    FileDescriptorPath::WeakDependency => "WeakDependency",
                    FileDescriptorPath::MessageType => "MessageType",
                    FileDescriptorPath::EnumType => "EnumType",
                    FileDescriptorPath::Service => "Service",
                    FileDescriptorPath::Extension => "Extension",
                    FileDescriptorPath::Options => "Options",
                    FileDescriptorPath::SourceCodeInfo => "SourceCodeInfo",
                    FileDescriptorPath::Syntax => "Syntax",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FileDescriptorPath {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FileDescriptorPath {
        #[inline]
        fn eq(&self, other: &FileDescriptorPath) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FileDescriptorPath {}
    #[automatically_derived]
    impl ::core::cmp::Eq for FileDescriptorPath {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl FileDescriptorPath {
        pub const fn as_i32(self) -> i32 {
            self as i32
        }
        pub(crate) const NAME: i32 = Self::Name.as_i32();
        pub(crate) const PACKAGE: i32 = Self::Package.as_i32();
        pub(crate) const DEPENDENCY: i32 = Self::Dependency.as_i32();
        pub(crate) const PUBLIC_DEPENDENCY: i32 = Self::PublicDependency.as_i32();
        pub(crate) const WEAK_DEPENDENCY: i32 = Self::WeakDependency.as_i32();
        pub(crate) const MESSAGE_TYPE: i32 = Self::MessageType.as_i32();
        pub(crate) const ENUM_TYPE: i32 = Self::EnumType.as_i32();
        pub(crate) const SERVICE: i32 = Self::Service.as_i32();
        pub(crate) const EXTENSION: i32 = Self::Extension.as_i32();
        pub(crate) const OPTIONS: i32 = Self::Options.as_i32();
        pub(crate) const SOURCE_CODE_INFO: i32 = Self::SourceCodeInfo.as_i32();
        pub(crate) const SYNTAX: i32 = Self::Syntax.as_i32();
    }
    impl TryFrom<i32> for FileDescriptorPath {
        type Error = i32;
        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                Self::NAME => Ok(Self::Name),
                Self::PACKAGE => Ok(Self::Package),
                Self::DEPENDENCY => Ok(Self::Dependency),
                Self::PUBLIC_DEPENDENCY => Ok(Self::PublicDependency),
                Self::WEAK_DEPENDENCY => Ok(Self::WeakDependency),
                Self::MESSAGE_TYPE => Ok(Self::MessageType),
                Self::ENUM_TYPE => Ok(Self::EnumType),
                Self::SERVICE => Ok(Self::Service),
                Self::EXTENSION => Ok(Self::Extension),
                Self::OPTIONS => Ok(Self::Options),
                Self::SOURCE_CODE_INFO => Ok(Self::SourceCodeInfo),
                Self::SYNTAX => Ok(Self::Syntax),
                _ => Err(v),
            }
        }
    }
    impl PartialEq<i32> for FileDescriptorPath {
        fn eq(&self, other: &i32) -> bool {
            *other == *self as i32
        }
    }
    impl PartialEq<FileDescriptorPath> for i32 {
        fn eq(&self, other: &FileDescriptorPath) -> bool {
            *other == *self
        }
    }
    /// Paths for nodes in a [`DescriptorProto`]
    pub(crate) enum DescriptorPath {
        /// DescriptorProto.field
        Field = 2,
        /// DescriptorProto.nested_type
        NestedType = 3,
        /// DescriptorProto.enum_type
        EnumType = 4,
        Extension = 6,
        /// DescriptorProto.oneof_decl
        OneofDecl = 8,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DescriptorPath {
        #[inline]
        fn clone(&self) -> DescriptorPath {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DescriptorPath {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DescriptorPath {
        #[inline]
        fn eq(&self, other: &DescriptorPath) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DescriptorPath {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DescriptorPath {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DescriptorPath {}
    impl DescriptorPath {
        pub const fn as_i32(self) -> i32 {
            self as i32
        }
        pub(crate) const FIELD: i32 = Self::Field.as_i32();
        pub(crate) const NESTED_TYPE: i32 = Self::NestedType.as_i32();
        pub(crate) const ENUM_TYPE: i32 = Self::EnumType.as_i32();
        pub(crate) const EXTENSION: i32 = Self::Extension.as_i32();
        pub(crate) const ONEOF_DECL: i32 = Self::OneofDecl.as_i32();
    }
    impl TryFrom<i32> for DescriptorPath {
        type Error = i32;
        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                Self::FIELD => Ok(Self::Field),
                Self::NESTED_TYPE => Ok(Self::NestedType),
                Self::ENUM_TYPE => Ok(Self::EnumType),
                Self::EXTENSION => Ok(Self::Extension),
                Self::ONEOF_DECL => Ok(Self::OneofDecl),
                _ => Err(v),
            }
        }
    }
}
pub mod message {
    use crate::{
        ast::{impl_traits, Access, Accessor, Ast, FullyQualifiedName, Nodes},
        field::{self, Field},
        file, message,
        oneof::{self, Oneof},
    };
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
        fields: Nodes<field::Key>,
        messages: Nodes<message::Key>,
        oneofs: Nodes<oneof::Key>,
        real_oneofs: Nodes<oneof::Key>,
        synthetic_oneofs: Nodes<oneof::Key>,
        dependents: Nodes<file::Key>,
        applied_extensions: Nodes<oneof::Key>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "fqn",
                "fields",
                "messages",
                "oneofs",
                "real_oneofs",
                "synthetic_oneofs",
                "dependents",
                "applied_extensions",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.fqn,
                &self.fields,
                &self.messages,
                &self.oneofs,
                &self.real_oneofs,
                &self.synthetic_oneofs,
                &self.dependents,
                &&self.applied_extensions,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Inner", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
                && self.fields == other.fields
                && self.messages == other.messages
                && self.oneofs == other.oneofs
                && self.real_oneofs == other.real_oneofs
                && self.synthetic_oneofs == other.synthetic_oneofs
                && self.dependents == other.dependents
                && self.applied_extensions == other.applied_extensions
        }
    }
    pub struct Message<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Message<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Message<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Message<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Message<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Message<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Message<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Message<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
    pub enum WellKnownMessage {
        /// Any contains an arbitrary serialized message along with a URL that
        /// describes the type of the serialized message.
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
        /// represented as a count of seconds and fractions of seconds
        /// at nanosecond resolution. It is independent of any calendar
        /// and concepts like "day" or "month". It is related to
        /// Timestamp in that the difference between two Timestamp values
        /// is a Duration and it can be added or subtracted from a Timestamp.
        /// Range is approximately +-10,000 years.
        ///
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#duration>
        Duration,
        /// A generic empty message that you can re-use to avoid defining
        /// duplicated empty messages in your APIs. A typical example is
        /// to use it as the request or the response type of an API
        /// method. For Instance:
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
        /// Declares an API to be included in this API. The including API must
        /// redeclare all the methods from the included API, but documentation
        /// and options are inherited as follows:
        ///
        /// If after comment and whitespace stripping, the documentation string
        /// of the redeclared method is empty, it will be inherited from
        /// the original method.
        ///
        /// Each annotation belonging to the service config (http, visibility)
        /// which is not set in the redeclared method will be inherited.
        ///
        /// If an http annotation is inherited, the path pattern will be
        /// modified as follows. Any version prefix will be replaced by
        /// the version of the including API plus the root path if
        /// specified.
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
        /// A protocol buffer option, which can be attached to a message, field,
        /// enumeration, etc.
        ///
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#option>
        Option,
        /// SourceContext represents information about the source of a protobuf
        /// element, like the file in which it is defined.
        ///
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#sourcecontext>
        SourceContext,
        /// Wrapper message for string.
        ///
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#stringvalue>
        StringValue,
        /// Struct represents a structured data value, consisting of fields
        /// which map to dynamically typed values. In some languages,
        /// Struct might be supported by a native representation. For
        /// example, in scripting languages like JS a struct is
        /// represented as an object. The details of that representation
        /// are described together with the proto support for
        /// the language.
        ///
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct>
        Struct,
        /// A Timestamp represents a point in time independent of any time zone
        /// or calendar, represented as seconds and fractions of seconds
        /// at nanosecond resolution in UTC Epoch time. It is encoded
        /// using the Proleptic Gregorian Calendar which extends the
        /// Gregorian calendar backwards to year one. It is encoded
        /// assuming all minutes are 60 seconds long, i.e. leap seconds
        /// are "smeared" so that no leap second table is needed for
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
        /// Value represents a dynamically typed value which can be either null,
        /// a number, a string, a boolean, a recursive struct value, or
        /// a list of values. A producer of value is expected to set one
        /// of that variants, absence of any variant indicates an error.
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
        pub(crate) const ANY: &'static str = "Any";
        pub(crate) const API: &'static str = "Api";
        pub(crate) const BOOL_VALUE: &'static str = "BoolValue";
        pub(crate) const BYTES_VALUE: &'static str = "BytesValue";
        pub(crate) const DOUBLE_VALUE: &'static str = "DoubleValue";
        pub(crate) const DURATION: &'static str = "Duration";
        pub(crate) const EMPTY: &'static str = "Empty";
        pub(crate) const ENUM: &'static str = "Enum";
        pub(crate) const ENUM_VALUE: &'static str = "EnumValue";
        pub(crate) const FIELD: &'static str = "Field";
        pub(crate) const FIELD_KIND: &'static str = "FieldKind";
        pub(crate) const FIELD_MASK: &'static str = "FieldMask";
        pub(crate) const FLOAT_VALUE: &'static str = "FloatValue";
        pub(crate) const INT32_VALUE: &'static str = "Int32Value";
        pub(crate) const INT64_VALUE: &'static str = "Int64Value";
        pub(crate) const LIST_VALUE: &'static str = "ListValue";
        pub(crate) const METHOD: &'static str = "Method";
        pub(crate) const MIXIN: &'static str = "Mixin";
        pub(crate) const OPTION: &'static str = "Option";
        pub(crate) const SOURCE_CONTEXT: &'static str = "SourceContext";
        pub(crate) const STRING_VALUE: &'static str = "StringValue";
        pub(crate) const STRUCT: &'static str = "Struct";
        pub(crate) const TIMESTAMP: &'static str = "Timestamp";
        pub(crate) const TYPE: &'static str = "Type";
        pub(crate) const UINT32_VALUE: &'static str = "UInt32Value";
        pub(crate) const UINT64_VALUE: &'static str = "UInt64Value";
        pub(crate) const VALUE: &'static str = "Value";
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
        fn from_str(s: &str) -> ::std::result::Result<WellKnownMessage, Self::Err> {
            match s {
                Self::ANY => Ok(WellKnownMessage::Any),
                Self::API => Ok(WellKnownMessage::Api),
                Self::BOOL_VALUE => Ok(WellKnownMessage::BoolValue),
                Self::BYTES_VALUE => Ok(WellKnownMessage::BytesValue),
                Self::DOUBLE_VALUE => Ok(WellKnownMessage::DoubleValue),
                Self::DURATION => Ok(WellKnownMessage::Duration),
                Self::EMPTY => Ok(WellKnownMessage::Empty),
                Self::ENUM => Ok(WellKnownMessage::Enum),
                Self::ENUM_VALUE => Ok(WellKnownMessage::EnumValue),
                Self::FIELD => Ok(WellKnownMessage::Field),
                Self::FIELD_KIND => Ok(WellKnownMessage::FieldKind),
                Self::FIELD_MASK => Ok(WellKnownMessage::FieldMask),
                Self::FLOAT_VALUE => Ok(WellKnownMessage::FloatValue),
                Self::INT32_VALUE => Ok(WellKnownMessage::Int32Value),
                Self::INT64_VALUE => Ok(WellKnownMessage::Int64Value),
                Self::LIST_VALUE => Ok(WellKnownMessage::ListValue),
                Self::METHOD => Ok(WellKnownMessage::Method),
                Self::MIXIN => Ok(WellKnownMessage::Mixin),
                Self::OPTION => Ok(WellKnownMessage::Option),
                Self::SOURCE_CONTEXT => Ok(WellKnownMessage::SourceContext),
                Self::STRING_VALUE => Ok(WellKnownMessage::StringValue),
                Self::STRUCT => Ok(WellKnownMessage::Struct),
                Self::TIMESTAMP => Ok(WellKnownMessage::Timestamp),
                Self::TYPE => Ok(WellKnownMessage::Type),
                Self::UINT32_VALUE => Ok(WellKnownMessage::UInt32Value),
                Self::UINT64_VALUE => Ok(WellKnownMessage::UInt64Value),
                Self::VALUE => Ok(WellKnownMessage::Value),
                _ => Err(()),
            }
        }
    }
}
pub mod method {
    use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Inner", "fqn", &&self.fqn)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Inner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<FullyQualifiedName>;
        }
    }
    pub struct Method<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Method<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Method<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Method<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Method<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Method<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Method<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Method<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
}
pub mod oneof {
    use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};
    pub struct Oneof<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Oneof<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Oneof<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Oneof<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Oneof<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Oneof<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Oneof<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Oneof<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Inner", "fqn", &&self.fqn)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Inner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<FullyQualifiedName>;
        }
    }
}
pub mod package {
    use crate::{
        ast::{impl_traits, Accessor, Ast, Fqn, FullyQualifiedName, Nodes},
        file,
    };
    use std::fmt::Debug;
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        name: String,
        is_well_known: bool,
        files: Nodes<file::Key>,
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.name == other.name
                && self.is_well_known == other.is_well_known
                && self.files == other.files
                && self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                name: ::core::clone::Clone::clone(&self.name),
                is_well_known: ::core::clone::Clone::clone(&self.is_well_known),
                files: ::core::clone::Clone::clone(&self.files),
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Inner",
                "name",
                &self.name,
                "is_well_known",
                &self.is_well_known,
                "files",
                &self.files,
                "fqn",
                &&self.fqn,
            )
        }
    }
    pub struct Package<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Package<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Package<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Package<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Package<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Package<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Package<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Package<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
}
pub mod service {
    use crate::ast::{impl_traits, Accessor, Ast, FullyQualifiedName};
    #[repr(transparent)]
    pub(crate) struct Key(::slotmap::KeyData);
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
    pub(crate) struct Inner {
        fqn: FullyQualifiedName,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Inner {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Inner", "fqn", &&self.fqn)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Inner {
        #[inline]
        fn clone(&self) -> Inner {
            Inner {
                fqn: ::core::clone::Clone::clone(&self.fqn),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Inner {
        #[inline]
        fn eq(&self, other: &Inner) -> bool {
            self.fqn == other.fqn
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Inner {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Inner {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<FullyQualifiedName>;
        }
    }
    pub struct Service<'ast>(Accessor<'ast, Key, Inner>);
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
    impl<'ast> crate::ast::Access<Inner> for Service<'ast> {
        fn access(&self) -> &Inner {
            self.0.access()
        }
    }
    impl<'ast> Service<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        pub fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fully_qualified_name(self)
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        pub fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            <Self as crate::ast::Fqn>::fqn(self)
        }
    }
    impl<'ast> crate::ast::Fqn for Service<'ast> {
        ///Returns the [`FullyQualifiedName`] of the Message.
        fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
            &self.0.fqn
        }
        ///Alias for `fully_qualified_name` - returns the
        /// [`FullyQualifiedName`] of the Package.
        fn fqn(&self) -> &crate::ast::FullyQualifiedName {
            self.fully_qualified_name()
        }
    }
    impl<'ast> From<(Key, &'ast Ast)> for Service<'ast> {
        fn from((key, ast): (Key, &'ast Ast)) -> Self {
            Self(crate::ast::Accessor::new(key, ast))
        }
    }
    impl<'ast> From<crate::ast::Accessor<'ast, Key, Inner>> for Service<'ast> {
        fn from(accessor: crate::ast::Accessor<'ast, Key, Inner>) -> Self {
            Self(accessor)
        }
    }
    impl<'ast> ::std::fmt::Display for Service<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Display::fmt(&self.access().fqn, f)
        }
    }
    impl<'ast> ::std::fmt::Debug for Service<'ast> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            use crate::ast::Access;
            ::std::fmt::Debug::fmt(self.access(), f)
        }
    }
}
pub(crate) type HashMap<K, V> = ahash::HashMap<K, V>;
pub(crate) type HashSet<V> = ahash::HashSet<V>;
pub(crate) type IndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<AHasher>>;
