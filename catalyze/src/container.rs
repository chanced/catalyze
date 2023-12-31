use std::collections::HashSet;

use crate::{
    file::{File, WeakFile},
    fqn::FullyQualifiedName,
    message::Message,
};

pub enum Container {
    Message(Message),
    File(File),
}
