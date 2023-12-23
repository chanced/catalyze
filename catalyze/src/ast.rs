use crate::message::MessageKey;

pub(crate) enum HydrateKey {
    Message(MessageKey),
    Package,
    Enum,
    Service,
    Field,
    File,
    Comment,
    UninterpretedOption,
    Extension,
    EnumValue,
    Method,
    Oneof,
    ReservedRange,
    ReservedName,
}
