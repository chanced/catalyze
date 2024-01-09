// use std::{fmt::Debug, path::PathBuf};

// use ahash::HashMapExt;
// use indexmap::IndexSet;
// use itertools::Itertools;
// use protobuf::descriptor::{
//     DescriptorProto, EnumDescriptorProto, FieldDescriptorProto,
// FileDescriptorProto,     MethodDescriptorProto, OneofDescriptorProto,
// ServiceDescriptorProto, };
// use slotmap::SlotMap;

// use crate::{
//     ast::{Fqn, FullyQualifiedName},
//     r#enum, enum_value,
//     error::Error,
//     extension, field,
//     file::{self, File},
//     message, method, oneof, package, service, HashMap, HashSet,
// };

// use super::Ast;

// #[inline]
// pub fn hydrate(
//     files: &[protobuf::descriptor::FileDescriptorProto],
//     targets: &HashSet<PathBuf>,
// ) -> Result<Ast, Error> {
//     Hydrate::new(files, targets).run()
// }

// type PackageState = State<package::Key, HydratePackage>;
// type FileState<'i> = State<file::Key, HydrateFile<'i>>;
// type MessageState<'i> = State<message::Key, HydrateMessage<'i>>;
// type EnumState<'i> = State<r#enum::Key, HydrateEnum<'i>>;
// type ServiceState<'i> = State<service::Key, HydrateService<'i>>;
// type FieldState<'i> = State<field::Key, HydrateField<'i>>;
// type ExtensionState<'i> = State<extension::Key, HydrateExtension<'i>>;
// type EnumValueState<'i> = State<enum_value::Key, HydrateEnumValue<'i>>;
// type MethodState<'i> = State<method::Key, HydrateMethod<'i>>;
// type OneofState<'i> = State<oneof::Key, HydrateOneof<'i>>;

// impl FileState<'_> {
//     #[must_use]
//     fn name(&self, ast: &Ast) -> &str {
//         match self {
//             State::Hydrating(hydrate) => hydrate.descriptor.name(),
//             State::Final(key) => File::from((*key, ast)).name(),
//         }
//     }
//     #[must_use]
//     fn fully_qualified_name(&self, ast: &Ast) -> &FullyQualifiedName {
//         match self {
//             State::Hydrating(hydrate) => &hydrate.fqn,
//             State::Final(key) => File::from((*key,
// ast)).fully_qualified_name(),         }
//     }
// }

// impl From<HydratePackage> for PackageState {
//     fn from(v: HydratePackage) -> Self {
//         Self::Hydrating(v)
//     }
// }
// impl<'i> From<HydrateFile<'i>> for FileState<'i> {
//     fn from(v: HydrateFile<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateMessage<'i>> for MessageState<'i> {
//     fn from(v: HydrateMessage<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateEnum<'i>> for EnumState<'i> {
//     fn from(v: HydrateEnum<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateService<'i>> for ServiceState<'i> {
//     fn from(v: HydrateService<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateField<'i>> for FieldState<'i> {
//     fn from(v: HydrateField<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateExtension<'i>> for ExtensionState<'i> {
//     fn from(v: HydrateExtension<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateEnumValue<'i>> for EnumValueState<'i> {
//     fn from(v: HydrateEnumValue<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateMethod<'i>> for MethodState<'i> {
//     fn from(v: HydrateMethod<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// impl<'i> From<HydrateOneof<'i>> for OneofState<'i> {
//     fn from(v: HydrateOneof<'i>) -> Self {
//         Self::Hydrating(v)
//     }
// }

// #[derive(Debug, Default)]
// struct HydrateFile<'i> {
//     key: file::Key,
//     // progress: Progress,
//     fqn: FullyQualifiedName,
//     descriptor: &'i FileDescriptorProto,
//     msgs: IndexSet<MessageKey>,
//     enums: IndexSet<EnumKey>,
//     services: IndexSet<ServiceKey>,
//     pkg: Option<PackageKey>,
//     dependents: IndexSet<FileKey>,
// }
// impl<'i> HydrateFile<'i> {
//     fn new(descriptor: &'i FileDescriptorProto) -> HydrateFile<'i> {
//         Self {
//             descriptor,
//             ..Default::default()
//         }
//     }
// }

