use snafu::Snafu;

#[derive(Debug, PartialEq, Eq, Hash, Snafu)]
pub enum Error {
    #[snafu(display(
        "Unsupported or invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""
    ))]
    UnsupportedSyntax { value: String },

    #[snafu(display(
        "Group field types are deprecated and not supported. Use an embedded message instead."
    ))]
    GroupNotSupported,
    #[snafu(display(
        "Invalid span: {:?}; path: {:?}; expected a span length of either 3 or 4, found {}",
        span,
        path,
        span.len()
    ))]
    InvalidSpan { span: Vec<i32>, path: Vec<i32> },
}

impl Error {
    pub(crate) fn unsupported_syntax(v: impl ToString) -> Self {
        Self::UnsupportedSyntax {
            value: v.to_string(),
        }
    }
    pub(crate) fn invalid_span(loc: &protobuf::descriptor::source_code_info::Location) -> Self {
        Self::InvalidSpan {
            span: loc.span.clone(),
            path: loc.path.clone(),
        }
    }
}
