use std::sync::{Arc, Weak};

use inherent::inherent;

use crate::{
    fqn::{Fqn, FullyQualifiedName},
    node::{Downgrade, Upgrade},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inner {
    fqn: FullyQualifiedName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum(Arc<Inner>);

#[inherent]
impl Fqn for Enum {
    pub fn fully_qualified_name(&self) -> &FullyQualifiedName {
        &self.0.fqn
    }
}
impl Downgrade for Enum {
    type Target = WeakEnum;

    fn downgrade(&self) -> Self::Target {
        self.clone().into()
    }
}

#[derive(Debug)]
pub(crate) struct WeakEnum(Weak<Inner>);

impl From<Enum> for WeakEnum {
    fn from(value: Enum) -> Self {
        Self(Arc::downgrade(&value.0))
    }
}

impl PartialEq<Enum> for WeakEnum {
    fn eq(&self, other: &Enum) -> bool {
        self.upgrade() == *other
    }
}
impl PartialEq<WeakEnum> for Enum {
    fn eq(&self, other: &WeakEnum) -> bool {
        *self == other.upgrade()
    }
}
impl PartialEq for WeakEnum {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade() == other.upgrade()
    }
}
impl Upgrade for WeakEnum {
    type Target = Enum;

    fn upgrade(&self) -> Self::Target {
        Enum(self.0.upgrade().unwrap())
    }
}
