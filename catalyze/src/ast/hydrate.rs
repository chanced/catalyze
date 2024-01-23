use ahash::HashMapExt;
use crossbeam::{
    channel,
    deque::{Injector, Stealer, Worker},
    sync::WaitGroup,
};
use itertools::Itertools;
use std::{
    self,
    ops::{Add, ControlFlow},
    path::PathBuf,
    str::FromStr,
    thread,
};

use crate::HashMap;
use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};

use crate::{error::Error, to_i32, CPU_COUNT};
use parking_lot::Mutex;

use super::{
    container,
    r#enum::{self},
    enum_value, extension, extension_decl, field,
    file::{self},
    location,
    message::{self},
    method,
    node::{self, ExtendNodes, Ident},
    oneof, package, service, EnumTable, EnumValueTable, ExtensionDeclTable, ExtensionTable,
    FieldTable, FileTable, FullyQualifiedName, MessageTable, MethodTable, OneofTable, PackageTable,
    ServiceTable,
};

type Receiver = channel::Receiver<Result<Completed, Error>>;
#[derive(Clone)]
struct Sender(channel::Sender<Result<Completed, Error>>);
impl Sender {
    fn send(&self, result: Result<Completed, Error>) -> ControlFlow<()> {
        match self.0.send(result) {
            Ok(()) => ControlFlow::Continue(()),
            Err(_) => ControlFlow::Break(()),
        }
    }
}

enum Completed {
    Link(Link),
    Finalize(Finalize),
}

impl From<Job> for Completed {
    fn from(job: Job) -> Self {
        match job {
            Job::Populate(_) => unreachable!(),
            Job::Link(link) => Self::Link(link),
            Job::Finalize(finalize) => Self::Finalize(finalize),
        }
    }
}
impl From<Finalize> for Completed {
    fn from(v: Finalize) -> Self {
        Self::Finalize(v)
    }
}

impl From<Link> for Completed {
    fn from(v: Link) -> Self {
        Self::Link(v)
    }
}

struct Link {
    file: file::Key,
    fqn: FullyQualifiedName,
    nodes: HashMap<FullyQualifiedName, node::Key>,
    dependencies: Vec<file::DependencyInner>,
    dependents: Vec<file::DependentInner>,
}

struct Finalize {
    key: file::Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    path: PathBuf,
}
enum Job {
    Populate(FileDescriptorProto),
    Link(Link),
    Finalize(Finalize),
}

impl From<Link> for Job {
    fn from(v: Link) -> Self {
        Self::Link(v)
    }
}

impl From<FileDescriptorProto> for Job {
    fn from(v: FileDescriptorProto) -> Self {
        Self::Populate(v)
    }
}

pub(crate) fn run(
    file_descriptors: Vec<FileDescriptorProto>,
    targets: &[String],
) -> Result<super::Ast, Error> {
    Hydrate::new(file_descriptors, targets).map(Into::into)
}

pub(super) struct Message {
    descriptor: DescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Message,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
}

fn x(p: PathBuf) {
    println!("{}", p.display());
}

