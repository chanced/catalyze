use ahash::HashMapExt;
use crossbeam::{
    channel,
    deque::{Injector, Stealer, Worker},
    sync::WaitGroup,
};
use itertools::Itertools;
use std::{
    self,
    ops::ControlFlow,
    path::PathBuf,
    str::FromStr,
    thread::{self},
};

use crate::HashMap;
use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};

use crate::{error::Error, to_i32};
use parking_lot::Mutex;

use super::{
    container,
    r#enum::{self},
    enum_value, extension, extension_decl, field,
    file::{self, DependenciesInner},
    location,
    message::{self},
    method,
    node::{self, Ident},
    oneof, package, service, EnumTable, EnumValueTable, ExtensionDeclTable, ExtensionTable,
    FieldTable, FileTable, FullyQualifiedName, MessageTable, MethodTable, OneofTable, PackageTable,
    ServiceTable,
};

type Nodes = HashMap<FullyQualifiedName, node::Key>;
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
            Job::Hydrate(_) => unreachable!(),
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
    dependencies: DependenciesInner,
}

struct Finalize {
    key: file::Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    path: PathBuf,
    nodes: HashMap<FullyQualifiedName, node::Key>,
}
enum Job {
    Hydrate(FileDescriptorProto),
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
        Self::Hydrate(v)
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
    files_by_name: HashMap<Box<str>, file::Key>,
    files_by_path: HashMap<PathBuf, file::Key>,
}

impl<'input> Hydrate<'input> {
    fn new(
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &'input [String],
    ) -> Result<Self, Error> {
        let (well_known, packages) = create_packages_table();
        let (workers, stealers) = create_workers(file_descriptors.len());
        let (sender, receiver) = channel::unbounded();
        let sender = Sender(sender);
        let injector = Injector::new();
        let this = Self {
            injector,
            targets,
            stealers,
            well_known,
            packages,
            files: Mutex::new(FileTable::with_capacity(file_descriptors.len())),
            ..Default::default()
        };
        this.run(file_descriptors, sender, receiver, workers)?;
        Ok(this)
    }

    fn run(
        &self,
        descriptors: Vec<FileDescriptorProto>,
        sender: Sender,
        receiver: Receiver,
        workers: Vec<Worker<Job>>,
    ) -> Result<(), Error> {
        let wg = WaitGroup::new();
        let worker_count = workers.len();

        for descriptor in descriptors {
            self.injector.push(Job::Hydrate(descriptor));
        }

        let res = thread::scope(|scope| {
            let acc_handle = scope.spawn(move || self.accumulate(receiver));
            let mut worker_handles = Vec::with_capacity(worker_count);
            for worker in workers {
                let sender = sender.clone();
                let wg = wg.clone();
                worker_handles.push(scope.spawn(move || self.work(worker, sender, wg)));
            }
            drop(sender);
            for handle in worker_handles {
                handle.join().unwrap();
            }
            acc_handle.join().unwrap()
        })?;

        Ok(())
    }

    fn link(&self, link: Link) -> Result<Finalize, Error> {
        todo!()
    }

    fn work(&self, local: Worker<Job>, results: Sender, wg: WaitGroup) {
        let mut wg = Some(wg);
        while let Some(job) = self.find_work(&local) {
            if match job {
                Job::Hydrate(desc) => results.send(self.file(desc).map(Completed::Link)),
                Job::Link(link) => {
                    if let Some(a) = wg.take() {
                        WaitGroup::wait(a);
                    }
                    results.send(self.link(link).map(Completed::Finalize))
                }
                Job::Finalize(done) => ControlFlow::Continue(()),
            }
            .is_break()
            {
                break;
            }
        }
    }

    fn queue(&self, descriptors: Vec<FileDescriptorProto>) {
        for descriptor in descriptors {
            self.injector.push(Job::Hydrate(descriptor));
        }
    }

