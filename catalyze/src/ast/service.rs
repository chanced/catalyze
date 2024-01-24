use protobuf::{descriptor::ServiceOptions, SpecialFields};

use crate::error::Error;

use super::{
    access::NodeKeys,
    file, impl_traits_and_methods, location, method,
    node::{self, Ident},
    package, resolve,
    uninterpreted::UninterpretedOption,
    FullyQualifiedName, Set,
};

slotmap::new_key_type! {
    pub(super) struct Key;
}

pub(super) struct Hydrate {
    pub(super) location: location::Detail,
    pub(super) methods: Vec<Ident<method::Key>>,
    pub(super) special_fields: SpecialFields,
    pub(super) file: file::Key,
    pub(super) package: Option<package::Key>,
    pub(super) options: ServiceOptions,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Inner {
    key: Key,
    fqn: FullyQualifiedName,
    name: Box<str>,
    node_path: Box<[i32]>,
    span: location::Span,
    file: file::Key,
    package: Option<package::Key>,
    comments: Option<location::Comments>,

    methods: Set<super::method::Key>,

    deprecated: bool,

    uninterpreted_options: Vec<UninterpretedOption>,
    special_fields: SpecialFields,
    options_special_fields: SpecialFields,
}

impl Inner {
    #[allow(clippy::unnecessary_wraps)]
    pub(super) fn hydrate(&mut self, hydrate: Hydrate) -> Result<Ident<Key>, Error> {
        let Hydrate {
            location,
            methods,
            special_fields,
            file,
            package,
            options,
        } = hydrate;
        self.methods = methods.into();
        self.file = file;
        self.package = package;
        self.special_fields = special_fields;
        self.hydrate_location(location);
        self.hydrate_options(options);
        Ok(self.into())
    }
    fn hydrate_options(&mut self, opts: ServiceOptions) {
        self.deprecated = opts.deprecated.unwrap_or(false);
        self.uninterpreted_options = opts
            .uninterpreted_option
            .into_iter()
            .map(Into::into)
            .collect();
        self.options_special_fields = opts.special_fields;
    }
}

impl NodeKeys for Inner {
    fn keys(&self) -> impl Iterator<Item = node::Key> {
        self.methods.iter().copied().map(Into::into)
    }
}
pub struct Service<'ast>(resolve::Resolver<'ast, Key, Inner>);

impl_traits_and_methods!(Service, Key, Inner);
