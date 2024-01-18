pub mod access;
pub mod container;
pub mod r#enum;
pub mod enum_value;
pub mod extension;
pub mod extension_block;
pub mod field;
pub mod file;
pub mod message;
pub mod method;
pub mod node;
pub mod oneof;
pub mod package;
pub mod path;
pub mod reference;
pub mod reserved;
pub mod service;
pub mod uninterpreted;

mod location;

use std::{
    fmt,
    iter::once,
    ops::{Deref, Index, IndexMut},
    path::Display,
    str::FromStr,
    sync::Arc,
};

use ahash::{HashMapExt, HashSetExt};
use protobuf::descriptor::{
    self, DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};
use slotmap::SlotMap;

use crate::{
    ast::file::{DependencyInner, DependentInner},
    error::Error,
    to_i32, HashMap, HashSet,
};
use r#enum::WellKnownEnum;
use field::Field;
use message::WellKnownMessage;

/// Zero-based spans of a node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start_line: i32,
    pub start_column: i32,
    pub end_line: i32,
    pub end_column: i32,
}
impl Span {
    fn new(span: &[i32]) -> Result<Self, ()> {
        match span.len() {
            3 => Ok(Self {
                start_line: span[0],
                start_column: span[1],
                end_line: span[0],
                end_column: span[2],
            }),
            4 => Ok(Self {
                start_line: span[0],
                start_column: span[1],
                end_line: span[2],
                end_column: span[3],
            }),
            _ => Err(()),
        }
    }
    pub fn start_line(&self) -> i32 {
        self.start_line
    }
    pub fn start_column(&self) -> i32 {
        self.start_column
    }
    pub fn end_line(&self) -> i32 {
        self.end_line
    }
    pub fn end_column(&self) -> i32 {
        self.end_column
    }
}

trait FromFqn {
    fn from_fqn(fqn: FullyQualifiedName) -> Self;
}

#[doc(hidden)]
trait Get<K, T> {
    fn get(&self, key: K) -> &T;
}

trait Resolve<T> {
    fn resolve(&self) -> &T;
}

struct Resolver<'ast, K, I> {
    ast: &'ast Ast,
    key: K,
    marker: std::marker::PhantomData<I>,
}

impl<'ast, K, I> Resolver<'ast, K, I> {
    const fn new(key: K, ast: &'ast Ast) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> Clone for Resolver<'ast, K, I>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        Self {
            ast: self.ast,
            key: self.key.clone(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> From<(K, &'ast Ast)> for Resolver<'ast, K, I> {
    fn from((key, ast): (K, &'ast Ast)) -> Self {
        Self {
            ast,
            key,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'ast, K, I> Copy for Resolver<'ast, K, I> where K: Copy {}

impl<'ast, K, I> PartialEq for Resolver<'ast, K, I>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<'ast, K, I> Eq for Resolver<'ast, K, I> where K: Eq {}

#[derive(Debug, Clone)]
struct Table<K, V>
where
    K: slotmap::Key,
{
    map: SlotMap<K, V>,
    lookup: HashMap<FullyQualifiedName, K>,
    order: Vec<K>,
}

impl<K, V> Table<K, V>
where
    K: slotmap::Key,
{
    fn with_capacity(len: usize) -> Self {
        Self {
            map: SlotMap::with_capacity_and_key(len),
            lookup: HashMap::with_capacity(len),
            order: Vec::with_capacity(len),
        }
    }
}

impl<K, V> Default for Table<K, V>
where
    K: slotmap::Key,
{
    fn default() -> Self {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::default(),
            order: Vec::default(),
        }
    }
}
impl<K, V> Table<K, V>
where
    K: slotmap::Key,
    V: access::FullyQualifiedName,
{
    fn get(&self, key: K) -> Option<&V> {
        self.map.get(key)
    }
    fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.map.get_mut(key)
    }
    fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.order.iter().map(move |key| (*key, &self.map[*key]))
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.map.iter_mut()
    }
    fn keys(&self) -> impl '_ + Iterator<Item = K> {
        self.order.iter().copied()
    }
    fn get_by_fqn(&self, fqn: &FullyQualifiedName) -> Option<&V> {
        self.lookup.get(fqn).map(|key| &self.map[*key])
    }
    fn get_mut_by_fqn(&mut self, fqn: &FullyQualifiedName) -> Option<&mut V> {
        self.lookup.get(fqn).map(|key| &mut self.map[*key])
    }
}

impl<K, V> Index<K> for Table<K, V>
where
    K: slotmap::Key,
{
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        &self.map[key]
    }
}
impl<K, V> IndexMut<K> for Table<K, V>
where
    K: slotmap::Key,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        &mut self.map[key]
    }
}

impl<K, V> Table<K, V>
where
    K: slotmap::Key,
    V: From<FullyQualifiedName> + access::Key<Key = K>,
{
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            lookup: HashMap::default(),
            order: Vec::new(),
        }
    }
    fn get_or_insert_key_by_fqn(&mut self, fqn: FullyQualifiedName) -> K {
        self.get_or_insert_by_fqn(fqn).0
    }

