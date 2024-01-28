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

/// A trait implemented by all nodes (except `Package` itself) which returns
/// the [`Package`](super::package::Package) of the node, if any.
pub trait Package<'ast> {
    fn package(self) -> Option<super::package::Package<'ast>>;
}

/// A trait implemented by all nodes (except `File` and `Package`) which returns
/// the containing [`File`](super::file::File).
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

    /// Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of
    /// the node.
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

pub trait ReferencesMut {
    fn references_mut(&mut self) -> impl '_ + Iterator<Item = &'_ mut reference::Inner>;
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
