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
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error>;
    fn progress(&self) -> Progress;
}

trait Finalize: Default {
    type Target;
    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error>;
}

#[derive(Debug, Clone, Copy)]
enum Progress {
    Init,
    Hydrating,
    Finalizing,
}
impl Default for Progress {
    fn default() -> Self {
        Self::Init
    }
}

#[derive(Debug)]
enum State<H, T> {
    Hydrating(H),
    Final(T),
}

impl<H, T> State<H, T>
where
    H: Debug + Finalize<Target = T>,
    T: Debug,
{
    fn finalize(&mut self, hydrate: &mut HydrateAst) -> Result<(), Error> {
        match self {
            Self::Hydrating(node) => {
                let node = mem::replace(node, H::default());
                let finalized = node.finalize(hydrate)?;
                mem::replace(self, State::Final(finalized));
                Ok(())
            }
            Self::Final(_) => Ok(()),
        }
    }

    #[must_use]
    fn is_hydrating(&self) -> bool {
        matches!(self, Self::Hydrating(..))
    }

    #[must_use]
    fn is_final(&self) -> bool {
        matches!(self, Self::Final(..))
    }
}

impl<H, T> State<H, T>
where
    H: Debug + Hydrate,
{
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        match self {
            Self::Hydrating(p) => {
                p.hydrate(hydrate, stack)?;
                Ok(())
            }
            Self::Final(_) => Ok(()),
        }
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
    progress: Progress,
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

impl Hydrate for HydrateMessage<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}

impl Finalize for HydrateMessage<'_> {
    type Target = Message;
    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
struct HydratePackage<'i> {
    progress: Progress,
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

impl<'i> Hydrate for HydratePackage<'i> {
    fn progress(&self) -> Progress {
        self.progress
    }
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }
}

impl<'i> Finalize for HydratePackage<'i> {
    type Target = Package;
    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default, Clone)]
struct HydrateEnum<'i> {
    progress: Progress,
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

impl<'i> Hydrate for HydrateEnum<'i> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}

impl<'i> Finalize for HydrateEnum<'i> {
    type Target = Enum;
    fn finalize(mut self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateService<'i> {
    progress: Progress,
    fqn: Option<FullyQualifiedName>,
    descriptor: Option<&'i ServiceDescriptorProto>,
    file: FileKey,
}

impl Hydrate for HydrateService<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}

impl Finalize for HydrateService<'_> {
    type Target = Service;
    fn finalize(mut self, _hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateField<'i> {
    progress: Progress,
    descriptor: Option<&'i FieldDescriptorProto>,
    msg: MessageKey,
}

impl Hydrate for HydrateField<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}

impl Finalize for HydrateField<'_> {
    type Target = Field;
    fn finalize(mut self, _hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}
#[derive(Debug, Default)]
struct HydrateEnumValue<'i> {
    progress: Progress,
    descriptor: Option<&'i EnumDescriptorProto>,
    enum_: EnumKey,
}

impl Hydrate for HydrateEnumValue<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}
impl Finalize for HydrateEnumValue<'_> {
    type Target = EnumValue;

    fn finalize(mut self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateMethod<'i> {
    progress: Progress,
    descriptor: Option<&'i MethodDescriptorProto>,
}

impl Hydrate for HydrateMethod<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}
impl Finalize for HydrateMethod<'_> {
    type Target = Method;

    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateOneof<'i> {
    progress: Progress,
    descriptor: Option<&'i OneofDescriptorProto>,
}
impl Hydrate for HydrateOneof<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}
impl Finalize for HydrateOneof<'_> {
    type Target = Oneof;

    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateExtension<'i> {
    progress: Progress,
    descriptor: Option<&'i FieldDescriptorProto>,
}

impl Hydrate for HydrateExtension<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}

impl Finalize for HydrateExtension<'_> {
    type Target = Extension;

    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct HydrateFile<'i> {
    progress: Progress,
    fqn: FullyQualifiedName,
    descriptor: &'i FileDescriptorProto,
    msgs: Vec<MessageKey>,
    enums: Vec<EnumKey>,
    services: Vec<ServiceKey>,
    pkg: Option<PackageKey>,
}

impl Hydrate for HydrateFile<'_> {
    fn hydrate(&mut self, hydrate: &mut HydrateAst, stack: &mut Vec<Key>) -> Result<(), Error> {
        todo!()
    }

    fn progress(&self) -> Progress {
        self.progress
    }
}
impl Finalize for HydrateFile<'_> {
    type Target = File;

    fn finalize(self, hydrate: &mut HydrateAst) -> Result<Self::Target, Error> {
        todo!()
    }
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
            Key::Package(key) => self.packages.get_mut(key).unwrap().hydrate(self, stack),
            Key::File(key) => self.files.get_mut(key).unwrap().hydrate(self, stack),
            Key::Message(key) => self.messages.get_mut(key).unwrap().hydrate(self, stack),
            Key::Enum(key) => self.enums.get_mut(key).unwrap().hydrate(self, stack),
            Key::Service(key) => self.services.get_mut(key).unwrap().hydrate(self, stack),
            Key::Field(key) => self.fields.get_mut(key).unwrap().hydrate(self, stack),
            Key::Extension(key) => self.extensions.get_mut(key).unwrap().hydrate(self, stack),
            Key::EnumValue(key) => self.enum_values.get_mut(key).unwrap().hydrate(self, stack),
            Key::Method(key) => self.methods.get_mut(key).unwrap().hydrate(self, stack),
            Key::Oneof(key) => self.oneofs.get_mut(key).unwrap().hydrate(self, stack),
        }
    }
    fn run(mut self) -> Result<Ast, Error> {
        let mut stack: Vec<Key> = self.input.clone().into_iter().map(Into::into).collect();
        while let Some(next) = stack.pop() {
            self.hydrate(next, &mut stack)?;
        }
        todo!()
    }
}
