use ahash::HashMapExt;
use crossbeam::{
    channel,
    deque::{Injector, Stealer, Worker},
    sync::WaitGroup,
};
use itertools::Itertools;
use std::{self, ops::ControlFlow, path::PathBuf, thread, time::Duration};

use crate::{error::Error, to_i32, HashMap};
use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto,
    FileDescriptorProto, MethodDescriptorProto, OneofDescriptorProto, ServiceDescriptorProto,
};

use super::{
    container, r#enum, enum_value, extension, extension_decl, field, file, location, message,
    method,
    node::{self, Ident},
    oneof, package, relationship, service, Ast, FullyQualifiedName, Name,
};

type Nodes = HashMap<FullyQualifiedName, node::Key>;
type Receiver = channel::Receiver<Result<Completed, Error>>;

enum Job {
    Initialize(Initialize),
    Populate(Populate),
    Link(Link),
    Finalize(Finalize),
}
impl From<Initialize> for Job {
    fn from(v: Initialize) -> Self {
        Self::Initialize(v)
    }
}
impl From<Populate> for Job {
    fn from(v: Populate) -> Self {
        Self::Populate(v)
    }
}
impl From<Link> for Job {
    fn from(v: Link) -> Self {
        Self::Link(v)
    }
}
impl From<Finalize> for Job {
    fn from(v: Finalize) -> Self {
        Self::Finalize(v)
    }
}

enum Initialize {
    File(File),
    Message(Message),
    Enum(Enum),
    EnumValue(EnumValue),
    Service(Service),
    Method(Method),
    Field(Field),
    Oneof(Oneof),
    Extension(Extension),
    ExtensionDecl(ExtensionDecl),
}

enum Populated {
    Package(package::Inner),
    File(file::Inner),
    Message(message::Inner),
    Enum(r#enum::Inner),
    EnumValue(enum_value::Inner),
    Service(service::Inner),
    Method(method::Inner),
    Field(field::Inner),
    Oneof(oneof::Inner),
    Extension(extension::Inner),
    ExtensionDecl(extension_decl::Inner),
}

enum Populate {
    File(FileDescriptorProto),
    Package(FullyQualifiedName),
    Message(Message),
    Enum(Enum),
    EnumValue(EnumValue),
    Service(Service),
    Method(Method),
    Field(Field),
    Oneof(Oneof),
    Extension(Extension),
    ExtensionDecl(ExtensionDecl),
}

impl From<ExtensionDecl> for Populate {
    fn from(v: ExtensionDecl) -> Self {
        Self::ExtensionDecl(v)
    }
}

impl From<Extension> for Populate {
    fn from(v: Extension) -> Self {
        Self::Extension(v)
    }
}

impl From<Oneof> for Populate {
    fn from(v: Oneof) -> Self {
        Self::Oneof(v)
    }
}

impl From<Field> for Populate {
    fn from(v: Field) -> Self {
        Self::Field(v)
    }
}

impl From<Method> for Populate {
    fn from(v: Method) -> Self {
        Self::Method(v)
    }
}

impl From<Service> for Populate {
    fn from(v: Service) -> Self {
        Self::Service(v)
    }
}

impl From<EnumValue> for Populate {
    fn from(v: EnumValue) -> Self {
        Self::EnumValue(v)
    }
}

impl From<Enum> for Populate {
    fn from(v: Enum) -> Self {
        Self::Enum(v)
    }
}

impl From<Message> for Populate {
    fn from(v: Message) -> Self {
        Self::Message(v)
    }
}

impl From<FullyQualifiedName> for Populate {
    fn from(v: FullyQualifiedName) -> Self {
        Self::Package(v)
    }
}

impl From<FileDescriptorProto> for Populate {
    fn from(v: FileDescriptorProto) -> Self {
        Self::File(v)
    }
}

enum Completed {
    Populate(Populated),
    Link(Link),
    Finalize(Finalize),
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
    key: Key,
}

struct Finalize {
    key: file::Key,
    fqn: FullyQualifiedName,
    name: Name,
    path: PathBuf,
    nodes: HashMap<FullyQualifiedName, node::Key>,
}
enum Associate {
    Package(),
}

impl From<Link> for Job {
    fn from(v: Link) -> Self {
        Self::Link(v)
    }
}

impl From<FileDescriptorProto> for Job {
    fn from(v: FileDescriptorProto) -> Self {
        Self::Populate(v.into())
    }
}

pub(crate) fn run(
    file_descriptors: Vec<FileDescriptorProto>,
    targets: &[String],
) -> Result<super::Ast, Error> {
    // Hydrator::new(file_descriptors, targets).map(Into::into)
    todo!()
}

struct File {
    descriptor: FileDescriptorProto,
    location: location::File,
    package: Option<package::Key>,
}

struct Message {
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

struct Extension {
    descriptor: FieldDescriptorProto,
    fqn: FullyQualifiedName,
    container: container::Key,
    file: file::Key,
    package: Option<package::Key>,
}

struct ExtensionDecl {
    location: location::ExtensionDecl,
    extensions: Vec<Extension>,
}

struct Accumulator<'input> {
    ast: Ast,
    targets: &'input [String],
}

impl<'input> Accumulator<'input> {
    fn new(file_count: usize, targets: &'input [String]) -> Self {
        Self {
            ast: Ast::new(file_count),
            targets,
        }
    }
    fn receive(&self, receiver: &Receiver) -> Result<Completed, Error> {
        receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("hydration failed due to timeout.")
    }