    fn get_or_insert_by_fqn(&mut self, fqn: FullyQualifiedName) -> (K, &mut V) {
        let key = *self
            .lookup
            .entry(fqn.clone())
            .or_insert_with(|| self.map.insert(fqn.into()));
        let value = &mut self.map[key];
        if value.key() != key {
            value.set_key(key);
        }
        (key, value)
    }
}

type PackageTable = Table<package::Key, package::Inner>;
type FileTable = Table<file::Key, file::Inner>;
type MessageTable = Table<message::Key, message::Inner>;
type EnumTable = Table<r#enum::Key, r#enum::Inner>;
type EnumValueTable = Table<enum_value::Key, enum_value::Inner>;
type ServiceTable = Table<service::Key, service::Inner>;
type MethodTable = Table<method::Key, method::Inner>;
type FieldTable = Table<field::Key, field::Inner>;
type OneofTable = Table<oneof::Key, oneof::Inner>;
type ExtensionTable = Table<extension::Key, extension::Inner>;
type ExtensionBlockTable = Table<extension_block::Key, extension_block::Inner>;

#[derive(Debug, Clone)]
struct Set<K> {
    set: Vec<K>,
    by_name: HashMap<Box<str>, K>,
}
impl<K> Default for Set<K> {
    fn default() -> Self {
        Self {
            set: Vec::default(),
            by_name: HashMap::default(),
        }
    }
}

impl<K> Set<K>
where
    K: Copy,
{
    fn from_vec(hydrated: Vec<Hydrated<K>>) -> Self {
        let mut set = Vec::with_capacity(hydrated.len());
        let mut by_name = HashMap::with_capacity(hydrated.len());
        for (key, _, name) in hydrated {
            set.push(key);
            by_name.insert(name, key);
        }
        Self { set, by_name }
    }
}
impl<K> Set<K>
where
    K: slotmap::Key + Copy,
{
    fn by_name(&self, name: &str) -> Option<K> {
        self.by_name.get(name).copied()
    }
    fn get(&self, index: usize) -> Option<K> {
        self.set.get(index).copied()
    }
    fn from_slice(hydrated: &[Hydrated<K>]) -> Self {
        let mut set = Vec::with_capacity(hydrated.len());
        let mut by_name = HashMap::with_capacity(hydrated.len());
        for (key, _, name) in hydrated {
            set.push(*key);
            by_name.insert(name.clone(), *key);
        }
        Self { set, by_name }
    }
}
impl<K> From<Vec<Hydrated<K>>> for Set<K>
where
    K: Copy,
{
    fn from(v: Vec<Hydrated<K>>) -> Self {
        Self::from_vec(v)
    }
}
impl Index<usize> for Set<package::Key> {
    type Output = package::Key;
    fn index(&self, index: usize) -> &Self::Output {
        &self.set[index]
    }
}

impl<K> PartialEq for Set<K>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}
impl<K> Eq for Set<K> where K: PartialEq {}

impl<K> Deref for Set<K> {
    type Target = [K];
    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

/// (key, fqn, name)
type Hydrated<K> = (K, FullyQualifiedName, Box<str>);

#[derive(Debug, Default)]
pub struct Ast {
    packages: PackageTable,
    files: FileTable,
    messages: MessageTable,
    enums: EnumTable,
    enum_values: EnumValueTable,
    services: ServiceTable,
    methods: MethodTable,
    fields: FieldTable,
    oneofs: OneofTable,
    extensions: ExtensionTable,
    extension_blocks: ExtensionBlockTable,
    nodes: HashMap<FullyQualifiedName, node::Key>,
    fqns: FullyQualifiedNames,
    well_known_package: package::Key,
}

impl Ast {
    fn new(file_descriptors: Vec<FileDescriptorProto>, targets: &[String]) -> Result<Self, Error> {
        Self {
            files: FileTable::with_capacity(file_descriptors.len()),
            ..Default::default()
        }
        .hydrate(file_descriptors, targets)
    }
    fn hydrate(
        mut self,
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &[String],
    ) -> Result<Self, Error> {
        let well_known = self.hydrate_package(Some("google.protobuf".to_string()));
        let mut by_fqn = HashMap::new();
        for descriptor in file_descriptors {
            let (key, fqn) = self.hydrate_file(descriptor, targets)?;
            self.nodes
                .extend(once((fqn, key.into())).chain(by_fqn.drain()));
        }
        Ok(self)
    }

