use std::{fmt::Debug, path::PathBuf};

use itertools::Itertools;
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
    HydrateAst::new(files, targets).run()
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

trait Hydrate {
    fn hydrate<'i>(&self, hydrate: &mut HydrateAst<'i>) -> Result<(), Error>;
}

trait Finalize {
    type Target;
    fn finalize(self, hydrate: &mut HydrateAst) -> Self::Target;
}

#[derive(Debug)]
enum State<H, T> {
    Init(H),
    Hydrating(H),
    Finalizing(H),
    Final(T),
}

impl<H, T> State<H, T>
where
    H: Debug + Finalize<Target = T>,
    T: Debug,
{
    fn as_init(&mut self) -> &mut H {
        match self {
            Self::Init(h) => h,
            _ => panic!("state is not Init: {self:?}"),
        }
    }
    fn as_hydrating(&mut self) -> &mut H {
        match self {
            Self::Hydrating(h) => h,
            _ => panic!("state is not Hydrating: {self:?}"),
        }
    }
    fn as_finalizing(&mut self) -> &mut H {
        match self {
            Self::Finalizing(h) => h,
            _ => panic!("state is not Finalizing: {self:?}"),
        }
    }
    fn as_final(&mut self) -> &mut T {
        match self {
            Self::Final(t) => t,
            _ => panic!("state is not Final: {self:?}"),
        }
    }
    fn to_hydrating(self) -> Self {
        match self {
            Self::Init(h) => Self::Hydrating(h),
            _ => panic!("state is not Init: {self:?}"),
        }
    }
    fn to_finalizing(self) -> Self {
        match self {
            Self::Hydrating(h) => Self::Finalizing(h),
            _ => panic!("state is not Hydrating: {self:?}"),
        }
    }
    fn to_final(self, t: T) -> Self {
        match self {
            Self::Finalizing(_) => Self::Final(t),
            _ => panic!("state is not Finalizing: {self:?}"),
        }
    }
}
macro_rules! create_state {
    ($($node: ident)+) => {
        paste! {
            $(
                type [<$node State>]<'i> = State<[<Hydrate $node >]<'i>, $node>;
                impl<'i> From<[<Hydrate $node>]<'i>> for [<$node State>]<'i> {
                    fn from(value: [<Hydrate $node >]<'_>) -> [<$node State>] {
                        [<$node State>]::Init(value)
                    }
                }
            )+
        }
    };
}
create_state!(Package File Message Enum Service Field Extension EnumValue Method Oneof);

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
    descriptor: &'i DescriptorProto,
    fqn: FullyQualifiedName,
    container: ContainerKey,
    fields: Vec<FieldKey>,
    oneofs: Vec<OneofKey>,
    extensions: Vec<ExtensionKey>,
    embeds: Vec<MessageKey>,
}

impl Finalize for HydrateMessage<'_> {
    type Target = Message;
    fn finalize(self, _hydrate: &mut HydrateAst) -> Self::Target {
        todo!()
    }
}

impl<'i> HydrateMessage<'i> {
    fn new(_desc: &DescriptorProto, container: HydrateContainer<'_, 'i>) -> Self {
        let _fqn = container.fqn().clone();
        todo!()
    }
}

#[derive(Debug, Clone)]
struct HydratePackage<'i> {
    name: String,
    fqn: FullyQualifiedName,
    messages: Vec<MessageKey>,
    enums: Vec<EnumKey>,
    services: Vec<ServiceKey>,
    files: Vec<FileKey>,
    // useless lifetime for uniformity
    phantom: std::marker::PhantomData<&'i ()>,
}

#[derive(Debug, Default, Clone)]
struct HydrateEnum<'i> {
    fqn: Option<FullyQualifiedName>,
    descriptor: Option<&'i EnumDescriptorProto>,
    values: Vec<EnumValueKey>,
    pkg: Option<PackageKey>,
    container: Option<ContainerKey>,
    file: FileKey,
    dependents: Vec<MessageKey>,
}

#[derive(Debug, Default)]
struct HydrateService<'i> {
    fqn: Option<FullyQualifiedName>,
    descriptor: Option<&'i ServiceDescriptorProto>,
    file: FileKey,
}

impl Finalize for HydrateService<'_> {
    type Target = Service;
    fn finalize(self, _hydrate: &mut HydrateAst) -> Self::Target {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateField<'i> {
    descriptor: Option<&'i FieldDescriptorProto>,
    msg: MessageKey,
}
impl Finalize for HydrateField<'_> {
    type Target = Field;
    fn finalize(self, _hydrate: &mut HydrateAst) -> Self::Target {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateEnumValue<'i> {
    descriptor: Option<&'i EnumDescriptorProto>,
    enum_: EnumKey,
}

impl Finalize for HydrateEnumValue<'_> {
    type Target = EnumValue;
    fn finalize(self, _hydrate: &mut HydrateAst) -> Self::Target {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateMethod<'i> {
    descriptor: Option<&'i MethodDescriptorProto>,
}

impl Finalize for HydrateMethod<'_> {
    type Target = Method;
    fn finalize(self, _hydrate: &mut HydrateAst) -> Self::Target {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateOneof<'i> {
    descriptor: Option<&'i OneofDescriptorProto>,
}

#[derive(Debug, Default)]
struct HydrateExtension<'i> {
    descriptor: Option<&'i FieldDescriptorProto>,
}

#[derive(Debug, Default)]
struct HydrateFile<'i> {
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

pub struct HydrateAst<'i> {
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

impl<'i> HydrateAst<'i> {
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

    fn init_file(&mut self, key: FileKey) -> Result<FileState, Error> {
        let _state = self.files.get_mut(key).unwrap();
        // let mut file = state.hydrate_mut();

        // for msg in file.descriptor.message_type {}
        todo!()
    }

    fn hydrate(&mut self, key: Key, stack: &mut Vec<Key>) -> Result<(), Error> {
        match key {
            Key::Package(key) => self.packages.get(key).unwrap().hydrate(self),
            Key::File(key) => key.hydrate(self),
            Key::Message(key) => key.hydrate(self),
            Key::Enum(key) => key.hydrate(self),
            Key::Service(key) => key.hydrate(self),
            Key::Field(key) => key.hydrate(self),
            Key::Extension(key) => key.hydrate(self),
            Key::EnumValue(key) => key.hydrate(self),
            Key::Method(key) => key.hydrate(self),
            Key::Oneof(key) => key.hydrate(self),
        }
        todo!()
    }
    fn run(mut self) -> Result<Ast, Error> {
        let mut stack: Vec<Key> = self.input.clone().into_iter().map(Into::into).collect();
        while let Some(next) = stack.pop() {
            self.hydrate(next, &mut stack)?;
        }
        todo!()
    }
}
