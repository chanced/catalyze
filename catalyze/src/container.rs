use crate::{file::File, message::Message};

pub enum Container<'ast> {
    Message(Message<'ast>),
    File(File<'ast>),
}