    fn hydrate_file(
        &mut self,
        descriptor: FileDescriptorProto,
        targets: &[String],
    ) -> Result<(file::Key, FullyQualifiedName), Error> {
        // TODO: handle SpecialFields
        // let mut by_path = HashMap::default();
        let name = descriptor.name.unwrap();
        let locations = location::File::new(descriptor.source_code_info.unwrap_or_else(|| {
            panic!("source_code_info not found on FileDescriptorProto for \"{name}\"")
        }))?;

        let (package, package_fqn) = self.hydrate_package(descriptor.package);
        let fqn = self.fqns.get_or_insert(&name, package_fqn);
        let key = self.files.get_or_insert_key_by_fqn(fqn.clone());
        let messages = self.hydrate_messages(
            descriptor.message_type,
            locations.messages,
            fqn.clone(),
            key.into(),
            key,
            package,
        )?;
        let enums = self.hydrate_enums(
            descriptor.enum_type,
            locations.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
        );
        let services = self.hydrate_services(
            descriptor.service,
            locations.services,
            fqn.clone(),
            key,
            package,
        )?;
        let (extension_blocks, extensions) = self.hydrate_extensions(
            descriptor.extension,
            locations.extensions,
            key.into(),
            fqn,
            key,
            package,
        )?;
        let dependencies = self.hydrate_dependencies(
            key,
            descriptor.dependency,
            descriptor.public_dependency,
            descriptor.weak_dependency,
        );
        let file = &mut self.files[key];
        let is_build_target = targets.iter().any(|t| t == &name);
        file.hydrate(file::Hydrate {
            name: name.into_boxed_str(),
            syntax: descriptor.syntax,
            options: descriptor.options.unwrap_or_default(),
            package,
            messages,
            enums,
            services,
            extensions,
            extension_blocks,
            dependencies,
            package_comments: locations.package.and_then(|loc| loc.comments),
            comments: locations.syntax.and_then(|loc| loc.comments),
            is_build_target,
        })
    }

    fn hydrate_package(
        &mut self,
        package: Option<String>,
    ) -> (Option<package::Key>, Option<FullyQualifiedName>) {
        let Some(package) = package else {
            return (None, None);
        };

        let is_well_known = package == package::WELL_KNOWN;
        if package.is_empty() {
            return (None, None);
        }
        let fqn = self.fqns.for_package(&package);
        let (key, pkg) = self.packages.get_or_insert_by_fqn(fqn.clone());
        pkg.set_name(package);

        (Some(key), Some(fqn))
    }

    fn hydrate_dependencies(
        &mut self,
        dependent: file::Key,
        dependencies_by_fqn: Vec<String>,
        public_dependencies: Vec<i32>,
        weak_dependencies: Vec<i32>,
    ) -> file::DependenciesInner {
        let mut all = Vec::with_capacity(dependencies_by_fqn.len());
        let mut weak = Vec::with_capacity(weak_dependencies.len());
        let mut public = Vec::with_capacity(public_dependencies.len());

        for (i, dependency) in dependencies_by_fqn.into_iter().enumerate() {
            let index = to_i32(i);
            let is_weak = weak_dependencies.contains(&index);
            let is_public = public_dependencies.contains(&index);
            let fqn = self.fqns.insert(FullyQualifiedName::from(dependency));
            let (dependency_key, dependency_file) = self.files.get_or_insert_by_fqn(fqn.clone());
            dependency_file.add_dependent(DependentInner {
                is_used: bool::default(),
                is_public,
                is_weak,
                dependent,
                dependency: dependency_key,
            });
            let dep = DependencyInner {
                is_used: bool::default(),
                is_public,
                is_weak,
                dependent,
                dependency: dependency_key,
            };
            all.push(dep);

            if is_public {
                public.push(dep);
            }
            if is_weak {
                weak.push(dep);
            }
        }
        file::DependenciesInner {
            all,
            public,
            weak,
            unusued: Vec::default(),
        }
    }

    fn hydrate_messages(
        &mut self,
        descriptors: Vec<DescriptorProto>,
        locations: Vec<location::Message>,
        container_fqn: FullyQualifiedName,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Hydrated<message::Key>>, Error> {
        assert_message_locations(&container_fqn, &locations, &descriptors);
        let mut messages = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = self
                .fqns
                .get_or_insert(descriptor.name(), Some(container_fqn.clone()));

            let (key, fqn, name) = self.hydrate_message(
                descriptor,
                fqn,
                location,
                container,
                container_fqn.clone(),
                file,
                package,
            )?;
            self.nodes.insert(fqn.clone(), key.into());
            messages.push((key, fqn, name));
        }
        Ok(messages)
    }
    fn is_well_known(&self, package: Option<package::Key>) -> bool {
        if let Some(package) = package {
            return package == self.well_known_package;
        }
        false
    }
    #[allow(clippy::too_many_arguments)]
    fn hydrate_message(
        &mut self,
        descriptor: DescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Message,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Hydrated<message::Key>, Error> {
        let name = descriptor.name.unwrap_or_default().into();
        let key = self.messages.get_or_insert_key_by_fqn(fqn.clone());

        let (extension_blocks, extensions) = self.hydrate_extensions(
            descriptor.extension,
            location.extensions,
            container,
            container_fqn.clone(),
            file,
            package,
        )?;
        let messages = self.hydrate_messages(
            descriptor.nested_type,
            location.messages,
            fqn.clone(),
            key.into(),
            file,
            package,
        )?;
        let enums = self.hydrate_enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
        );
        let oneofs = self.hydrate_oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
        )?;
        let fields = self.hydrate_fields(
            descriptor.field,
            location.fields,
            key.into(),
            fqn,
            file,
            package,
        )?;
        let location = location.detail;
        Ok(self.messages[key].hydrate(message::Hydrate {
            name,
            fields,
            location,
            messages,
            oneofs,
            enums,
            extension_blocks,
            extensions,
            package,
            options: descriptor.options,
            reserved_names: descriptor.reserved_name,
            reserved_ranges: descriptor.reserved_range,
            special_fields: descriptor.special_fields,
            extension_range: descriptor.extension_range,
        }))
    }

