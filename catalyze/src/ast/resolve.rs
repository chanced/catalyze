use super::Ast;

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

macro_rules! impl_resolve {

    ($($col:ident -> $mod:ident,)+) => {
        $(
            impl crate::ast::resolve::Get<$mod::Key, $mod::Inner> for Ast {
                fn get(&self, key: $mod::Key) -> &$mod::Inner {
                    &self.$col[key]
                }
            }
            impl<'ast> crate::ast::resolve::Resolve<$mod::Inner> for crate::ast::resolve::Resolver<'ast, $mod::Key, $mod::Inner>
            {
                fn resolve(&self) -> &$mod::Inner {
                    crate::ast::resolve::Get::get(self.ast, self.key.clone())
                }
            }
            impl<'ast> ::std::ops::Deref for Resolver<'ast, $mod::Key, $mod::Inner>{
                type Target = $mod::Inner;
                fn deref(&self) -> &Self::Target {
                    self.resolve()
                }
            }
            impl<'ast> ::std::fmt::Debug for crate::ast::resolve::Resolver<'ast, $mod::Key, $mod::Inner>
            {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::std::fmt::Debug::fmt(self.resolve(), f)
                }
            }
        )+
    };
}

use super::{
    r#enum, enum_value, extension, extension_block, field, file, message, method, oneof, package,
    service,
};

impl_resolve!(
    packages -> package,
    files -> file,
    messages -> message,
    enums -> r#enum,
    enum_values -> enum_value,
    oneofs -> oneof,
    services -> service,
    methods -> method,
    fields -> field,
    extensions -> extension,
    extension_blocks -> extension_block,
);
