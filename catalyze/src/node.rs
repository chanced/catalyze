use crate::{
    enum_value::EnumValue,
    extension::Extension,
    field::Field,
    file::File,
    fqn::{Fqn, FullyQualifiedName},
    message::Message,
    method::Method,
    oneof::Oneof,
    package::Package,
    r#enum::Enum,
    service::Service,
};
use std::{collections::HashMap, fmt};

pub(crate) trait Upgrade {
    type Target;
    fn upgrade(&self) -> Self::Target;
}

pub(crate) trait Downgrade {
    type Target;
    fn downgrade(&self) -> Self::Target;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Package,
    File,
    Message,
    Oneof,
    Enum,
    EnumValue,
    Service,
    Method,
    Field,
    Extension,
}

impl fmt::Display for Kind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Package => write!(fmt, "Package"),
            Self::File => write!(fmt, "File"),
            Self::Message => write!(fmt, "Message"),
            Self::Oneof => write!(fmt, "Oneof"),
            Self::Enum => write!(fmt, "Enum"),
            Self::EnumValue => write!(fmt, "EnumValue"),
            Self::Service => write!(fmt, "Service"),
            Self::Method => write!(fmt, "Method"),
            Self::Field => write!(fmt, "Field"),
            Self::Extension => write!(fmt, "Extension"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Package(Package),
    File(File),
    Message(Message),
    Oneof(Oneof),
    Enum(Enum),
    EnumValue(EnumValue),
    Service(Service),
    Method(Method),
    Field(Field),
    Extension(Extension),
}

impl Node {
    pub const fn kind(&self) -> Kind {
        match self {
            Self::Package(_) => Kind::Package,
            Self::File(_) => Kind::File,
            Self::Message(_) => Kind::Message,
            Self::Oneof(_) => Kind::Oneof,
            Self::Enum(_) => Kind::Enum,
            Self::EnumValue(_) => Kind::EnumValue,
            Self::Service(_) => Kind::Service,
            Self::Method(_) => Kind::Method,
            Self::Field(_) => Kind::Field,
            Self::Extension(_) => Kind::Extension,
        }
    }
}

pub(crate) struct Nodes<T> {
    lookup: HashMap<FullyQualifiedName, T>,
    list: Vec<T>,
}

impl<T: Fqn + Clone> Nodes<T> {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
            list: Vec::new(),
        }
    }

    pub fn insert(&mut self, node: T) {
        self.lookup.insert(node.fqn().clone(), node.clone());
        self.list.push(node);
    }

    pub fn get(&self, fqn: &FullyQualifiedName) -> Option<&T> {
        self.lookup.get(fqn)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.list.iter()
    }
}
