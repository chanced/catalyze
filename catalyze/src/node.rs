use std::fmt;

use crate::{file::File, message::Message, package::Package};

pub(crate) enum Key {}

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
            Kind::Package => write!(fmt, "Package"),
            Kind::File => write!(fmt, "File"),
            Kind::Message => write!(fmt, "Message"),
            Kind::Oneof => write!(fmt, "Oneof"),
            Kind::Enum => write!(fmt, "Enum"),
            Kind::EnumValue => write!(fmt, "EnumValue"),
            Kind::Service => write!(fmt, "Service"),
            Kind::Method => write!(fmt, "Method"),
            Kind::Field => write!(fmt, "Field"),
            Kind::Extension => write!(fmt, "Extension"),
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
