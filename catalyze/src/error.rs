use snafu::Snafu;

#[derive(Debug, PartialEq, Eq, Hash, Snafu)]
pub enum Error {
    #[snafu(display("Invalid syntax: {value:?}; expected either \"proto2\" or \"proto3\""))]
    InvalidSyntax { value: String },

    #[snafu(display(
        "Group field types are deprecated and not supported. Use an embedded message instead."
    ))]
    GroupNotSupported,
}
impl Error {
    pub(crate) fn invalid_syntax(v: String) -> Self {
        Self::InvalidSyntax { value: v }
    }
}
