use std::{fmt, path::PathBuf};
use snafu::Snafu;

use crate::ast::FullyQualifiedName;

#[derive(snafu::Snafu, Debug)]
#[snafu(
    visibility(pub(crate)),
    context(suffix(Ctx)),
)]
pub enum Error {
    /// Hydration errors occur due to incompatibility or malformed data. 
    /// 
    /// These errors should be incredibly rare.
    /// compiler.
    #[snafu(display("{source} in {:?}", file_path.display()))]
    Hydration{
        source: HydrationFailed,
        file_path: PathBuf
    }
}
/// Hydration errors occur due to incompatibility or malformed data. 
/// 
/// These errors should be incredibly rare.
/// compiler.
#[derive(Debug, snafu::Snafu)] 
#[snafu(
    visibility(pub(crate)),
    context(suffix(Ctx)),
)]
pub enum HydrationFailed {
    #[snafu(transparent)]
    UnsupportedSyntax { 
        source: UnsupportedSyntax 
    },

    /// Group is not supported, please use an embedded message instead.
    #[snafu(
        display("{} for node with fully qualified name \"{fully_qualified_name}\"", source)
    )]
    GroupNotSupported{ 
        source: GroupNotSupported, 
        fully_qualified_name: FullyQualifiedName, 
    },

    #[snafu(transparent, context(false))]
    InvalidSpan { 
        source: InvalidSpan, 
    },

    #[snafu(display("Missing source code info"))]
    MissingSourceCodeInfo,

    #[snafu(display("{source} for node with fully qualified name \"{fully_qualified_name}\""))]
    UnknownFieldType { 
        source: UnknownFieldType, 
        fully_qualified_name: FullyQualifiedName,
    },
    
    /// The number of locations for a given file is invalid. 
    #[snafu(transparent)]
    LocationMisaligned { source: LocationsMisaligned },

    #[snafu(
        display(
            "Invalid Oneof index {} for field with fully qualified name \"{field_fqn}\"", 
            source.index
        )
    )]
    OneofIndex { 
        source: InvalidIndex<i32>, 
        field_fqn: FullyQualifiedName,
    },
    DependencyIndex {
        source: InvalidIndex<i32>,
        dependeency_kind: DependencyKind
    }
    
}

#[derive(Debug, snafu::Snafu)]
#[snafu(
    visibility(pub(crate)),
    display("Locations for {kind} are misaligned; expected: {expected} locations, found: {found}"),
    context(suffix(Ctx))
)]
pub struct LocationsMisaligned {
    pub kind: &'static str,
    pub expected: usize,
    pub found: usize,
    pub backtrace: snafu::Backtrace,
}


#[derive(Debug, Snafu)] 
#[snafu(
    display(
        "Group field types are deprecated and not supported. \
        Please use an embedded message instead."
    ),
    module
)]
pub struct GroupNotSupported {
    pub backtrace: snafu::Backtrace,
}

#[derive(Debug, snafu::Snafu)]
#[snafu(
    display(
        "Unsupported or invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""
    ),
    context(suffix(Ctx)),
    visibility(pub(crate))
)]
pub struct UnsupportedSyntax {
    pub backtrace: snafu::Backtrace,
    pub value: String,
}

#[derive(Debug, snafu::Snafu)]
#[snafu(
    visibility(pub(crate)),
    context(suffix(Ctx)),
    display("Unknown field type: {type_}"),
    module
)]
pub struct UnknownFieldType {
    pub backtrace: snafu::Backtrace,
    pub type_: i32,
}

#[derive(Debug, snafu::Snafu)]
#[snafu(
    visibility(pub(crate)), 
    context(suffix(Ctx)),
    module,
    display(
        "Invalid span: {span:?}; expected a span length of either 3 or 4, found {}", span.len(),
    ),
)]
pub struct InvalidSpan {
    pub span: Vec<i32>,
    pub backtrace: snafu::Backtrace,
}

#[derive(Debug, snafu::Snafu)]
#[snafu(
    visibility(pub), 
    context(suffix(Ctx)),
    display("Invalid index: {index}"),
    module
)]
pub struct InvalidIndex<I = i32> where I: TryInto<usize> + fmt::Debug + fmt::Display {
    pub index: I,
    pub backtrace: snafu::Backtrace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DependencyKind {
    Oneof,
    Weak,
    Public,
}


impl fmt::Display for DependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