struct Enum {
    descriptor: EnumDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Enum,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct EnumValue {
    descriptor: EnumValueDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::EnumValue,
    r#enum: r#enum::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Service {
    descriptor: ServiceDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Service,
    file: file::Key,
    package: Option<package::Key>,
}

struct Method {
    fqn: FullyQualifiedName,
    location: location::Method,
    descriptor: MethodDescriptorProto,
    service: service::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Field {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Field,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Oneof {
    descriptor: OneofDescriptorProto,
    fqn: FullyQualifiedName,
    location: location::Oneof,
    message: message::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct Extension {}

struct ExtensionDecl {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
    location: location::ExtensionDecl,
}

#[derive(Default)]
struct Hydrate<'input> {
    packages: Mutex<PackageTable>,
    files: Mutex<FileTable>,
    messages: Mutex<MessageTable>,
    enums: Mutex<EnumTable>,
    enum_values: Mutex<EnumValueTable>,
    services: Mutex<ServiceTable>,
    methods: Mutex<MethodTable>,
    fields: Mutex<FieldTable>,
    oneofs: Mutex<OneofTable>,
    extensions: Mutex<ExtensionTable>,
    extension_blocks: Mutex<ExtensionDeclTable>,
    well_known: package::Key,

    targets: &'input [String],
    injector: Injector<Job>,
    stealers: Vec<Stealer<Job>>,
}

#[allow(clippy::from_over_into)]
impl Into<super::Ast> for Hydrate<'_> {
    fn into(self) -> super::Ast {
        let nodes = HashMap::default();
        super::Ast {
            packages: self.packages.into_inner(),
            files: self.files.into_inner(),
            messages: self.messages.into_inner(),
            enums: self.enums.into_inner(),
            enum_values: self.enum_values.into_inner(),
            services: self.services.into_inner(),
            methods: self.methods.into_inner(),
            fields: self.fields.into_inner(),
            oneofs: self.oneofs.into_inner(),
            extensions: self.extensions.into_inner(),
            extension_blocks: self.extension_blocks.into_inner(),
            well_known_package: self.well_known,
            nodes,
        }
    }
}

macro_rules! key_methods {
    ($(($mod:ident, $col:ident) => $key_fn:ident,)+) => {
        impl Hydrate<'_> {
            $(fn $key_fn(&self, fqn: FullyQualifiedName) -> $mod::Key {
                self.$col.lock().get_or_insert_key(fqn)
            })+
        }
    };
}

key_methods!(
    (package, packages) =>  package_key,
    (file, files) =>  file_key,
    (message, messages) =>  message_key,
    (r#enum, enums) =>  enum_key,
    (enum_value, enum_values) =>  enum_value_key,
    (service, services) =>  service_key,
    (method, methods) =>  method_key,
    (field, fields) =>  field_key,
    (oneof, oneofs) =>  oneof_key,
    (extension, extensions) =>  extension_key,
);

#[derive(Default)]
struct Accumulated {
    nodes: HashMap<FullyQualifiedName, node::Key>,
    by_name: HashMap<Box<str>, file::Key>,
    by_path: HashMap<PathBuf, file::Key>,
}

impl<'input> Hydrate<'input> {
    fn new(
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &'input [String],
    ) -> Result<Self, Error> {
        let (well_known, packages) = create_packages_table();
        let (workers, stealers) = create_workers();
        let (sender, receiver) = channel::unbounded();

        let injector = Injector::new();
        Self {
            injector,
            targets,
            stealers,
            well_known,
            packages,
            files: Mutex::new(FileTable::with_capacity(file_descriptors.len())),
            ..Default::default()
        }
        .start(file_descriptors, Sender(sender), receiver, workers)
    }

    fn start(
        mut self,
        descriptors: Vec<FileDescriptorProto>,
        sender: Sender,
        receiver: Receiver,
        workers: Vec<Worker<Job>>,
    ) -> Result<Self, Error> {
        let wg = WaitGroup::new();
        let worker_count = workers.len();
        let res = thread::scope(|scope| {
            let acc_handle = scope.spawn(move || self.accumulate(receiver));
            let mut worker_handles = Vec::with_capacity(worker_count);
            for worker in workers.into_iter() {
                let sender = sender.clone();
                let wg = wg.clone();
                let worker =
                    worker_handles.push(scope.spawn(move || self.work(worker, sender, wg.clone())));
            }
            drop(sender);
            for handle in worker_handles {
                handle.join().unwrap();
            }
            acc_handle.join().unwrap()
        });
        todo!()
        // Ok(self)
    }

    fn link(&self, link: Link) -> Result<Finalize, Error> {
        todo!()
    }

    fn work(&self, local: Worker<Job>, results: Sender, wg: WaitGroup) {
        let mut wg = Some(wg);

        let link = while let Some(job) = self.find_work(&local) {
            if match job {
                Job::Populate(desc) => results.send(self.file(desc).map(Completed::Link)),
                Job::Link(link) => {
                    if let Some(wg) = wg.take() {
                        wg.wait();
                    }
                    results.send(self.link(link).map(Completed::Finalize))
                }
                Job::Finalize(done) => ControlFlow::Continue(()),
            }
            .is_break()
            {
                break;
            }
        };
    }

    fn queue(&self, descriptors: Vec<FileDescriptorProto>) {
        for descriptor in descriptors {
            self.injector.push(Job::Populate(descriptor));
        }
    }

    fn accumulate(&self, results: Receiver) -> Result<Accumulated, Error> {
        let mut acc = Accumulated::default();
        loop {
            if let Ok(result) = results.recv() {
                match result? {
                    Completed::Link(_) => todo!(),
                    Completed::Finalize(_) => todo!(),
                }
                todo!()
            } else {
                return Ok(acc);
            }
        }
    }

    fn find_work(&self, local: &Worker<Job>) -> Option<Job> {
        local
            .pop()
            .or_else(|| self.injector.steal().success())
            .or_else(|| self.stealers.iter().find_map(|s| s.steal().success()))
    }

    fn file(&self, descriptor: FileDescriptorProto) -> Result<Link, Error> {
        let name = descriptor.name.unwrap_or_default().into_boxed_str();
        let locations = location::File::new(descriptor.source_code_info.unwrap_or_else(|| {
            panic!("source_code_info not found on FileDescriptorProto for \"{name}\"")
        }))?;
        let is_build_target = self.targets.iter().any(|t| t == name.as_ref());

        let (package, package_fqn) = self.package(descriptor.package);
        let fqn = FullyQualifiedName::new(&name, package_fqn);
        let key = self.file_key(fqn.clone());

        let messages = self.messages(
            descriptor.message_type,
            locations.messages,
            fqn.clone(),
            key.into(),
            key,
            package,
        )?;

        let enums = self.enums(
            descriptor.enum_type,
            locations.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
        );

        let services = self.services(
            descriptor.service,
            locations.services,
            fqn.clone(),
            key,
            package,
        )?;

        let (extension_blocks, extensions) = self.extensions(
            descriptor.extension,
            locations.extensions,
            key.into(),
            fqn,
            key,
            package,
        )?;

        let dependencies = self.dependencies(
            key,
            descriptor.dependency,
            descriptor.public_dependency,
            descriptor.weak_dependency,
        );
        let file = &mut self.files.lock()[key];
        let Ident { key, fqn, name } = file.hydrate(file::Hydrate {
            name,
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
        })?;
        todo!()
    }
    fn package(
        &self,
        package: Option<String>,
    ) -> (Option<package::Key>, Option<FullyQualifiedName>) {
        let Some(package) = package else {
            return (None, None);
        };
        if package.is_empty() {
            return (None, None);
        }
        let fqn = FullyQualifiedName::for_package(&package);
        let key = self.package_key(fqn.clone());
        let mut packages = self.packages.lock();
        packages.get_mut(key).unwrap().populate(package);
        (Some(key), Some(fqn))
    }

    fn insert_dependency(
        &self,
        index: i32,
        fqn: FullyQualifiedName,
        dependent: file::Key,
        is_public: bool,
        is_weak: bool,
    ) -> file::DependencyInner {
        let mut files = self.files.lock();
        let (dependency_key, dependency_file) = files.get_or_insert(fqn.clone());
        let dependency = file::DependencyInner {
            is_used: bool::default(),
            is_public,
            is_weak,
            dependent,
            dependency: dependency_key,
        };
        dependency_file.add_dependent(dependency.into());
        dependency
    }

    fn dependencies(
        &self,
        dependent: file::Key,
        dependencies_by_fqn: Vec<String>,
        public_dependencies: Vec<i32>,
        weak_dependencies: Vec<i32>,
    ) -> file::DependenciesInner {
        let mut direct = Vec::with_capacity(dependencies_by_fqn.len());
        let mut weak = Vec::with_capacity(weak_dependencies.len());
        let mut public = Vec::with_capacity(public_dependencies.len());

        for (i, dependency) in dependencies_by_fqn.into_iter().enumerate() {
            let index = to_i32(i);
            let is_weak = weak_dependencies.contains(&index);
            let is_public = public_dependencies.contains(&index);
            let fqn = FullyQualifiedName(dependency.into());
            let dependency = self.insert_dependency(index, fqn, dependent, is_public, is_weak);
            direct.push(dependency);
            if is_public {
                public.push(dependency);
            }
            if is_weak {
                weak.push(dependency);
            }
        }
        file::DependenciesInner {
            transitive: Vec::default(),
            direct,
            public,
            weak,
            unusued: Vec::default(),
        }
    }

    fn messages(
        &self,
        descriptors: Vec<DescriptorProto>,
        locations: Vec<location::Message>,
        container_fqn: FullyQualifiedName,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Ident<message::Key>>, Error> {
        assert_message_locations(&container_fqn, &locations, &descriptors);
        assert_message_locations(&container_fqn, &locations, &descriptors);
        let mut messages = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let populated = self.message(Message {
                descriptor,
                fqn,
                location,
                container,
                file,
                package,
            })?;
            messages.push(populated);
        }

        Ok(messages)
    }

    fn is_well_known(&self, package: Option<package::Key>) -> bool {
        if let Some(package) = package {
            return package == self.well_known;
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn message(&self, hydrate: Message) -> Result<Ident<message::Key>, Error> {
        let Message {
            descriptor,
            fqn,
            location,
            container,
            file,
            package,
        } = hydrate;
        let key = self.messages.lock().get_or_insert_key(fqn.clone());
        let name = descriptor.name.unwrap_or_default().into_boxed_str();
        let well_known = if self.is_well_known(package) {
            message::WellKnownMessage::from_str(&name).ok()
        } else {
            None
        };

        let direct_node_count = descriptor
            .nested_type
            .len()
            .add(descriptor.enum_type.len())
            .add(descriptor.extension.len())
            .add(descriptor.oneof_decl.len())
            .add(descriptor.field.len());

        let mut descendants: HashMap<FullyQualifiedName, node::Key> =
            HashMap::with_capacity(direct_node_count);
        let (extension_blocks, extensions) = self.extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn.clone(),
            file,
            package,
        )?;
        descendants.extend_nodes(extensions.iter());

        let messages = self.messages(
            descriptor.nested_type,
            location.messages,
            fqn.clone(),
            key.into(),
            file,
            package,
        )?;

        let enums = self.enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
        );

        let oneofs = self.oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
        )?;

        let fields = self.fields(
            descriptor.field,
            location.fields,
            key.into(),
            fqn,
            file,
            package,
        )?;

        let location = location.detail;
        let populated = self.messages.lock()[key].hydrate(message::Hydrate {
            name,
            container,
            fields,
            location,
            messages,
            oneofs,
            enums,
            well_known,
            extension_blocks,
            extensions,
            package,
            options: descriptor.options,
            reserved_names: descriptor.reserved_name,
            reserved_ranges: descriptor.reserved_range,
            special_fields: descriptor.special_fields,
            extension_range: descriptor.extension_range,
        });
        Ok(populated)
    }

    fn enums(
        &self,
        descriptors: Vec<EnumDescriptorProto>,
        locations: Vec<location::Enum>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Vec<Ident<r#enum::Key>> {
        assert_enum_locations(&container_fqn, &locations, &descriptors);
        let mut enums = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let populated =
                self.r#enum(descriptor, fqn.clone(), location, container, file, package);
            enums.push(populated);
        }
        enums
    }