    fn hydrate_enums(
        &mut self,
        descriptors: Vec<EnumDescriptorProto>,
        locations: Vec<location::Enum>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Vec<Hydrated<r#enum::Key>> {
        assert_enum_locations(&container_fqn, &locations, &descriptors);
        let mut enums = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let (key, fqn, name) =
                self.hydrate_enum(descriptor, fqn.clone(), location, container, file, package);
            self.nodes.insert(fqn.clone(), key.into());
            enums.push((key, fqn, name));
        }
        enums
    }

    fn hydrate_enum(
        &mut self,
        descriptor: EnumDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Enum,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Hydrated<r#enum::Key> {
        let name = descriptor.name.clone().unwrap_or_default().into_boxed_str();
        let key = self.enums.get_or_insert_key_by_fqn(fqn.clone());
        let values =
            self.hydrate_enum_values(descriptor.value, location.values, key, fqn, file, package);
        let well_known = if self.is_well_known(package) {
            WellKnownEnum::from_str(&name).ok()
        } else {
            None
        };
        self.enums[key].hydrate(r#enum::Hydrate {
            name,
            values,
            container,
            location: location.detail,
            options: descriptor.options,
            reserved_names: descriptor.reserved_name,
            reserved_ranges: descriptor.reserved_range,
            special_fields: descriptor.special_fields,
            well_known,
        })
    }

    fn hydrate_enum_values(
        &mut self,
        descriptors: Vec<EnumValueDescriptorProto>,
        locations: Vec<location::EnumValue>,
        r#enum: r#enum::Key,
        enum_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Vec<(enum_value::Key, FullyQualifiedName, Box<str>)> {
        assert_enum_value_locations(&enum_fqn, &locations, &descriptors);
        let mut values = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone()));
            let (key, fqn, name) =
                self.hydrate_enum_value(descriptor, fqn.clone(), location, r#enum, file, package);
            self.nodes.insert(fqn.clone(), key.into());
            values.push((key, fqn, name));
        }
        values
    }

    fn hydrate_enum_value(
        &mut self,
        descriptor: EnumValueDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::EnumValue,
        r#enum: r#enum::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Hydrated<enum_value::Key> {
        let key = self.enum_values.get_or_insert_key_by_fqn(fqn);
        let (key, fqn, name) = self.enum_values[key].hydrate(enum_value::Hydrate {
            name: descriptor.name().into(),
            number: descriptor.number(),
            location: location.detail,
            options: descriptor.options,
            special_fields: descriptor.special_fields,
            r#enum,
            file,
            package,
        });
        self.nodes.insert(fqn.clone(), key.into());
        (key, fqn, name)
    }

    fn hydrate_services(
        &mut self,
        descriptors: Vec<ServiceDescriptorProto>,
        locations: Vec<location::Service>,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Hydrated<service::Key>>, Error> {
        assert_service_locations(&container_fqn, &locations, &descriptors);
        let mut services = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let (key, fqn, name) =
                self.hydrate_service(descriptor, fqn.clone(), location, file, package)?;
            services.push((key, fqn.clone(), name));
            self.nodes.insert(fqn, key.into());
        }
        Ok(services)
    }

    fn hydrate_service(
        &mut self,
        descriptor: ServiceDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Service,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Hydrated<service::Key>, Error> {
        todo!()
    }

    fn hydrate_methods(
        &mut self,
        descriptors: Vec<MethodDescriptorProto>,
        locations: Vec<location::Method>,
        container_fqn: FullyQualifiedName,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<method::Key>, Error> {
        todo!()
    }

    fn hydrate_method(&mut self) -> Result<Hydrated<method::Key>, Error> {
        todo!()
    }

    fn hydrate_fields(
        &mut self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::Field>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Hydrated<field::Key>>, Error> {
        todo!()
    }

    fn hydrate_field(hydrate: Field) -> Result<Hydrated<field::Key>, Error> {
        todo!()
    }

    fn hydrate_oneofs(
        &mut self,
        descriptors: Vec<OneofDescriptorProto>,
        locations: Vec<location::Oneof>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Hydrated<oneof::Key>>, Error> {
        todo!()
    }

    fn hydrate_oneof(&mut self) -> Result<Hydrated<oneof::Key>, Error> {
        todo!()
    }

    fn hydrate_extensions(
        &mut self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::ExtensionBlock>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<(Vec<extension_block::Key>, Vec<Hydrated<extension::Key>>), Error> {
        // let mut services = Vec::with_capacity(service.len());
        // for (i, descriptor) in nodes.service.into_iter().enumerate() {
        //     let index = to_i32(i);
        //     let fqn = FullyQualifiedName::new(descriptor.name(), Some(fqn.clone()));
        //     let node_path = vec![path::File::Service.as_i32(), index];
        //     let key = hydrate_service(Service {
        //         fqn: fqn.clone(),
        //         descriptor,
        //         location: locations.services[i],
        //         nodes,
        //         file: key,
        //         index,
        //         package,
        //     })?;
        //     services.push(key);
        //     nodes_by_fqn.insert(fqn, key.into());
        //     nodes_by_path.insert(node_path, key.into());
        // }
        todo!()
    }
}

macro_rules! impl_resolve {

    ($($col:ident -> $mod:ident,)+) => {
        $(
            impl Get<$mod::Key, $mod::Inner> for Ast {
                fn get(&self, key: $mod::Key) -> &$mod::Inner {
                    &self.$col[key]
                }
            }
            impl<'ast> Resolve<$mod::Inner> for Resolver<'ast, $mod::Key, $mod::Inner>
            {
                fn resolve(&self) -> &$mod::Inner {
                    self.ast.get(self.key.clone())
                }
            }
            impl<'ast> Deref for Resolver<'ast, $mod::Key, $mod::Inner>{
                type Target = $mod::Inner;
                fn deref(&self) -> &Self::Target {
                    self.resolve()
                }
            }
            impl<'ast> fmt::Debug for Resolver<'ast, $mod::Key, $mod::Inner>
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_tuple("Resolver")
                        .field(&self.key)
                        .field(self.resolve())
                        .finish()
                }
            }
        )+
    };
}
fn assert_enum_locations(
    container_fqn: &str,
    locations: &[location::Enum],
    descriptors: &[EnumDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for enums in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_enum_value_locations(
    enum_fqn: &str,
    locations: &[location::EnumValue],
    descriptors: &[EnumValueDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for enum values in \"{enum_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_message_locations(
    container_fqn: &str,
    locations: &[location::Message],
    descriptors: &[DescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for messages in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_oneof_locations(
    message_fqn: &str,
    locations: &[location::Oneof],
    descriptors: &[OneofDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for oneofs in \"{message_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_service_locations(
    container_fqn: &str,
    locations: &[location::Service],
    descriptors: &[ServiceDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for services in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_method_locations(
    service_fqn: &str,
    locations: &[location::Method],
    descriptors: &[MethodDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for methods in \"{service_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_field_locations(
    message_fqn: &str,
    locations: &[location::Field],
    descriptors: &[FieldDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for fields in \"{message_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_extension_locations(
    container_fqn: &str,
    locations: &[location::ExtensionBlock],
    descriptors: &[FieldDescriptorProto],
) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of locations for extensions in \"{container_fqn}\", expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

fn assert_file_locations(locations: &[location::File], descriptors: &[FileDescriptorProto]) {
    assert_eq!(
        locations.len(),
        descriptors.len(),
        "invalid number of file locations for files , expected: {}, found: {}",
        descriptors.len(),
        locations.len()
    );
}

impl_resolve!(
    packages -> package,
    files -> file,
    messages -> message,
    enums -> r#enum,
    enum_values -> enum_value,
    oneofs -> oneof,
    services -> service,
    methods -> method,
    fields -> field,
    extensions -> extension,
    extension_blocks -> extension_block,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellKnownType {
    Enum(WellKnownEnum),
    Message(WellKnownMessage),
}

impl WellKnownType {
    pub const PACKAGE: &'static str = "google.protobuf";
}

#[derive(Default, Debug, Clone)]
struct FullyQualifiedNames(HashSet<Arc<str>>);
impl FullyQualifiedNames {
    fn new() -> Self {
        Self(HashSet::default())
    }

    fn get_or_insert(
        &mut self,
        value: &str,
        container: Option<FullyQualifiedName>,
    ) -> FullyQualifiedName {
        self.insert(FullyQualifiedName::new(value, container))
    }
    fn for_package(&mut self, package_name: &str) -> FullyQualifiedName {
        if package_name.starts_with('.') && self.0.contains(package_name) {
            return self.0.get(package_name).unwrap().clone().into();
        }
        self.insert(FullyQualifiedName::from_package_name(package_name))
    }

    fn insert(&mut self, fqn: FullyQualifiedName) -> FullyQualifiedName {
        if let Some(fqn) = self.0.get(&fqn.0) {
            return FullyQualifiedName(fqn.clone());
        }
        self.0.insert(fqn.0.clone());
        fqn
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName(Arc<str>);

impl AsRef<str> for FullyQualifiedName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Arc<str>> for FullyQualifiedName {
    fn from(value: Arc<str>) -> Self {
        Self(value)
    }
}
impl From<&str> for FullyQualifiedName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl Deref for FullyQualifiedName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for FullyQualifiedName {
    fn default() -> Self {
        Self("".into())
    }
}
impl From<String> for FullyQualifiedName {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl FullyQualifiedName {
    pub fn new(value: impl AsRef<str>, container: Option<Self>) -> Self {
        let value = value.as_ref();
        if value.is_empty() {
            if let Some(fqn) = container {
                return fqn;
            }
            return Self::default();
        }
        let container = container.unwrap_or_default();
        if container.is_empty() {
            return Self(value.into());
        }
        Self(format!("{container}.{value}").into())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    fn push(&mut self, value: impl AsRef<str>) {
        let value = value.as_ref();
        if value.is_empty() {
            return;
        }
        let mut existing = self.0.to_string();
        if !self.0.is_empty() {
            existing.push('.');
        }
        existing.push_str(value);
        self.0 = existing.into();
    }
    fn from_package_name(package_name: impl AsRef<str>) -> Self {
        let package_name = package_name.as_ref();
        if package_name.is_empty() {
            return Self::default();
        }
        if package_name.starts_with('.') {
            Self(package_name.into())
        } else {
            Self(format!(".{package_name}").into())
        }
    }
}

impl fmt::Display for FullyQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Comments {
    /// Any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    leading: Option<Box<str>>,
    trailing: Option<Box<str>>,
    leading_detached: Vec<Box<str>>,
}

impl Comments {
    pub fn new_maybe(
        leading: Option<String>,
        trailing: Option<String>,
        leading_detacted: Vec<String>,
    ) -> Option<Self> {
        if leading.is_none() && trailing.is_none() && leading_detacted.is_empty() {
            return None;
        }
        let leading = leading.map(String::into_boxed_str);
        let trailing = trailing.map(String::into_boxed_str);
        let leading_detached = leading_detacted
            .into_iter()
            .map(String::into_boxed_str)
            .collect();
        Some(Self {
            leading,
            trailing,
            leading_detached,
        })
    }
    /// Any comment immediately preceding the node, without any
    /// whitespace between it and the comment.
    pub fn leading(&self) -> Option<&str> {
        self.leading.as_deref()
    }

    /// Any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub fn trailing(&self) -> Option<&str> {
        self.trailing.as_deref()
    }

    /// Each comment block or line above the entity but seperated by whitespace.
    pub fn leading_detached(&self) -> &[Box<str>] {
        &self.leading_detached
    }
}

macro_rules! impl_key {
    ($inner:ident, $key:ident) => {
        impl crate::ast::access::Key for $inner {
            type Key = $key;
            fn key(&self) -> Self::Key {
                self.key
            }
            fn key_mut(&mut self) -> &mut Self::Key {
                &mut self.key
            }
        }
        impl $inner {
            pub(super) fn set_key(&mut self, key: $key) {
                self.key = key;
            }
        }
    };
}

// macro_rules! impl_fsm {
//     ($inner:ident) => {
//         impl crate::ast::Fsm for $inner {}
//     };
// }
macro_rules! impl_access {
    ($node: ident, $key: ident, $inner: ident) => {
        impl<'ast> crate::ast::Resolve<$inner> for $node<'ast> {
            fn resolve(&self) -> &$inner {
                crate::ast::Resolve::resolve(&self.0)
            }
        }
    };
}
macro_rules! impl_clone_copy {
    ($node:ident) => {
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl<'ast> Clone for $node<'ast> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<'ast> Copy for $node<'ast> {}
    };
}
macro_rules! impl_access_fqn {
    ($node:ident, $key:ident, $inner: ident) => {
        impl<'ast> crate::ast::access::FullyQualifiedName for $node<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::Resolve;
                &self.resolve().fqn
            }
        }
        impl<'ast> $node<'ast> {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                use crate::ast::Resolve;
                &self.resolve().fqn
            }
            fn fqn(&self) -> &crate::ast::FullyQualifiedName {
                self.fully_qualified_name()
            }
        }
        impl<'ast> crate::ast::access::FullyQualifiedName for $inner {
            fn fully_qualified_name(&self) -> &crate::ast::FullyQualifiedName {
                &self.fqn
            }
        }
    };
}

macro_rules! impl_from_fqn {
    ($inner:ident) => {
        impl From<crate::ast::FullyQualifiedName> for $inner {
            fn from(fqn: crate::ast::FullyQualifiedName) -> Self {
                let mut this = Self::default();
                this.fqn = fqn;
                this
            }
        }
        impl crate::ast::FromFqn for $inner {
            fn from_fqn(fqn: crate::ast::FullyQualifiedName) -> Self {
                fqn.into()
            }
        }
    };
}
macro_rules! impl_eq {
    ($node:ident) => {
        impl<'ast> PartialEq for $node<'ast> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<'ast> Eq for $node<'ast> {}
    };
    () => {};
}

macro_rules! impl_from_key_and_ast {
    ($node:ident, $key:ident, $inner:ident) => {
        impl<'ast> From<($key, &'ast crate::ast::Ast)> for $node<'ast> {
            fn from((key, ast): ($key, &'ast crate::ast::Ast)) -> Self {
                Self(crate::ast::Resolver::new(key, ast))
            }
        }
        impl<'ast> From<crate::ast::Resolver<'ast, $key, $inner>> for $node<'ast> {
            fn from(resolver: crate::ast::Resolver<'ast, $key, $inner>) -> Self {
                Self(resolver)
            }
        }
    };
}

macro_rules! impl_fmt {
    ($node: ident, $key: ident, $inner: ident) => {
        impl<'ast> ::std::fmt::Display for $node<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Resolve;
                ::std::fmt::Display::fmt(&self.resolve().fqn, f)
            }
        }

        impl<'ast> ::std::fmt::Debug for $node<'ast> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use crate::ast::Resolve;
                ::std::fmt::Debug::fmt(self.resolve(), f)
            }
        }
    };
}
macro_rules! impl_access_reserved {
    ($node: ident, $inner:ident) => {
        impl $inner {
            fn set_reserved<R>(&mut self, names: Vec<String>, ranges: Vec<R>)
            where
                R: Into<crate::ast::reserved::ReservedRange>,
            {
                self.reserved = crate::ast::reserved::Reserved {
                    names: names.into(),
                    ranges: ranges.into_iter().map(Into::into).collect(),
                };
            }
        }
        impl<'ast> $node<'ast> {
            pub fn reserved_names(&self) -> &[String] {
                &self.0.reserved.names
            }
            pub fn reserved_ranges(&self) -> &[crate::ast::reserved::ReservedRange] {
                &self.0.reserved.ranges
            }
            pub fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
        impl<'ast> crate::ast::access::Reserved for $node<'ast> {
            fn reserved(&self) -> &crate::ast::reserved::Reserved {
                &self.0.reserved
            }
        }
    };
}

macro_rules! impl_access_file {
    ($node:ident, $inner: ident) => {
        impl<'ast> crate::ast::access::File<'ast> for $node<'ast> {
            fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
        impl<'ast> $node<'ast> {
            pub fn file(self) -> crate::ast::file::File<'ast> {
                (self.0.file, self.0.ast).into()
            }
        }
    };
}
macro_rules! impl_access_package {
    ($node: ident, $inner: ident) => {
        impl<'ast> crate::ast::access::Package<'ast> for $node<'ast> {
            fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }

        impl<'ast> $node<'ast> {
            pub fn package(self) -> Option<crate::ast::package::Package<'ast>> {
                self.0.package.map(|key| (key, self.0.ast).into())
            }
        }
    };
}
macro_rules! set_unknown_fields {
    ($inner:ident) => {
        pub(super) fn set_unknown_fields<T>(&mut self, fields: impl IntoIterator<Item = T>)
        where
            T: Into<crate::ast::UnknownFields>,
        {
            self.unknown_fields = fields.into_iter().map(Into::into);
        }
    };
}

macro_rules! impl_set_uninterpreted_options {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn set_uninterpreted_options(
                &mut self,
                opts: Vec<protobuf::descriptor::UninterpretedOption>,
            ) {
                self.uninterpreted_options = opts.into_iter().map(Into::into).collect();
            }
        }
    };
}
macro_rules! impl_access_name {
    ($node:ident, $inner: ident) => {
        impl $inner {
            pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
                self.name = name.into();
            }
        }
        impl<'ast> crate::ast::access::Name for $node<'ast> {
            fn name(&self) -> &str {
                &self.0.name
            }
        }
        impl<'ast> $node<'ast> {
            pub fn name(&self) -> &str {
                &self.0.name
            }
        }
    };
}

macro_rules! inner_method_package {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn package(&self) -> Option<crate::ast::package::Key> {
                self.package
            }
            pub(super) fn set_package(&mut self, package: Option<crate::ast::package::Key>) {
                self.package = package;
            }
        }
    };
}
macro_rules! inner_method_file {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn file(&self) -> crate::ast::file::Key {
                self.file
            }
            pub(super) fn set_file(&mut self, file: crate::ast::file::Key) {
                self.file = file;
            }
        }
    };
}

