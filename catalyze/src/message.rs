use std::sync::{Arc, Weak};

use inherent::inherent;

use crate::{
    extension::Extension,
    field::Field,
    file::WeakFile,
    fqn::{Fqn, FullyQualifiedName},
    node::{Downgrade, Nodes, Upgrade},
    oneof::Oneof,
};

#[derive(Debug, PartialEq)]
struct Inner {
    fqn: FullyQualifiedName,
    fields: Nodes<Field>,
    messages: Nodes<Message>,
    oneofs: Nodes<Oneof>,
    real_oneofs: Nodes<Oneof>,
    synthetic_oneofs: Nodes<Oneof>,
    dependents: Nodes<WeakFile>,
    applied_extensions: Nodes<Extension>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message(Arc<Inner>);

#[inherent]
impl Fqn for Message {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
impl Downgrade for Message {
    type Target = WeakMessage;

    fn downgrade(&self) -> Self::Target {
        WeakMessage(Arc::downgrade(&self.0))
    }
}
pub(crate) struct WeakMessage(Weak<Inner>);
impl Upgrade for WeakMessage {
    type Target = Message;
    fn upgrade(&self) -> Self::Target {
        Message(self.0.upgrade().unwrap())
    }
}
impl PartialEq<Message> for WeakMessage {
    fn eq(&self, other: &Message) -> bool {
        self.upgrade() == *other
    }
}
impl PartialEq<WeakMessage> for Message {
    fn eq(&self, other: &WeakMessage) -> bool {
        *self == other.upgrade()
    }
}
impl PartialEq for WeakMessage {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade() == other.upgrade()
    }
}