    fn r#enum(
        &self,
        descriptor: EnumDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::Enum,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Ident<r#enum::Key> {
        let name = descriptor.name.clone().unwrap_or_default().into_boxed_str();
        let key = self.enums.lock().get_or_insert_key(fqn.clone());
        let values = self.enum_values(descriptor.value, location.values, key, fqn, file, package);
        let well_known = if self.is_well_known(package) {
            r#enum::WellKnownEnum::from_str(&name).ok()
        } else {
            None
        };
        self.enums.lock()[key].hydrate(r#enum::Hydrate {
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

    fn enum_values(
        &self,
        descriptors: Vec<EnumValueDescriptorProto>,
        locations: Vec<location::EnumValue>,
        r#enum: r#enum::Key,
        enum_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Vec<Ident<enum_value::Key>> {
        assert_enum_value_locations(&enum_fqn, &locations, &descriptors);
        let mut values = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone()));
            let ident = self.enum_value(descriptor, fqn.clone(), location, r#enum, file, package);
            values.push(ident);
        }
        values
    }

    fn enum_value(
        &self,
        descriptor: EnumValueDescriptorProto,
        fqn: FullyQualifiedName,
        location: location::EnumValue,
        r#enum: r#enum::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Ident<enum_value::Key> {
        let mut enum_values = self.enum_values.lock();
        let key = enum_values.get_or_insert_key(fqn);
        enum_values[key].hydrate(enum_value::Hydrate {
            name: descriptor.name().into(),
            number: descriptor.number(),
            location: location.detail,
            options: descriptor.options,
            special_fields: descriptor.special_fields,
            r#enum,
            file,
            package,
        })
    }