macro_rules! node_method_ast {
    ($node:ident) => {
        impl<'ast> $node<'ast> {
            pub(crate) fn ast(self) -> &'ast crate::ast::Ast {
                self.0.ast
            }
        }
    };
}
macro_rules! node_method_new {
    ($node:ident, $key:ident) => {
        impl<'ast> $node<'ast> {
            pub(super) fn new(key: $key, ast: &'ast crate::ast::Ast) -> Self {
                Self((key, ast).into())
            }
        }
    };
}
macro_rules! node_method_key {
    ($node:ident, $key: ident) => {
        impl<'ast> $node<'ast> {
            pub(crate) fn key(self) -> $key {
                self.0.key
            }
        }
    };
}

macro_rules! impl_span {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::Span for $node<'ast> {
            fn span(&self) -> crate::ast::Span {
                self.0.span
            }
        }
        impl<'ast> $node<'ast> {
            pub fn span(&self) -> crate::ast::Span {
                self.0.span
            }
        }

        impl $inner {
            pub(super) fn set_span(&mut self, span: crate::ast::Span) {
                self.span = span;
            }
        }
    };
}

macro_rules! inner_method_hydrate_location {
    ($inner:ident) => {
        impl $inner {
            pub(super) fn hydrate_location(&mut self, location: crate::ast::location::Location) {
                self.comments = location.comments;
                self.span = location.span;
                self.node_path = location.path.into();
            }
        }
    };
}

