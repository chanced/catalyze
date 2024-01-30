use super::Ast;
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

    ($($col:ident -> $mod:ident,)+) => {
        $(
            impl Get<$mod::Key, $mod::Inner> for Ast {
                fn get(&self, key: $mod::Key) -> &$mod::Inner {
                    &self.$col[key]
                }
                fn get_mut(&mut self, key: $mod::Key) -> &mut $mod::Inner {
                    &mut self.$col[key]
                }
            }
            impl<'ast> Resolve<$mod::Inner> for Resolver<'ast, $mod::Key, $mod::Inner>
            {
                fn resolve(&self) -> &$mod::Inner {
                    Get::get(self.ast, self.key.clone())
                }
            }
            impl<'ast> ::std::ops::Deref for Resolver<'ast, $mod::Key, $mod::Inner>{
                type Target = $mod::Inner;
                fn deref(&self) -> &Self::Target {
                    self.resolve()
                }
            }
            impl<'ast> fmt::Debug for Resolver<'ast, $mod::Key, $mod::Inner>
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(self.resolve(), f)
                }
            }
        )+
    };
}

use super::{
    enum_, enum_value, extension, extension_decl, field, file, message, method, oneof, package,
    service,
};

impl_resolve!(
    packages -> package,
    files -> file,
    messages -> message,
    enums -> enum_,
    enum_values -> enum_value,
    oneofs -> oneof,
    services -> service,
    methods -> method,
    fields -> field,
    extensions -> extension,
    extension_decls -> extension_decl,
);
