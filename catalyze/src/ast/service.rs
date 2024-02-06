use protobuf::{descriptor::ServiceOptions as ProtoServiceOpts, SpecialFields};

use crate::error::HydrationFailed;

use super::{
    access::AccessNodeKeys,
    collection::Collection,
    file::FileKey,
    impl_traits_and_methods,
    location::{Comments, Location, Span},
    method::{MethodIdent, MethodKey},
    node,
    package::PackageKey,
    reference::ReferenceInner,
    resolve::Resolver,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct ServiceKey;
}

pub(super) type Ident = node::Ident<ServiceKey>;
pub(super) type Table = super::table::Table<ServiceKey, ServiceInner>;

pub struct Service<'ast>(pub(super) Resolver<'ast, ServiceKey, ServiceInner>);
impl_traits_and_methods!(Service, ServiceKey, ServiceInner);

impl<'ast> Service<'ast> {
    pub fn name(&self) -> &str {
        &self.0.name
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct ServiceInner {
    pub(super) key: ServiceKey,
    pub(super) fqn: FullyQualifiedName,
    pub(super) name: Name,
    pub(super) node_path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) comments: Option<Comments>,
    pub(super) methods: Collection<MethodKey>,
    pub(super) references: Vec<ReferenceInner>,
    pub(super) special_fields: SpecialFields,
    pub(super) options: ServiceOptions,
    pub(super) proto_opts: ProtoServiceOpts,
}

impl ServiceInner {
    #[allow(clippy::unnecessary_wraps)]
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            location,
            methods,
            references,
            special_fields,
            file,
            package,
            mut options,
        } = hydrate;
        self.name = name;
        self.methods = methods.into();
        self.file = file;
        self.package = package;
        self.references = references;
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.options.hydrate(&mut options);
        self.proto_opts = options;
        Ok(self.into())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServiceOptions {
    pub uninterpreted_options: Vec<UninterpretedOption>,
    pub deprecated: bool,
}

impl ServiceOptions {
    fn hydrate(&mut self, proto_opts: &mut ProtoServiceOpts) {
        self.deprecated = proto_opts.deprecated.unwrap_or(false);
        let uninterpreted_options = std::mem::take(&mut proto_opts.uninterpreted_option);
        self.uninterpreted_options = uninterpreted_options.into_iter().map(Into::into).collect();
    }
}

impl AccessNodeKeys for ServiceInner {
    fn keys(&self) -> impl Iterator<Item = node::NodeKey> {
        self.methods.iter().copied().map(Into::into)
    }
}

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) location: Location,
    pub(super) methods: Vec<MethodIdent>,
    pub(super) special_fields: SpecialFields,
    pub(super) file: FileKey,
    pub(super) package: Option<PackageKey>,
    pub(super) references: Vec<ReferenceInner>,
    pub(super) options: ProtoServiceOpts,
}