macro_rules! impl_comments {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::Comments for $node<'ast> {
            fn comments(&self) -> Option<&crate::ast::Comments> {
                self.0.comments.as_ref()
            }
        }
        impl<'ast> $node<'ast> {
            pub fn comments(&self) -> Option<&crate::ast::Comments> {
                self.0.comments.as_ref()
            }
        }

        impl $inner {
            pub(super) fn set_comments(&mut self, comments: crate::ast::Comments) {
                self.comments = Some(comments);
            }
        }
    };
}

macro_rules! impl_node_path {
    ($node:ident, $inner:ident) => {
        impl<'ast> crate::ast::access::NodePath for $node<'ast> {
            fn node_path(&self) -> &[i32] {
                &self.0.node_path
            }
        }
        impl<'ast> $node<'ast> {
            pub fn node_path(&self) -> &[i32] {
                crate::ast::access::NodePath::node_path(self)
            }
        }
        impl $inner {
            pub(super) fn set_node_path(&mut self, path: Vec<i32>) {
                self.node_path = path.into();
            }
        }
    };
}

macro_rules! impl_base_traits_and_methods {
    ($node:ident, $key:ident, $inner:ident) => {
        crate::ast::impl_key!($inner, $key);
        crate::ast::node_method_new!($node, $key);
        crate::ast::node_method_key!($node, $key);
        crate::ast::node_method_ast!($node);
        crate::ast::impl_clone_copy!($node);
        crate::ast::impl_eq!($node);
        crate::ast::impl_access!($node, $key, $inner);
        crate::ast::impl_access_fqn!($node, $key, $inner);
        crate::ast::impl_from_key_and_ast!($node, $key, $inner);
        crate::ast::impl_fmt!($node, $key, $inner);
        crate::ast::impl_from_fqn!($inner);
        crate::ast::impl_access_name!($node, $inner);
        // crate::ast::impl_state!($inner);
        // crate::ast::impl_fsm!($inner);
    };
}
macro_rules! impl_traits_and_methods {
    (Package, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Package, $key, $inner);
    };

    (File, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(File, $key, $inner);
        crate::ast::impl_access_package!(File, $inner);
        crate::ast::impl_comments!(File, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::inner_method_package!($inner);
    };

    (Message, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Message, $key, $inner);
        crate::ast::impl_access_reserved!(Message, $inner);
        crate::ast::impl_access_file!(Message, $inner);
        crate::ast::impl_access_package!(Message, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::impl_node_path!(Message, $inner);
        crate::ast::impl_span!(Message, $inner);
        crate::ast::impl_comments!(Message, $inner);
        crate::ast::inner_method_file!($inner);
        crate::ast::inner_method_package!($inner);
        crate::ast::inner_method_hydrate_location!($inner);
    };

    (Enum, $key:ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!(Enum, $key, $inner);
        crate::ast::impl_access_reserved!(Enum, $inner);
        crate::ast::impl_access_file!(Enum, $inner);
        crate::ast::impl_access_package!(Enum, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::impl_node_path!(Enum, $inner);
        crate::ast::impl_span!(Enum, $inner);
        crate::ast::impl_comments!(Enum, $inner);
        crate::ast::inner_method_file!($inner);
        crate::ast::inner_method_package!($inner);
        crate::ast::inner_method_hydrate_location!($inner);
    };

    ($node: ident, $key: ident, $inner: ident) => {
        crate::ast::impl_base_traits_and_methods!($node, $key, $inner);
        crate::ast::impl_access_file!($node, $inner);
        crate::ast::impl_access_package!($node, $inner);
        crate::ast::impl_set_uninterpreted_options!($inner);
        crate::ast::impl_node_path!($node, $inner);
        crate::ast::impl_span!($node, $inner);
        crate::ast::impl_comments!($node, $inner);
        crate::ast::inner_method_file!($inner);
        crate::ast::inner_method_package!($inner);
        crate::ast::inner_method_hydrate_location!($inner);
    };
}

use impl_access;
use impl_access_file;
use impl_access_fqn;
use impl_access_name;
use impl_access_package;
use impl_access_reserved;
use impl_base_traits_and_methods;
use impl_clone_copy;
use impl_comments;
use impl_eq;
use impl_fmt;
use impl_from_fqn;
use impl_from_key_and_ast;
use impl_key;
use impl_node_path;
use impl_set_uninterpreted_options;
use impl_span;
use impl_traits_and_methods;
use inner_method_file;
use inner_method_hydrate_location;
use inner_method_package;
use node_method_ast;
use node_method_key;
use node_method_new;

use self::file::Hydrate;
// use impl_state;

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_fully_qualified_name() {
        // let fqn = FullyQualifiedName::new("foo", None);
        // assert_eq!(fqn.as_str(), ".foo");

        // let fqn = FullyQualifiedName::new("foo",
        // Some(FullyQualifiedName("bar".into())); assert_eq!(fqn.
        // as_str(), "bar.foo");

        // let fqn = FullyQualifiedName::new("foo", Some(".bar"));
        // assert_eq!(fqn.as_str(), ".bar.foo");
    }
}
