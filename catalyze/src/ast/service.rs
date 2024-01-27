use protobuf::{descriptor::ServiceOptions, SpecialFields};

use crate::error::HydrationFailed;

use super::{
    access::NodeKeys,
    collection::Collection,
    file, impl_traits_and_methods, location, method,
    node::{self},
    package, resolve,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Name,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) type Ident = node::Ident<Key>;
pub(super) type Table = super::table::Table<Key, Inner>;

pub(super) struct Hydrate {
    pub(super) name: Name,
    pub(super) location: location::Detail,
    pub(super) methods: Vec<method::Ident>,
    pub(super) special_fields: SpecialFields,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) options: protobuf::MessageField<ServiceOptions>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Name,
    node_path: Box<[i32]>,
    span: location::Span,
    file: file::Key,
    package: Option<package::Key>,
    comments: Option<location::Comments>,

    methods: Collection<super::method::Key>,

    deprecated: bool,

    uninterpreted_options: Vec<UninterpretedOption>,
    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}

impl Inner {
    #[allow(clippy::unnecessary_wraps)]
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident, HydrationFailed> {
        let Hydrate {
            name,
            location,
            methods,
            special_fields,
            file,
            package,
            options,
        } = hydrate;
        self.name = name;
        self.methods = methods.into();
        self.file = file;
        self.package = package;
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.hydrate_options(options.unwrap_or_default())?;
        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: ServiceOptions) -> Result<(), HydrationFailed> {
        self.deprecated = opts.deprecated.unwrap_or(false);
        self.uninterpreted_options = opts
            .uninterpreted_option
            .into_iter()
            .map(Into::into)
            .collect();
        self.options_special_fields = opts.special_fields;
        Ok(())
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        self.methods.iter().copied().map(Into::into)
    }
}
pub struct Service<'ast>(resolve::Resolver<'ast, Key, Inner>);

impl_traits_and_methods!(Service, Key, Inner);