    fn complete_populate(hydrated: Populated) -> Result<(), Error> {
        todo!()
    }

    fn complete_populate_file(file: file::Inner) -> Result<(), Error> {
        todo!()
    }

    fn complete_populate_package(package: package::Package) -> Result<(), Error> {
        todo!()
    }

    fn accumulate(&self, receiver: Receiver) -> Result<Ast, Error> {
        // let is_build_target = self.targets.iter().any(|t| t == name.as_ref());

        // TODO: configure timeout duration
        while let completed = self.receive(&receiver)? {
            match completed {
                Completed::Populate(hydrated) => todo!(),
                Completed::Link(_) => todo!(),
                Completed::Finalize(_) => todo!(),
            }
        }
        todo!()
    }
}

struct Hydrator<'input> {
    file_count: usize,
    injector: Injector<Job>,
    stealers: Vec<Stealer<Job>>,
    accumulator: Accumulator<'input>,
}

impl<'input> Hydrator<'input> {
    fn execute(
        file_descriptors: Vec<FileDescriptorProto>,
        targets: &'input [String],
    ) -> Result<Ast, Error> {
        let file_count = file_descriptors.len();
        let (workers, stealers) = create_workers(file_descriptors.len());
        let (sender, receiver) = channel::unbounded();
        let senders = Sender::new_vec(sender, workers.len());
        let injector = Injector::new();
        let accumulator = Accumulator::new(file_count, targets);
        let rally_points = RallyPoints::new_vec(workers.len());
        for descriptor in file_descriptors {
            injector.push(descriptor.into());
        }

        Self {
            accumulator,
            injector,
            stealers,
            file_count,
        }
        .spawn(workers, receiver, senders, rally_points)
    }

    fn spawn(
        &self,
        workers: Vec<Worker<Job>>,
        receiver: Receiver,
        senders: Vec<Sender>,
        rally_points: Vec<RallyPoints>,
    ) -> Result<Ast, Error> {
        let mut worker_handles = Vec::with_capacity(workers.len());
        let mut iter = workers
            .into_iter()
            .zip(senders.into_iter())
            .zip(rally_points.into_iter())
            .map(|((w, s), g)| (w, s, g));

        thread::scope(|scope| {
            let ast = scope.spawn(move || self.accumulator.accumulate(receiver));
            for (worker, sender, wg) in iter {
                worker_handles.push(scope.spawn(move || self.work(worker, sender, wg)));
            }
            for handle in worker_handles {
                handle.join().unwrap();
            }
            ast.join().unwrap()
        })
    }

    fn link(&self, link: Link) -> Result<Key, Error> {
        todo!()
    }

    fn finalize(&self, finalize: Finalize) -> Result<Completed, Error> {
        todo!()
    }

