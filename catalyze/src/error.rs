

#[derive(snafu::Snafu, Debug)]
pub enum Error {
    /// Hydration errors occur due to incompatibility or malformed data. 
    /// 
    /// These errors should be incredibly rare.
    /// compiler.
    #[snafu(transparent)]
    Hydration{
        source: Pathed<HydrateError>
    }
}


pub type HydrateError = hydrate::Error;

/// Hydration errors occur due to incompatibility or malformed data. 
/// 
/// These errors should be incredibly rare.
/// compiler.
pub mod hydrate {

   #[derive(Debug,snafu::Snafu)] 
    pub enum Error {
        #[snafu(transparent)]
        UnsupportedSyntax { source: super::SyntaxError },

        /// Group is not supported, please use an embedded message instead.
        #[snafu(transparent)]
        Group{ source: super::GroupError },

        #[snafu(transparent)]
        InvalidSpan { source: super::SpanError },

        #[snafu(display("Missing source code info"))]
        MissingSourceCodeInfo,

        #[snafu(transparent)]
        FieldType {
            source: super::FieldTypeError
        },
        
        #[snafu(transparent)]
        InvalidIndex {
            source: super::IndexError,
            
        },
        /// The number of locations for a given file is invalid. 
        #[snafu(transparent)]
        Locations {
            source: super::LocationsError,
        },
    }
}


pub type LocationsError = locations::Error;
pub mod locations {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub(crate)),
        display("Invalid number of {kind} locations expected: {expected}, found: {found}",)
    )]
    pub struct Error {
        pub kind: &'static str,
        pub expected: usize,
        pub found: usize,
    }
}

pub type GroupError = FullyQualified<group::Error>;
pub mod group {
    #[derive(Debug, snafu::Snafu)] 
    #[snafu(display(
        "Group field types are deprecated and not supported. Please use an embedded message instead."
    ))]
    pub struct Error{
        pub backtrace: snafu::Backtrace,
    }
}

pub type SyntaxError = syntax::Error;
pub mod syntax {
    #[derive(Debug, PartialEq, Eq, snafu::Snafu)]
    
    #[snafu(
        display("Unsupported or invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""),
        visibility(pub(crate))
    )]
    pub struct Error {
        pub value: String,
    }
}

pub type FieldTypeError = FullyQualified<field_type::Error>;

pub mod field_type {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub(crate)),
        display("Unknown field type: {value}")
    )]
    pub struct Error {
        pub backtrace: snafu::Backtrace,
        value: i32,
    }
}

pub type SpanError = span::Error;
pub mod span {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub(crate)), 
        display(
            "Invalid span: {span:?}; path: {path:?}; expected a span length of either 3 or 4, found {}", span.len(),
        ),
    )]
    pub struct Error {
        pub span: Vec<i32>,
        pub path: Vec<i32>,
        pub backtrace: snafu::Backtrace,
    }
}

pub type IndexError = FullyQualified<index::Error>;
pub mod index {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(visibility(pub(crate)), display("Invalid index: {index}"))]
    pub struct Error {
        pub index: i32,
        pub backtrace: snafu::Backtrace,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Kind {
        Oneof,
        WeakDependency,
    }
    impl std::fmt::Display for Kind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Kind::Oneof => write!(f, "Oneof"),
                Kind::WeakDependency => write!(f, "Weak dependency"),
            }
        }
    }
}

pub type Pathed<E> = pathed::Error<E>;
pub mod pathed {
    use std::{ops::Deref, path::PathBuf};

    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub),
        display("{source} at path {:?}", path.display())
    )]
    pub struct Error<E> where E: snafu::Error + snafu::ErrorCompat {
        pub source: E,
        pub path: PathBuf,
    }

    impl<E> Deref for Error<E>
    where
        E: snafu::Error + snafu::ErrorCompat,
    {
        type Target = E;
        fn deref(&self) -> &Self::Target {
            &self.source
        }
    }
}

pub type FullyQualified<E> = fully_qualified::Error<E>;
pub mod fully_qualified {
    use crate::ast::FullyQualifiedName;
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub),
        display("{source} at \"{fully_qualified_name}\"")
    )]

    pub struct Error<E> where E: snafu::Error + snafu::ErrorCompat {
        #[snafu(backtrace)]
        pub source: E,
        pub fully_qualified_name: FullyQualifiedName,
    }
    impl<E> std::ops::Deref for Error<E>
    where
        E: snafu::Error + snafu::ErrorCompat,
    {
        type Target = E;
        fn deref(&self) -> &Self::Target {
            &self.source
        }
    }
}

