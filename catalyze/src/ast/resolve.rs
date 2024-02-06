use super::{
    enum_::{EnumInner, EnumKey},
    enum_value::{EnumValueInner, EnumValueKey},
    extension::{ExtensionInner, ExtensionKey},
    extension_decl::{ExtensionDeclInner, ExtensionDeclKey},
    field::{FieldInner, FieldKey},
    file::{FileInner, FileKey},
    message::{MessageInner, MessageKey},
    method::{MethodInner, MethodKey},
    oneof::{OneofInner, OneofKey},
    package::{PackageInner, PackageKey},
    service::{ServiceInner, ServiceKey},
    Ast,
};
use std::fmt;

#[doc(hidden)]
pub(super) trait Get<K, T> {
    fn get(&self, key: K) -> &T;
    fn get_mut(&mut self, key: K) -> &mut T;
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

macro_rules! impl_resolve {

    ($($col:ident -> {$key:ident, $inner:ident},)+) => {
        $(
            impl Get<$key, $inner> for Ast {
                fn get(&self, key: $key) -> &$inner {
                    &self.$col[key]
                }
                fn get_mut(&mut self, key: $key) -> &mut $inner {
                    &mut self.$col[key]
                }
            }
            impl<'ast> Resolve<$inner> for Resolver<'ast, $key, $inner>
            {
                fn resolve(&self) -> &$inner {
                    Get::get(self.ast, self.key.clone())
                }
            }
            impl<'ast> ::std::ops::Deref for Resolver<'ast, $key, $inner>{
                type Target = $inner;
                fn deref(&self) -> &Self::Target {
                    self.resolve()
                }
            }
            impl<'ast> fmt::Debug for Resolver<'ast, $key, $inner>
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(self.resolve(), f)
                }
            }
        )+
    };
}

impl_resolve!(
    packages -> {PackageKey, PackageInner},
    files -> {FileKey, FileInner},
    messages -> {MessageKey, MessageInner},
    enums -> {EnumKey, EnumInner},
    enum_values -> {EnumValueKey, EnumValueInner},
    oneofs -> {OneofKey, OneofInner},
    services -> {ServiceKey, ServiceInner},
    methods -> {MethodKey, MethodInner},
    fields -> {FieldKey, FieldInner},
    extensions -> {ExtensionKey, ExtensionInner},
    extension_decls -> {ExtensionDeclKey, ExtensionDeclInner},
);