// pub struct Hydrate<'i> {
//     packages: SlotMap<PackageKey, PackageState>,
//     files: SlotMap<FileKey, FileState<'i>>,
//     messages: SlotMap<MessageKey, MessageState<'i>>,
//     enums: SlotMap<EnumKey, EnumState<'i>>,
//     services: SlotMap<ServiceKey, ServiceState<'i>>,
//     fields: SlotMap<FieldKey, FieldState<'i>>,
//     extensions: SlotMap<ExtensionKey, ExtensionState<'i>>,
//     enum_values: SlotMap<EnumValueKey, EnumValueState<'i>>,
//     methods: SlotMap<MethodKey, MethodState<'i>>,
//     oneofs: SlotMap<OneofKey, OneofState<'i>>,
//     targets: &'i HashSet<PathBuf>,
//     input: Vec<FileKey>,
//     file_lookup: HashMap<&'i str, FileKey>,
//     pkg_lookup: HashMap<FullyQualifiedName, HydratePackage>,
//     ast: Ast,
// }

// impl<'i> Hydrate<'i> {
//     fn new(
//         files: &'i [protobuf::descriptor::FileDescriptorProto],
//         targets: &'i HashSet<PathBuf>,
//     ) -> Self {
//         let mut file_map = SlotMap::with_key();
//         let mut file_lookup = HashMap::with_capacity(files.len());
//         let mut input = Vec::with_capacity(files.len());
//         for descriptor in files {
//             let key = file_map.insert(HydrateFile::new(descriptor).into());
//             input.push(key);
//             file_lookup.insert(descriptor.name(), key);
//         }

//         Self {
//             ast: Ast::default(),
//             packages: SlotMap::with_key(),
//             files: file_map,
//             messages: SlotMap::with_key(),
//             enums: SlotMap::with_key(),
//             services: SlotMap::with_key(),
//             fields: SlotMap::with_key(),
//             extensions: SlotMap::with_key(),
//             enum_values: SlotMap::with_key(),
//             methods: SlotMap::with_key(),
//             oneofs: SlotMap::with_key(),
//             pkg_lookup: HashMap::new(),
//             input,
//             targets,
//             file_lookup,
//         }
//     }

//     fn hydrate_node(&mut self, key: Key, stack: &mut Vec<Key>) -> Result<(),
// Error> {         match key {
//             Key::Package(key) => self.hydrate_package(key, stack),
//             Key::File(key) => self.hydrate_file(key, stack),
//             Key::Message(key) => self.hydrate_message(key, stack),
//             Key::Enum(key) => self.hydrate_enum(key, stack),
//             Key::Service(key) => self.hydrate_service(key, stack),
//             Key::Field(key) => self.hydrate_field(key, stack),
//             Key::Extension(key) => self.hydrate_extension(key, stack),
//             Key::EnumValue(key) => self.hydrate_enum_value(key, stack),
//             Key::Method(key) => self.hydrate_method(key, stack),
//             Key::Oneof(key) => self.hydrate_oneof(key, stack),
//         }
//     }

//     fn hydrate_file(&mut self, key: FileKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         let file = self.files.get(key).unwrap();
//         let Some(file) = file.as_hydrating() else {
//             return Ok(());
//         };
//         let dependencies =
// self.collect_dependency_keys(&file.descriptor.dependency);         let file =
// self.files.get_mut(key).unwrap().as_hydrating_mut().unwrap();

//         let pkg_name = file.descriptor.package();
//         let pkg = if pkg_name.is_empty() {
//             None
//         } else {
//             let pkg_fqn = FullyQualifiedName::new(pkg_name, None);
//             Some(self.pkg_lookup.entry(pkg_fqn).or_default())
//         };

//         let fqn = FullyQualifiedName::new(
//             file.descriptor.name(),
//             pkg.map(|pkg| pkg.fully_qualified_name().clone()),
//         );

