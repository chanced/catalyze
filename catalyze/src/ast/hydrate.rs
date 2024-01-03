use std::{fmt::Debug, mem, os::macos::raw::stat, path::PathBuf};

use paste::paste;
use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};
use slotmap::SlotMap;

use crate::{
    enum_value::EnumValue, error::Error, extension::Extension, field::Field, file::File,
    fqn::FullyQualifiedName, message::Message, method::Method, oneof::Oneof, package::Package,
    r#enum::Enum, service::Service, HashSet,
};

use super::Ast;

#[inline]
pub fn hydrate(
    files: &[protobuf::descriptor::FileDescriptorProto],
    targets: &HashSet<PathBuf>,
) -> Result<Ast, Error> {
    Hydrate::new(files, targets).run()
}

slotmap::new_key_type! {
    struct MessageKey;
    struct PackageKey;
    struct EnumKey;
    struct ServiceKey;
    struct FieldKey;
    struct FileKey;
    struct ExtensionKey;
    struct EnumValueKey;
    struct MethodKey;
    struct OneofKey;
}

// #[derive(Debug, Clone, Copy)]
// enum Progress {
//     Init,
//     Hydrating,
//     Finalizing,
// }
// impl Default for Progress {
//     fn default() -> Self {
//         Self::Init
//     }
// }

#[derive(Debug)]
enum State<H, T> {
    Hydrating(H),
    Final(T),
}

impl<H, T> State<H, T> {
    #[must_use]
    fn is_hydrating(&self) -> bool {
        matches!(self, Self::Hydrating(..))
    }

    #[must_use]
    fn is_final(&self) -> bool {
        matches!(self, Self::Final(..))
    }
}

macro_rules! create_state {
    ($($node: ident,)+) => {
        paste! {
            $(
                type [<$node State>]<'i> = State<[<Hydrate $node >]<'i>, $node>;
                impl<'i> From<[<Hydrate $node>]<'i>> for [<$node State>]<'i> {
                    fn from(value: [<Hydrate $node >]<'_>) -> [<$node State>] {
                        [<$node State>]::Hydrating(value)
                    }
                }
            )+
        }
    };
}
create_state!(Package, File, Message, Enum, Service, Field, Extension, EnumValue, Method, Oneof,);

#[derive(Debug, Clone, Copy)]
enum Key {
    Package(PackageKey),
    File(FileKey),
    Message(MessageKey),
    Enum(EnumKey),
    Service(ServiceKey),
    Field(FieldKey),
    Extension(ExtensionKey),
    EnumValue(EnumValueKey),
    Method(MethodKey),
    Oneof(OneofKey),
}

impl From<OneofKey> for Key {
    fn from(v: OneofKey) -> Self {
        Self::Oneof(v)
    }
}

impl From<MethodKey> for Key {
    fn from(v: MethodKey) -> Self {
        Self::Method(v)
    }
}

impl From<EnumValueKey> for Key {
    fn from(v: EnumValueKey) -> Self {
        Self::EnumValue(v)
    }
}

impl From<ExtensionKey> for Key {
    fn from(v: ExtensionKey) -> Self {
        Self::Extension(v)
    }
}

impl From<FieldKey> for Key {
    fn from(v: FieldKey) -> Self {
        Self::Field(v)
    }
}

impl From<ServiceKey> for Key {
    fn from(v: ServiceKey) -> Self {
        Self::Service(v)
    }
}

impl From<EnumKey> for Key {
    fn from(v: EnumKey) -> Self {
        Self::Enum(v)
    }
}

impl From<MessageKey> for Key {
    fn from(v: MessageKey) -> Self {
        Self::Message(v)
    }
}

impl From<FileKey> for Key {
    fn from(v: FileKey) -> Self {
        Self::File(v)
    }
}

impl From<PackageKey> for Key {
    fn from(v: PackageKey) -> Self {
        Self::Package(v)
    }
}

#[derive(Debug, Clone, Copy)]
enum ContainerKey {
    Message(MessageKey),
    File(FileKey),
}
impl Default for ContainerKey {
    fn default() -> Self {
        Self::File(FileKey::default())
    }
}

impl From<FileKey> for ContainerKey {
    fn from(v: FileKey) -> Self {
        Self::File(v)
    }
}
impl From<MessageKey> for ContainerKey {
    fn from(key: MessageKey) -> Self {
        Self::Message(key)
    }
}

#[derive(Debug, Clone)]
enum HydrateContainer<'h, 'i> {
    Message(&'h HydrateMessage<'i>),
    File(&'h HydrateFile<'i>),
}

