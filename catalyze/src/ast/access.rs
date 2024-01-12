pub trait References<'ast> {
    fn references(self) -> super::reference::References<'ast>;
}

/// A trait implemented by nodes with parent nodes, providing access to
/// the [`Container`](super::Container) node.
pub trait Container<'ast> {
    fn container(self) -> super::Container<'ast>;
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
    fn uninterpreted_options(&self) -> &[super::UninterpretedOption];
}

/// A trait implemented by nodes with reserved names and ranges.
pub trait Reserved {
    fn reserved_names(&self) -> &[String];
    fn reserved_ranges(&self) -> &[super::ReservedRange];
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