//         for msg in &file.descriptor.message_type {
//             let state = HydrateMessage::new(msg, fqn.clone(),
// ContainerKey::File(key), key).into();             let key =
// self.messages.insert(state);             file.msgs.insert(key);
//             stack.push(key.into());
//         }
//         for enum_ in &file.descriptor.enum_type {
//             let key = self
//                 .enums
//                 .insert(HydrateEnum::new(enum_, fqn.clone(),
// ContainerKey::File(key), key).into());             file.enums.insert(key);
//             stack.push(key.into());
//         }
//         for service in &file.descriptor.service {
//             let key = self
//                 .services
//                 .insert(HydrateService::new(service, fqn.clone(),
// key).into());             file.services.insert(key);
//             stack.push(key.into());
//         }
//         for extension in &file.descriptor.extension {
//             let state =
//                 HydrateExtension::new(extension, fqn.clone(),
// ContainerKey::File(key), key).into();             let key =
// self.extensions.insert(state);         }
//         for dep in dependencies {
//             self.connect_file_dependency(dep, key);
//         }
//         todo!()
//     }
//     fn collect_dependency_keys(&self, dependencies: &[String]) ->
// Vec<FileKey> {         dependencies
//             .iter()
//             .map(|dep| {
//                 self.file_lookup.get(dep.as_str()).unwrap_or_else(|| {
//                     panic!("File dependency not found: \"{dep}\"");
//                 })
//             })
//             .copied()
//             .collect_vec()
//     }
//     fn connect_file_dependency(&mut self, dependent: FileKey, dependency:
// FileKey) {         // adding dependency link to dependent
//         let file = self.files.get_mut(dependent).unwrap();
//         let file = file.as_hydrating_mut().expect(
//             "File is already
// finalized",
//         );
//         file.dependencies.insert(dependency);

//         // adding dependent link to dependency
//         let file = self.files.get_mut(dependency).unwrap();
//         let file = file.as_hydrating_mut().expect(
//             "File is already
// finalized",
//         );
//         file.dependents.insert(dependent);
//     }

//     fn hydrate_package(&mut self, key: PackageKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }

//     fn hydrate_message(&mut self, key: MessageKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }
//     fn hydrate_enum(&mut self, key: EnumKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }
//     fn hydrate_service(&mut self, key: ServiceKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }
//     fn hydrate_field(&mut self, key: FieldKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }
//     fn hydrate_extension(&mut self, key: ExtensionKey, stack: &mut Vec<Key>)
// -> Result<(), Error> {         todo!()
//     }
//     fn hydrate_enum_value(&mut self, key: EnumValueKey, stack: &mut Vec<Key>)
// -> Result<(), Error> {         todo!()
//     }
//     fn hydrate_method(&mut self, key: MethodKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }
//     fn hydrate_oneof(&mut self, key: OneofKey, stack: &mut Vec<Key>) ->
// Result<(), Error> {         todo!()
//     }

//     fn init_file(&mut self, descriptor: &'i FileDescriptorProto) -> FileKey {
//         for (key, file) in &self.files {
//             if file.name(&self.ast) == descriptor.name.as_deref().unwrap() {
//                 return key;
//             }
//         }
//         let state = HydrateFile::new(descriptor).into();
//         self.files.insert(state)
//     }

//     fn run(mut self) -> Result<Ast, Error> {
//         let mut hydrate_stack: Vec<Key> =
// self.input.iter().copied().map(Into::into).collect();         let mut
// final_stack = hydrate_stack.clone();         self.
// hydrate_stack(hydrate_stack)?;         todo!()
//     }
// }

// slotmap::new_key_type! {
//     struct MessageKey;
//     struct PackageKey;
//     struct EnumKey;
//     struct ServiceKey;
//     struct FieldKey;
//     struct FileKey;
//     struct ExtensionKey;
//     struct EnumValueKey;
//     struct MethodKey;
//     struct OneofKey;
// }

// #[derive(Debug)]
// enum State<K, H> {
//     Hydrating(H),
//     Final(K),
// }

// impl<T, H> State<T, H> {
//     #[must_use]
//     const fn is_hydrating(&self) -> bool {
//         matches!(self, Self::Hydrating(..))
//     }