impl HydrateContainer<'_, '_> {
    fn fqn(&self) -> &FullyQualifiedName {
        match self {
            Self::Message(m) => &m.fqn,
            Self::File(f) => &f.fqn,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct HydrateMessage<'i> {
    // progress: Progress,
    descriptor: &'i DescriptorProto,
    fqn: FullyQualifiedName,
    container: ContainerKey,
    fields: Vec<FieldKey>,
    oneofs: Vec<OneofKey>,
    extensions: Vec<ExtensionKey>,
    embeds: Vec<MessageKey>,
}
impl<'i> HydrateMessage<'i> {
    fn new(_desc: &DescriptorProto, container: HydrateContainer<'_, 'i>) -> Self {
        let _fqn = container.fqn().clone();
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
struct HydratePackage<'i> {
    // progress: Progress,
    name: String,
    fqn: FullyQualifiedName,
    messages: Vec<MessageKey>,
    enums: Vec<EnumKey>,
    services: Vec<ServiceKey>,
    files: Vec<FileKey>,
    // useless lifetime for uniformity
    phantom: std::marker::PhantomData<&'i ()>,
}

impl<'i> HydratePackage<'i> {}

#[derive(Debug, Default, Clone)]
struct HydrateEnum<'i> {
    // progress: Progress,
    fqn: Option<FullyQualifiedName>,
    descriptor: Option<&'i EnumDescriptorProto>,
    values: Vec<EnumValueKey>,
    pkg: Option<PackageKey>,
    container: Option<ContainerKey>,
    file: FileKey,
    dependents: Vec<MessageKey>,
}

impl<'i> HydrateEnum<'i> {
    fn new(descriptor: &'i EnumDescriptorProto, container: HydrateContainer<'_, 'i>) -> Self {
        let _fqn = container.fqn().clone();
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateService<'i> {
    // progress: Progress,
    fqn: Option<FullyQualifiedName>,
    descriptor: Option<&'i ServiceDescriptorProto>,
    file: FileKey,
}

#[derive(Debug, Default)]
struct HydrateField<'i> {
    // progress: Progress,
    descriptor: Option<&'i FieldDescriptorProto>,
    msg: MessageKey,
}

#[derive(Debug, Default)]
struct HydrateEnumValue<'i> {
    // progress: Progress,
    descriptor: Option<&'i EnumDescriptorProto>,
    enum_: EnumKey,
}

#[derive(Debug, Default)]
struct HydrateMethod<'i> {
    // progress: Progress,
    descriptor: Option<&'i MethodDescriptorProto>,
}

#[derive(Debug, Default)]
struct HydrateOneof<'i> {
    // progress: Progress,
    descriptor: Option<&'i OneofDescriptorProto>,
}

#[derive(Debug, Default)]
struct HydrateExtension<'i> {
    // progress: Progress,
    descriptor: Option<&'i FieldDescriptorProto>,
}

#[derive(Debug, Default)]
struct HydrateFile<'i> {
    // progress: Progress,
    fqn: FullyQualifiedName,
    descriptor: &'i FileDescriptorProto,
    msgs: Vec<MessageKey>,
    enums: Vec<EnumKey>,
    services: Vec<ServiceKey>,
    pkg: Option<PackageKey>,
}

impl<'i> HydrateFile<'i> {
    fn new(descriptor: &'i FileDescriptorProto) -> HydrateFile<'i> {
        Self {
            descriptor,
            ..Default::default()
        }
    }
}

pub struct Hydrate<'i> {
    packages: SlotMap<PackageKey, PackageState<'i>>,
    files: SlotMap<FileKey, FileState<'i>>,
    messages: SlotMap<MessageKey, MessageState<'i>>,
    enums: SlotMap<EnumKey, EnumState<'i>>,
    services: SlotMap<ServiceKey, ServiceState<'i>>,
    fields: SlotMap<FieldKey, FieldState<'i>>,
    extensions: SlotMap<ExtensionKey, ExtensionState<'i>>,
    enum_values: SlotMap<EnumValueKey, EnumValueState<'i>>,
    methods: SlotMap<MethodKey, MethodState<'i>>,
    oneofs: SlotMap<OneofKey, OneofState<'i>>,
    targets: &'i HashSet<PathBuf>,
    input: Vec<FileKey>,
}

impl<'i> Hydrate<'i> {
    fn new(
        files: &'i [protobuf::descriptor::FileDescriptorProto],
        targets: &'i HashSet<PathBuf>,
    ) -> Self {
        let mut file_map = SlotMap::with_key();
        let mut input = Vec::with_capacity(files.len());
        for descriptor in files {
            let state = HydrateFile::new(descriptor).into();
            let key = file_map.insert(state);
            input.push(key);
        }
        Self {
            packages: SlotMap::with_key(),
            files: file_map,
            messages: SlotMap::with_key(),
            enums: SlotMap::with_key(),
            services: SlotMap::with_key(),
            fields: SlotMap::with_key(),
            extensions: SlotMap::with_key(),
            enum_values: SlotMap::with_key(),
            methods: SlotMap::with_key(),
            oneofs: SlotMap::with_key(),
            input,
            targets,
        }
    }

    fn run(mut self) -> Result<Ast, Error> {
        let mut stack: Vec<Key> = self.input.clone().into_iter().map(Into::into).collect();
        while let Some(next) = stack.pop() {
            self.hydrate(next, &mut stack)?;
        }
        todo!()
    }

    fn init_file(&mut self, descriptor: &'i FileDescriptorProto) -> Result<Key, Error> {}

    fn hydrate(&mut self, key: Key, stack: &mut Vec<Key>) -> Result<(), Error> {
        match key {
            Key::Package(key) => self.hydrate_package(key, stack),
            Key::File(key) => self.hydrate_file(key, stack),
            Key::Message(key) => self.hydrate_message(key, stack),
            Key::Enum(key) => self.hydrate_enum(key, stack),
            Key::Service(key) => self.hydrate_service(key, stack),
            Key::Field(key) => self.hydrate_field(key, stack),
            Key::Extension(key) => self.hydrate_extension(key, stack),
            Key::EnumValue(key) => self.hydrate_enum_value(key, stack),
            Key::Method(key) => self.hydrate_method(key, stack),
            Key::Oneof(key) => self.hydrate_oneof(key, stack),
        }
    }
    fn hydrate_package(&mut self, key: PackageKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn hydrate_file(&mut self, key: FileKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_message(&mut self, key: MessageKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_enum(&mut self, key: EnumKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_service(&mut self, key: ServiceKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_field(&mut self, key: FieldKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_extension(&mut self, key: ExtensionKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_enum_value(&mut self, key: EnumValueKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_method(&mut self, key: MethodKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
    fn hydrate_oneof(&mut self, key: OneofKey, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
}
