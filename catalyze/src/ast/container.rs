use super::{
    file::{self, File},
    message::{self, Message},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Key {
    Message(message::Key),
    File(file::Key),
}

impl Default for Key {
    fn default() -> Self {
        Self::File(file::Key::default())
    }
}

impl From<message::Key> for Key {
    fn from(key: message::Key) -> Self {
        Self::Message(key)
    }
}
impl From<file::Key> for Key {
    fn from(key: file::Key) -> Self {
        Self::File(key)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Container<'ast> {
    Message(Message<'ast>),
    File(File<'ast>),
}

impl<'ast> From<File<'ast>> for Container<'ast> {
    fn from(v: File<'ast>) -> Self {
        Self::File(v)
    }
}

impl<'ast> From<Message<'ast>> for Container<'ast> {
    fn from(v: Message<'ast>) -> Self {
        Self::Message(v)
    }
}

impl<'ast> Container<'ast> {
    /// Returns `true` if the container is [`Message`].
    ///
    /// [`Message`]: Container::Message
    #[must_use]
    pub const fn is_message(self) -> bool {
        matches!(self, Self::Message(..))
    }

    #[must_use]
    pub fn as_message(self) -> Option<Message<'ast>> {
        if let Self::Message(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_message(self) -> Result<Message<'ast>, Self> {
        if let Self::Message(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the container is [`File`].
    ///
    /// [`File`]: Container::File
    #[must_use]
    pub fn is_file(self) -> bool {
        matches!(self, Self::File(..))
    }

    #[must_use]
    pub fn as_file(self) -> Option<File<'ast>> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_file(self) -> Result<File<'ast>, Self> {
        if let Self::File(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