    fn work(&self, local: Worker<Job>, results: Sender, mut rally: RallyPoints) {
        while let Some(job) = self.find_work(&local) {
            if match job {
                Job::Populate(job) => results.send(self.populate(job).map(Completed::Populate)),
                Job::Link(job) => {
                    rally.at_link();
                    todo!()
                    // results.send(self.link(job).map(Completed::Li))
                }
                Job::Finalize(job) => {
                    rally.at_finalize();
                    todo!()
                    // results.send(self.finalize(job).map(Completed::Finalize))
                }
            }
            .is_break()
            {
                break;
            }
        }
    }
    fn populate(&self, hydrate: Populate) -> Result<Populated, Error> {
        // match hydrate {
        //     Hydrate::File(file) => self.hydrate_file(file),
        //     Hydrate::Package(fqn) => self.hydrate_package(fqn),
        //     Hydrate::Message(message) => self.hydrate_message(message),
        //     Hydrate::Enum(r#enum) => self.hydrate_enum(r#enum),
        //     Hydrate::EnumValue(enum_value) => self.enum_value(enum_value),
        //     Hydrate::Service(service) => self.hydrate_service(service),
        //     Hydrate::Method(method) => self.hydrate_method(method),
        //     Hydrate::Field(field) => self.hydrate_field(field),
        //     Hydrate::Oneof(oneof) => self.hydrate_oneof(oneof),
        //     Hydrate::Extension(extension) => self.extension(extension),
        //     Hydrate::ExtensionDecl(extension_decl) =>
        // self.extension_decl(extension_decl), }
        todo!()
    }

    fn link_dependencies(&self) {
        todo!()
    }

    fn completed(&self, acc: &mut Ast, completed: Completed) -> Result<(), Error> {
        match completed {
            Completed::Populate(hydrate) => todo!(),
            Completed::Link(link) => {

                // self.injector.push(Job::Finalize(self.link(link)?));
            }
            Completed::Finalize(finalize) => {
                acc.files_by_name.insert(finalize.name, finalize.key);
                acc.files_by_path.insert(finalize.path, finalize.key);
                acc.nodes.extend(finalize.nodes);
            }
        }
        Ok(())
    }

    fn find_work(&self, local: &Worker<Job>) -> Option<Job> {
        local
            .pop()
            .or_else(|| self.injector.steal().success())
            .or_else(|| self.stealers.iter().find_map(|s| s.steal().success()))
    }

    fn populate_package(&self, package: Option<String>) -> Option<package::Inner> {
        todo!()
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
        let fqn = FullyQualifiedName::for_package(name);
        self.injector.push(Populate::Package(fqn.clone()).into());
        (Some(package::Key::default()), Some(fqn))
    }