    fn services(
        &self,
        descriptors: Vec<ServiceDescriptorProto>,
        locations: Vec<location::Service>,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Ident<service::Key>>, Error> {
        assert_service_locations(&container_fqn, &locations, &descriptors);
        let mut services = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let ident = self.service(Service {
                descriptor,
                fqn,
                location,
                file,
                package,
            })?;
            services.push(ident);
        }
        Ok(services)
    }

    fn service(&self, service: Service) -> Result<Ident<service::Key>, Error> {
        let Service {
            file,
            package,
            descriptor,
            fqn,
            location,
        } = service;
        todo!()
    }

    fn methods(
        &self,
        descriptors: Vec<MethodDescriptorProto>,
        locations: Vec<location::Method>,
        container_fqn: FullyQualifiedName,
        container: container::Key,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Ident<method::Key>>, Error> {
        todo!()
    }

    fn method(&self) -> Result<Ident<method::Key>, Error> {
        todo!()
    }

    fn fields(
        &self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::Field>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Ident<field::Key>>, Error> {
        todo!()
    }

    fn field(&self) -> Result<Ident<field::Key>, Error> {
        todo!()
    }

    fn oneofs(
        &self,
        descriptors: Vec<OneofDescriptorProto>,
        locations: Vec<location::Oneof>,
        message: message::Key,
        message_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<Vec<Ident<oneof::Key>>, Error> {
        todo!()
    }

    fn oneof(&self) -> Result<Ident<oneof::Key>, Error> {
        todo!()
    }

    fn extensions(
        &self,
        descriptors: Vec<FieldDescriptorProto>,
        locations: Vec<location::ExtensionDecl>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
    ) -> Result<(Vec<extension_decl::Key>, Vec<Ident<extension::Key>>), Error> {
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
    locations: &[location::ExtensionDecl],
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

fn create_workers() -> (Vec<Worker<Job>>, Vec<Stealer<Job>>) {
    let worker_count = (CPU_COUNT - 1).min(1);
    let workers = (0..worker_count)
        .map(|_| Worker::<Job>::new_lifo())
        .collect_vec();
    let stealers = workers.iter().map(Worker::stealer).collect_vec();
    (workers, stealers)
}

fn create_packages_table() -> (package::Key, Mutex<PackageTable>) {
    let mut packages = PackageTable::with_capacity(1);
    let key = packages.get_or_insert_key(FullyQualifiedName::for_package("google.protobuf"));
    (key, Mutex::new(packages))
}