//     #[must_use]
//     const fn is_final(&self) -> bool {
//         matches!(self, Self::Final(..))
//     }

//     #[must_use]
//     const fn as_hydrating(&self) -> Option<&H> {
//         if let Self::Hydrating(v) = self {
//             Some(v)
//         } else {
//             None
//         }
//     }
//     #[must_use]
//     fn as_hydrating_mut(&mut self) -> Option<&mut H> {
//         if let Self::Hydrating(v) = self {
//             Some(v)
//         } else {
//             None
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum Key {
//     Package(PackageKey),
//     File(FileKey),
//     Message(MessageKey),
//     Enum(EnumKey),
//     Service(ServiceKey),
//     Field(FieldKey),
//     Extension(ExtensionKey),
//     EnumValue(EnumValueKey),
//     Method(MethodKey),
//     Oneof(OneofKey),
// }
// impl From<PackageKey> for Key {
//     fn from(v: PackageKey) -> Self {
//         Self::Package(v)
//     }
// }
// impl From<FileKey> for Key {
//     fn from(v: FileKey) -> Self {
//         Self::File(v)
//     }
// }
// impl From<MessageKey> for Key {
//     fn from(v: MessageKey) -> Self {
//         Self::Message(v)
//     }
// }
// impl From<EnumKey> for Key {
//     fn from(v: EnumKey) -> Self {
//         Self::Enum(v)
//     }
// }
// impl From<ServiceKey> for Key {
//     fn from(v: ServiceKey) -> Self {
//         Self::Service(v)
//     }
// }
// impl From<FieldKey> for Key {
//     fn from(v: FieldKey) -> Self {
//         Self::Field(v)
//     }
// }
// impl From<ExtensionKey> for Key {
//     fn from(v: ExtensionKey) -> Self {
//         Self::Extension(v)
//     }
// }
// impl From<EnumValueKey> for Key {
//     fn from(v: EnumValueKey) -> Self {
//         Self::EnumValue(v)
//     }
// }
// impl From<MethodKey> for Key {
//     fn from(v: MethodKey) -> Self {
//         Self::Method(v)
//     }
// }
// impl From<OneofKey> for Key {
//     fn from(v: OneofKey) -> Self {
//         Self::Oneof(v)
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum ContainerKey {
//     Message(MessageKey),
//     File(FileKey),
// }
// impl Default for ContainerKey {
//     fn default() -> Self {
//         Self::File(FileKey::default())
//     }
// }

// impl From<FileKey> for ContainerKey {
//     fn from(v: FileKey) -> Self {
//         Self::File(v)
//     }
// }
// impl From<MessageKey> for ContainerKey {
//     fn from(key: MessageKey) -> Self {
//         Self::Message(key)
//     }
// }

// #[derive(Debug, Default, Clone)]
// struct HydrateMessage<'i> {
//     descriptor: &'i DescriptorProto,
//     fqn: FullyQualifiedName,
//     container: ContainerKey,
//     fields: IndexSet<FieldKey>,
//     oneofs: IndexSet<OneofKey>,
//     extensions: IndexSet<ExtensionKey>,
//     embeds: IndexSet<MessageKey>,
// }
// impl Fqn for HydrateMessage<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }

// impl<'i> HydrateMessage<'i> {
//     fn new(
//         descriptor: &'i DescriptorProto,
//         container_fqn: FullyQualifiedName,
//         container: ContainerKey,
//         file: FileKey,
//     ) -> Self {
//         let mut fqn = container_fqn;
//         fqn.push(descriptor.name());
//         Self {
//             descriptor,
//             fqn,
//             container,
//             embeds: IndexSet::new(),
//             extensions: IndexSet::new(),
//             fields: IndexSet::new(),
//             oneofs: IndexSet::new(),
//         }
//     }
// }

// #[derive(Debug, Clone, Default)]
// struct HydratePackage {
//     // progress: Progress,
//     name: String,
//     fqn: FullyQualifiedName,
//     messages: IndexSet<MessageKey>,
//     enums: IndexSet<EnumKey>,
//     services: IndexSet<ServiceKey>,
//     files: IndexSet<FileKey>,
// }

// impl Fqn for HydratePackage {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }

// #[derive(Debug, Default, Clone)]
// struct HydrateEnum<'i> {
//     // progress: Progress,
//     fqn: FullyQualifiedName,
//     descriptor: &'i EnumDescriptorProto,
//     values: IndexSet<EnumValueKey>,
//     container: ContainerKey,
//     file: FileKey,
//     dependents: IndexSet<MessageKey>,
// }
// impl Fqn for HydrateEnum<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }
// impl<'i> HydrateEnum<'i> {
//     fn new(
//         descriptor: &'i EnumDescriptorProto,
//         container_fqn: FullyQualifiedName,
//         container: ContainerKey,
//         file: FileKey,
//     ) -> Self {
//         let mut fqn = container_fqn;
//         fqn.push(descriptor.name());
//         Self {
//             descriptor,
//             fqn,
//             container,
//             dependents: IndexSet::new(),
//             file,
//             values: IndexSet::new(),
//         }
//     }
// }

// #[derive(Debug, Default)]
// struct HydrateService<'i> {
//     fqn: FullyQualifiedName,
//     descriptor: &'i ServiceDescriptorProto,
//     file: FileKey,
//     methods: IndexSet<MethodKey>,
// }
// impl Fqn for HydrateService<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }
// impl<'i> HydrateService<'i> {
//     fn new(
//         descriptor: &'i ServiceDescriptorProto,
//         file_fqn: FullyQualifiedName,
//         key: FileKey,
//     ) -> Self {
//         let mut fqn = file_fqn;
//         fqn.push(descriptor.name());
//         Self {
//             descriptor,
//             file: key,
//             fqn,
//             methods: IndexSet::new(),
//         }
//     }
// }

// #[derive(Debug, Default)]
// struct HydrateField<'i> {
//     fqn: FullyQualifiedName,
//     descriptor: &'i FieldDescriptorProto,
//     msg: MessageKey,
// }

// impl Fqn for HydrateField<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }

// #[derive(Debug, Default)]
// struct HydrateEnumValue<'i> {
//     fqn: FullyQualifiedName,
//     descriptor: &'i FieldDescriptorProto,
//     enum_: EnumKey,
//     file: FileKey,
// }
// impl<'i> HydrateEnumValue<'i> {
//     fn new(
//         descriptor: &'i FieldDescriptorProto,
//         enum_fqn: FullyQualifiedName,
//         enum_: EnumKey,
//         file: FileKey,
//     ) -> Self {
//         Self {
//             fqn: FullyQualifiedName::new(descriptor.name(), Some(enum_fqn)),
//             descriptor,
//             enum_,
//             file,
//         }
//     }
// }

// impl Fqn for HydrateEnumValue<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }
// #[derive(Debug, Default)]
// struct HydrateMethod<'i> {
//     fqn: FullyQualifiedName,
//     descriptor: &'i MethodDescriptorProto,
// }
// impl Fqn for HydrateMethod<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }
// #[derive(Debug, Default)]
// struct HydrateOneof<'i> {
//     fqn: FullyQualifiedName,
//     descriptor: &'i OneofDescriptorProto,
// }
// impl Fqn for HydrateOneof<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }

// #[derive(Debug, Default)]
// struct HydrateExtension<'i> {
//     fqn: FullyQualifiedName,
//     descriptor: &'i FieldDescriptorProto,
//     container_key: ContainerKey,
//     file: FileKey,
// }

// impl<'i> HydrateExtension<'i> {
//     fn new(
//         descriptor: &'i FieldDescriptorProto,
//         container_fqn: FullyQualifiedName,
//         container_key: ContainerKey,
//         file: FileKey,
//     ) -> Self {
//         let mut fqn = container_fqn;
//         fqn.push(descriptor.name.as_deref().unwrap());
//         HydrateExtension {
//             fqn,
//             descriptor,
//             container_key,
//             file,
//         }
//     }
// }
// impl Fqn for HydrateExtension<'_> {
//     fn fully_qualified_name(&self) -> &FullyQualifiedName {
//         &self.fqn
//     }
// }
