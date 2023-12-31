use std::fmt;

/// A trait implemented by all nodes that have a [`FullyQualifiedName`].
pub trait Fqn {
    /// Returns the [`FullyQualifiedName`] of the node.
    fn fully_qualified_name(&self) -> &FullyQualifiedName;

    /// Alias for `fully_qualified_name` - returns the [`FullyQualifiedName`] of
    /// the node.
    fn fqn(&self) -> &FullyQualifiedName {
        self.fully_qualified_name()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName(String);

impl FullyQualifiedName {
    pub fn new(value: impl AsRef<str>, container: Option<&str>) -> Self {
        let value = value.as_ref();
        if value.is_empty() {
            return Self(container.unwrap_or_default().to_string());
        }
        Self(format!("{}.{}", container.unwrap_or_default(), &value))
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
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fully_qualified_name() {
        let fqn = FullyQualifiedName::new("foo", None);
        assert_eq!(fqn.as_str(), ".foo");

        let fqn = FullyQualifiedName::new("foo", Some("bar"));
        assert_eq!(fqn.as_str(), "bar.foo");

        let fqn = FullyQualifiedName::new("foo", Some(".bar"));
        assert_eq!(fqn.as_str(), ".bar.foo");
    }
}
