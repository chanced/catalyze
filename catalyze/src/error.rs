macro_rules! deref {
    ($err: ident, $E: ident) => {
        impl<$E> std::ops::Deref for $err<$E>
        where
            E: 'static + snafu::Error + snafu::ErrorCompat,
        {
            type Target = $E;
            fn deref(&self) -> &Self::Target {
                &self.source
            }
        }        
    };
}

#[derive(snafu::Snafu, Debug)]
pub enum Error {
    /// Hydration errors occur due to incompatibility or malformed data. 
    /// 
    /// These errors should be incredibly rare.
    /// compiler.
    #[snafu(transparent)]
    Hydration{
        source: FilePathed<HydrationError>
    }
}


pub type HydrationError = hydration::Error;
/// Hydration errors occur due to incompatibility or malformed data. 
/// 
/// These errors should be incredibly rare.
/// compiler.
pub mod hydration {
   #[derive(Debug,snafu::Snafu)] 
   #[snafu(visibility(pub(crate)))]
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
        FieldType { source: super::FieldTypeError },
        
        #[snafu(transparent)]
        InvalidIndex { source: super::IndexError },

        /// The number of locations for a given file is invalid. 
        #[snafu(transparent)]
        Locations { source: super::LocationsError },
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
    #[derive(Debug, snafu::Snafu)]
    
    #[snafu(
        display("Unsupported or invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""),
        visibility(pub(crate))
    )]
    pub struct Error {
        pub backtrace: snafu::Backtrace,
        pub value: String,
    }
}

pub type FieldTypeError = FullyQualified<field_type::Error>;
pub mod field_type {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub(crate)),
        display("Unknown field type: {type_}")
    )]
    pub struct Error {
        pub backtrace: snafu::Backtrace,
        pub type_: i32,
    }
}


pub type SpanError = NodePathed<span::Error>;
pub mod span {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub(crate)), 
        display(
            "Invalid span: {span:?}; expected a span length of either 3 or 4, found {}", span.len(),
        ),
    )]
    pub struct Error {
        pub span: Vec<i32>,
        pub backtrace: snafu::Backtrace,
    }
}

pub type IndexError = FullyQualified<index::Error>;
pub mod index {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(visibility(pub(crate)), display("Invalid index: {index}"))]
    pub struct Error {
        pub collection: &'static str,
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
                Self::Oneof => write!(f, "Oneof"),
                Self::WeakDependency => write!(f, "Weak dependency"),
            }
        }
    }
}



pub type NodePathed<E> = node_pathed::Error<E>;
pub mod node_pathed {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub),
        display("{source} at node path {node_path:?}")
    )]
    pub struct Error<E> where E: 'static + snafu::Error + snafu::ErrorCompat { 
        pub node_path: Vec<i32>,
        pub source: E,
    }
    deref!(Error, E);
}

pub type FilePathed<E> = file_pathed::Error<E>;
pub mod file_pathed {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub),
        display("{source} at path {:?}", file_path.display())
    )]
    pub struct Error<E> where E: 'static + snafu::Error + snafu::ErrorCompat {
        pub source: E,
        pub file_path: std::path::PathBuf,
    }
    deref!(Error, E);
}



pub type FullyQualified<E> = fully_qualified::Error<E>;
pub mod fully_qualified {
    #[derive(Debug, snafu::Snafu)]
    #[snafu(
        visibility(pub),
        display("{source} at \"{fully_qualified_name}\"")
    )]
    pub struct Error<E> where E: 'static + snafu::Error + snafu::ErrorCompat {
        #[snafu(backtrace)]
        pub source: E,
        pub fully_qualified_name: crate::ast::FullyQualifiedName,
    }
    deref!(Error, E);
}

