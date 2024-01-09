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
}
impl Error {
    pub(crate) fn unsupported_syntax(v: impl ToString) -> Self {
        Self::UnsupportedSyntax {
            value: v.to_string(),
        }
    }
}