    fn link_dependencies(&self) {
        todo!()
    }
    fn completed(&self, acc: &mut Accumulated, completed: Completed) -> Result<(), Error> {
        match completed {
            Completed::Link(file) => {
                self.injector.push(Job::Finalize(self.link(file)?));
            }
            Completed::Finalize(finalize) => {
                acc.files_by_name.insert(finalize.name, finalize.key);
                acc.files_by_path.insert(finalize.path, finalize.key);
                acc.nodes.extend(finalize.nodes);
            }
        }
        Ok(())
    }

    fn accumulate(&self, results: Receiver) -> Result<Accumulated, Error> {
        let mut acc = Accumulated::default();
        loop {
            if let Ok(completed) = results.recv() {
                self.completed(&mut acc, completed?)?;
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
        let mut nodes = HashMap::with_capacity(locations.node_count);
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
            &mut nodes,
        )?;

        let enums = self.enums(
            descriptor.enum_type,
            locations.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        );

        let services = self.services(
            descriptor.service,
            locations.services,
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let (extension_blocks, extensions) = self.extensions(
            descriptor.extension,
            locations.extensions,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let dependencies = self.dependencies(
            key,
            descriptor.dependency,
            descriptor.public_dependency,
            descriptor.weak_dependency,
        );

        let file = &mut self.files.lock()[key];

        let ident = file.hydrate(file::Hydrate {
            name,
            nodes: nodes.clone(),
            syntax: descriptor.syntax,
            options: descriptor.options.unwrap_or_default(),
            package,
            messages,
            enums,
            services,
            extensions,
            extension_blocks,
            dependencies: dependencies.clone(),
            package_comments: locations.package.and_then(|loc| loc.comments),
            comments: locations.syntax.and_then(|loc| loc.comments),
            is_build_target,
        })?;

        Ok(Link {
            file: ident.key,
            fqn,
            nodes,
            dependencies,
        })
    }

    fn package(
        &self,
        package: Option<String>,
    ) -> (Option<package::Key>, Option<FullyQualifiedName>) {
        let Some(name) = package else {
            return (None, None);
        };
        if name.is_empty() {
            return (None, None);
        }
        let fqn = FullyQualifiedName::for_package(&name);
        let mut packages = self.packages.lock();
        let (key, pkg) = packages.get_or_insert_key_and_value(fqn.clone());
        pkg.hydrate(name);
        drop(packages);
        (Some(key), Some(fqn))
    }

    fn dependencies(
        &self,
        dependent: file::Key,
        dependencies: Vec<String>,
        public_dependencies: Vec<i32>,
        weak_dependencies: Vec<i32>,
    ) -> file::DependenciesInner {
        let mut direct = Vec::with_capacity(dependencies.len());
        let mut weak = Vec::with_capacity(weak_dependencies.len());
        let mut public = Vec::with_capacity(public_dependencies.len());

        for (i, dependency) in dependencies.into_iter().enumerate() {
            let index = to_i32(i);
            let is_weak = weak_dependencies.contains(&index);
            let is_public = public_dependencies.contains(&index);
            let fqn = FullyQualifiedName(dependency.into());
            let dependency_file = self.file_key(fqn.clone());
            let dependency = file::DependencyInner {
                is_used: bool::default(),
                is_public,
                is_weak,
                dependent,
                dependency: dependency_file,
            };
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
        nodes: &mut Nodes,
    ) -> Result<Vec<Ident<message::Key>>, Error> {
        assert_message_locations(&container_fqn, &locations, &descriptors);
        let mut messages = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let hydrate = Message {
                descriptor,
                fqn,
                location,
                container,
                file,
                package,
            };
            let msg = self.message(hydrate, nodes)?;
            nodes.insert(msg.fqn(), msg.node_key());
            messages.push(msg);
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
    fn message(&self, hydrate: Message, nodes: &mut Nodes) -> Result<Ident<message::Key>, Error> {
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
        let (extension_blocks, extensions) = self.extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let messages = self.messages(
            descriptor.nested_type,
            location.messages,
            fqn.clone(),
            key.into(),
            file,
            package,
            nodes,
        )?;

        let enums = self.enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        );

        let oneofs = self.oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let fields = self.fields(
            descriptor.field,
            location.fields,
            key.into(),
            fqn,
            file,
            package,
            nodes,
        )?;

        let location = location.detail;
        Ok(self.messages.lock()[key].hydrate(message::Hydrate {
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
        }))
    }

    fn enums(
        &self,
        descriptors: Vec<EnumDescriptorProto>,
        locations: Vec<location::Enum>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut Nodes,
    ) -> Vec<Ident<r#enum::Key>> {
        assert_enum_locations(&container_fqn, &locations, &descriptors);
        let mut enums = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let r#enum = self.r#enum(
                descriptor,
                fqn.clone(),
                location,
                container,
                file,
                package,
                nodes,
            );
            nodes.insert(r#enum.fqn.clone(), r#enum.key.into());
            enums.push(r#enum);
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
        nodes: &mut Nodes,
    ) -> Ident<r#enum::Key> {
        let name = descriptor.name.clone().unwrap_or_default().into_boxed_str();
        let key = self.enums.lock().get_or_insert_key(fqn.clone());
        let values = self.enum_values(
            descriptor.value,
            location.values,
            key,
            fqn,
            file,
            package,
            nodes,
        );
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
        nodes: &mut Nodes,
    ) -> Vec<Ident<enum_value::Key>> {
        assert_enum_value_locations(&enum_fqn, &locations, &descriptors);
        let mut values = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(enum_fqn.clone()));

            let enum_value = self.enum_value(EnumValue {
                descriptor,
                fqn: fqn.clone(),
                location,
                r#enum,
                file,
                package,
            });
            nodes.insert(enum_value.fqn(), enum_value.node_key());
            values.push(enum_value);
        }
        values
    }

    fn enum_value(&self, hydrate: EnumValue) -> Ident<enum_value::Key> {
        let EnumValue {
            descriptor,
            fqn,
            location,
            r#enum,
            file,
            package,
        } = hydrate;
        let mut enum_values = self.enum_values.lock();
        let key = enum_values.get_or_insert_key(fqn.clone());
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
        nodes: &mut Nodes,
    ) -> Result<Vec<Ident<service::Key>>, Error> {
        assert_service_locations(&container_fqn, &locations, &descriptors);
        let mut services = Vec::with_capacity(descriptors.len());
        for (descriptor, location) in descriptors.into_iter().zip(locations) {
            let fqn = FullyQualifiedName::new(descriptor.name(), Some(container_fqn.clone()));
            let ident = self.service(
                Service {
                    descriptor,
                    fqn,
                    location,
                    file,
                    package,
                },
                nodes,
            )?;
            nodes.insert(ident.fqn(), ident.node_key());
            services.push(ident);
        }
        Ok(services)
    }

    fn service(
        &self,
        service: Service,
        nodes: &mut HashMap<FullyQualifiedName, node::Key>,
    ) -> Result<Ident<service::Key>, Error> {
        let Service {
            file,
            package,
            descriptor,
            fqn,
            location,
        } = service;

        let key = self.service_key(fqn.clone());
        let methods = self.methods(
            descriptor.method,
            location.methods,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;
        self.services.lock()[key].hydrate(service::Hydrate {
            methods,
            location: location.detail,
            special_fields: descriptor.special_fields,
            options: descriptor.options.unwrap_or_default(),
            file,
            package,
        })
    }

    fn methods(
        &self,
        descriptors: Vec<MethodDescriptorProto>,
        locations: Vec<location::Method>,
        service: service::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut Nodes,
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
        nodes: &mut Nodes,
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
        nodes: &mut Nodes,
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
        nodes: &mut Nodes,
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

fn create_workers(descriptors_len: usize) -> (Vec<Worker<Job>>, Vec<Stealer<Job>>) {
    let worker_count = (descriptors_len.max(num_cpus::get()) - 1).min(1);
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast;
    #[test]
    fn test_hydration() {
        let commented = ast::test::commented_cgr();
        let files = commented.proto_file;
        super::run(files, &commented.file_to_generate);
    }
}