    fn populate_file(&self, descriptor: FileDescriptorProto) -> Result<Link, Error> {
        let name = descriptor.name.unwrap_or_default().into();
        let locations = location::File::new(descriptor.source_code_info.unwrap_or_else(|| {
            panic!("source_code_info not found on FileDescriptorProto for \"{name}\"")
        }))?;
        let mut nodes = HashMap::with_capacity(locations.node_count);

        let (package, package_fqn) = self.package(descriptor.package);
        let fqn = FullyQualifiedName::new(&name, package_fqn);

        let messages = self.messages(
            descriptor.message_type,
            locations.messages,
            fqn.clone(),
            key.into(),
            key,
            package,
            &mut nodes,
        )?;

        let enums = self.populate_enums(
            descriptor.enum_type,
            locations.enums,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        );

        let services = self.populate_services(
            descriptor.service,
            locations.services,
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let (extension_blocks, extensions) = self.populate_extensions(
            descriptor.extension,
            locations.extensions,
            key.into(),
            fqn.clone(),
            key,
            package,
            &mut nodes,
        )?;

        let dependencies = self.populate_dependencies(
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

        Ok(Link { nodes })
    }

    fn messages(
        &self,
        descriptors: Vec<DescriptorProto>,
        locations: Vec<location::Message>,
        container: container::Key,
        container_fqn: FullyQualifiedName,
        file: file::Key,
        package: Option<package::Key>,
        nodes: &mut Nodes,
    ) -> Result<Vec<message::Ident>, Error> {
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
            let msg = self.populate_message(hydrate, nodes)?;
            nodes.insert(msg.fqn(), msg.node_key());
            messages.push(msg);
        }
        Ok(messages)
    }

    fn populate_dependencies(
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

    #[allow(clippy::too_many_arguments)]
    fn populate_message(
        &self,
        hydrate: Message,
        nodes: &mut Nodes,
    ) -> Result<Ident<message::Key>, Error> {
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
        let (extension_blocks, extensions) = self.populate_extensions(
            descriptor.extension,
            location.extensions,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let messages = self.populate_messages(
            descriptor.nested_type,
            location.messages,
            fqn.clone(),
            key.into(),
            file,
            package,
            nodes,
        )?;

        let enums = self.populate_enums(
            descriptor.enum_type,
            location.enums,
            key.into(),
            fqn.clone(),
            file,
            package,
            nodes,
        );

        let oneofs = self.populate_oneofs(
            descriptor.oneof_decl,
            location.oneofs,
            key,
            fqn.clone(),
            file,
            package,
            nodes,
        )?;

        let fields = self.populate_fields(
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

    fn populate_enums(
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
            let r#enum = self.populate_enum(
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

    fn populate_enum(
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
        let values = self.populate_enum_values(
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

    fn populate_enum_values(
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
        let mut enum_values = self.populate_enum_values.lock();
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

    fn populate_services(
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
            let ident = self.populate_service(
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

    fn populate_service(
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
        let methods = self.populate_methods(
            descriptor.method,
            location.methods,
            key,
            fqn,
            file,
            package,
            nodes,
        )?;
        self.populate_services.lock()[key].hydrate(service::Hydrate {
            methods,
            location: location.detail,
            special_fields: descriptor.special_fields,
            options: descriptor.options.unwrap_or_default(),
            file,
            package,
        })
    }

    fn populate_methods(
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

    fn populate_method(&self) -> Result<Ident<method::Key>, Error> {
        todo!()
    }

    fn populate_fields(
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

    fn populate_field(&self) -> Result<Ident<field::Key>, Error> {
        todo!()
    }

    fn populate_oneofs(
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

    fn populate_oneof(&self) -> Result<Ident<oneof::Key>, Error> {
        todo!()
    }

    fn populate_extensions(
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

#[derive(Clone)]
struct Sender(channel::Sender<Result<Completed, Error>>);
impl Sender {
    fn new_vec(sender: channel::Sender<Result<Completed, Error>>, len: usize) -> Vec<Self> {
        let mut senders = Vec::with_capacity(len);
        for _ in 0..(len - 1) {
            senders.push(Self(sender.clone()));
        }
        senders.push(Self(sender));
        senders
    }
    fn send(&self, result: Result<Completed, Error>) -> ControlFlow<()> {
        match self.0.send(result) {
            Ok(()) => ControlFlow::Continue(()),
            Err(_) => ControlFlow::Break(()),
        }
    }
}

#[derive(Clone)]
struct RallyPoint(Option<WaitGroup>);
impl Default for RallyPoint {
    fn default() -> Self {
        Self(Some(WaitGroup::new()))
    }
}
impl RallyPoint {
    fn rally(&mut self) {
        if let Some(wg) = self.0.take() {
            WaitGroup::wait(wg);
        }
    }
}

#[derive(Default, Clone)]
struct RallyPoints {
    populate: RallyPoint,
    link: RallyPoint,
    finalize: RallyPoint,
}

impl RallyPoints {
    fn new_vec(len: usize) -> Vec<Self> {
        let mut rps = Vec::with_capacity(len);
        let rp = Self::default();
        for _ in 0..(len - 1) {
            rps.push(rp.clone());
        }
        rps.push(rp);
        rps
    }

    fn at_populate(&mut self) {
        self.populate.rally();
    }
    fn at_link(&mut self) {
        self.at_populate();
        self.link.rally();
    }
    fn at_finalize(&mut self) {
        self.at_link();
        self.finalize.rally();
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
        .map(|_| Worker::<Job>::new_fifo())
        .collect_vec();
    let stealers = workers.iter().map(Worker::stealer).collect_vec();
    (workers, stealers)
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
