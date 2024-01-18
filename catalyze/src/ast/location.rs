use std::iter::Peekable;

use protobuf::descriptor::{source_code_info::Location as ProtoLoc, SourceCodeInfo};

use crate::error::Error;

use super::{path, Comments, Span};

type Iter = Peekable<std::vec::IntoIter<ProtoLoc>>;

fn iterate_next<T>(prefix: &[i32], locations: &mut Iter) -> Option<(ProtoLoc, T)>
where
    T: From<i32>,
{
    let peeked = locations.peek()?;
    let subpath = peeked.path.get(..prefix.len())?;
    // len check is required because of how extension groups are pathed
    if subpath != &prefix[..subpath.len()] || peeked.path.len() == prefix.len() {
        return None;
    }
    locations.next().and_then(|next| {
        let next_path = next.path.get(prefix.len()).map(|&n| T::from(n))?;
        Some((next, next_path))
    })
}
fn extract(loc: ProtoLoc) -> Result<(Box<[i32]>, Span, Option<Comments>), Error> {
    let span = Span::new(&loc.span).map_err(|()| Error::invalid_span(&loc))?;
    let comments = Comments::new_maybe(
        loc.leading_comments,
        loc.trailing_comments,
        loc.leading_detached_comments,
    );
    Ok((loc.path.into(), span, comments))
}

#[derive(Debug)]
pub(super) struct Location {
    pub(super) path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
}
impl Location {
    pub(super) fn new(path: Box<[i32]>, span: Span, comments: Option<Comments>) -> Self {
        Self {
            path,
            span,
            comments,
        }
    }
}

#[derive(Debug)]
pub(super) struct File {
    pub(super) syntax: Option<Location>,
    pub(super) package: Option<Location>,
    pub(super) dependencies: Vec<Location>,
    pub(super) messages: Vec<Message>,
    pub(super) enums: Vec<Enum>,
    pub(super) services: Vec<Service>,
    pub(super) extensions: Vec<ExtensionBlock>,
}
impl File {
    pub(super) fn new(info: SourceCodeInfo) -> Result<Self, Error> {
        let mut locations = info.location.into_iter().peekable();
        let mut package = None;
        let mut syntax = None;
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut services = Vec::new();
        let mut dependencies = Vec::new();
        let mut extensions = Vec::new();

        let (path, span, comments) = extract(locations.next().unwrap())?;

        while let Some(next) = locations.next() {
            match path::File::from_i32(next.path[0]) {
                path::File::Syntax => {
                    let (path, span, comments) = extract(next)?;
                    syntax = Some(Location {
                        path,
                        span,
                        comments,
                    });
                }
                path::File::Dependency => {
                    let (path, span, comments) = extract(next)?;
                    dependencies.push(Location {
                        path,
                        span,
                        comments,
                    });
                }
                path::File::Package => {
                    let (path, span, comments) = extract(next)?;
                    package = Some(Location {
                        path,
                        span,
                        comments,
                    });
                }
                path::File::Message => {
                    messages.push(Message::new(next, &mut locations)?);
                }
                path::File::Enum => {
                    enums.push(Enum::new(next, &mut locations)?);
                }
                path::File::Service => {
                    services.push(Service::new(next, &mut locations)?);
                }
                path::File::Extension => {
                    extensions.push(ExtensionBlock::new(next, &mut locations)?);
                }
                _ => continue,
            }
        }
        Ok(Self {
            syntax,
            package,
            dependencies,
            messages,
            enums,
            services,
            extensions,
        })
    }
}

#[derive(Debug)]
pub(super) struct Message {
    pub(super) detail: Location,
    pub(super) messages: Vec<Message>,
    pub(super) enums: Vec<Enum>,
    pub(super) extensions: Vec<ExtensionBlock>,
    pub(super) oneofs: Vec<Oneof>,
    pub(super) fields: Vec<Field>,
}

impl Message {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;

        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut extensions = Vec::new();
        let mut oneofs = Vec::new();
        let mut fields = Vec::new();

        while let Some((next, next_path)) = iterate_next(&path, locations) {
            match next_path {
                path::Message::Field => {
                    fields.push(Field::new(next, locations)?);
                }
                path::Message::Nested => {
                    messages.push(Self::new(next, locations)?);
                }
                path::Message::Enum => {
                    enums.push(Enum::new(next, locations)?);
                }
                path::Message::Extension => {
                    extensions.push(ExtensionBlock::new(next, locations)?);
                }
                path::Message::Oneof => {
                    oneofs.push(Oneof::new(next, locations)?);
                }
                path::Message::Unknown(_) => continue,
            }
        }
        Ok(Self {
            detail: Location::new(path, span, comments),
            messages,
            fields,
            enums,
            extensions,
            oneofs,
        })
    }
}

#[derive(Debug)]
pub(super) struct ExtensionBlock {
    pub(super) detail: Location,
    pub(super) extensions: Vec<Field>,
}
impl ExtensionBlock {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let mut extensions = Vec::new();
        let (mut path, span, comments) = extract(node)?;
        while let Some((next, _)) = iterate_next::<i32>(&path, locations) {
            extensions.push(Field::new(next, locations)?);
        }
        Ok(Self {
            detail: Location::new(path, span, comments),
            extensions,
        })
    }
}

#[derive(Debug)]
pub(super) struct Field {
    pub(super) detail: Location,
}
impl Field {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;
        while iterate_next::<i32>(&path, locations).is_some() {}
        Ok(Self {
            detail: Location {
                path,
                span,
                comments,
            },
        })
    }
}

#[derive(Debug)]
pub(super) struct Oneof {
    pub(super) location: Location,
}
impl Oneof {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;
        while iterate_next::<i32>(&path, locations).is_some() {}
        Ok(Self {
            location: Location::new(path, span, comments),
        })
    }
}

#[derive(Debug)]
pub(super) struct Enum {
    pub(super) detail: Location,
    pub(super) values: Vec<EnumValue>,
}
impl Enum {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;
        let mut values = Vec::new();
        while let Some((next, next_path)) = iterate_next(&path, locations) {
            match next_path {
                path::Enum::Value => {
                    values.push(EnumValue::new(next, locations)?);
                }
                path::Enum::Unknown(_) => continue,
            }
        }
        Ok(Self {
            detail: Location::new(path, span, comments),
            values,
        })
    }
}
#[derive(Debug)]
pub(super) struct EnumValue {
    pub(super) location: Location,
}
impl EnumValue {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;
        while iterate_next::<i32>(&path, locations).is_some() {}
        Ok(Self {
            location: Location::new(path, span, comments),
        })
    }
}
#[derive(Debug)]
pub(super) struct Service {
    pub(super) detail: Location,
    pub(super) methods: Vec<Method>,
}
impl Service {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;
        let mut methods = Vec::new();
        while let Some((next, next_path)) = iterate_next(&path, locations) {
            match next_path {
                path::Service::Method => {
                    methods.push(Method::new(next, locations)?);
                }
                path::Service::Mixin | path::Service::Unknown(_) => continue,
            }
        }
        Ok(Self {
            detail: Location::new(path, span, comments),
            methods,
        })
    }
}

#[derive(Debug)]
pub(super) struct Method {
    pub(super) detail: Location,
}
impl Method {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let (path, span, comments) = extract(node)?;
        while iterate_next::<i32>(&path, locations).is_some() {}
        Ok(Self {
            detail: Location::new(path, span, comments),
        })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use protobuf::{descriptor::SourceCodeInfo, plugin::CodeGeneratorRequest, Message};

    use super::*;

    #[test]
    fn test_new() {
        let f = File::new(construct_info()).unwrap();
        assert_eq!(f.enums.len(), 2);
        assert_eq!(
            f.enums[0]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading
                .as_deref()
                .unwrap()
                .trim(),
            "Enum0 comments"
        );
        assert_eq!(
            f.enums[0].values[0]
                .location
                .comments
                .as_ref()
                .unwrap()
                .leading
                .as_deref()
                .unwrap()
                .trim(),
            "Enum0 Zero"
        );
        assert_eq!(
            f.enums[0].values[1]
                .location
                .comments
                .as_ref()
                .unwrap()
                .leading
                .as_deref()
                .unwrap()
                .trim(),
            "Enum0 One"
        );

        assert_eq!(
            f.enums[1]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Enum1 comments"
        );

        assert_eq!(f.messages.len(), 5);
        assert_eq!(
            f.messages[0]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Message0 comments"
        );
        assert_eq!(
            f.messages[3]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Message3 comments"
        );

        assert_eq!(
            f.messages[0].enums[0]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Message0EmbeddedEnum comments"
        );

        assert_eq!(
            f.messages[0].enums[0].values[1]
                .location
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "One - Messaeg0EmbeddedEnum"
        );

        assert_eq!(
            f.messages[3].fields[0]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Message3 field 0"
        );
        assert_eq!(
            f.messages[3].fields[1]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Message3 field 1"
        );
        assert_eq!(
            f.messages[3].oneofs[0]
                .location
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "oneof0 comments"
        );
        println!(
            "{:#?}",
            f.extensions
                .iter()
                .map(|e| e
                    .detail
                    .comments
                    .as_ref()
                    .unwrap()
                    .leading()
                    .unwrap()
                    .trim())
                .collect_vec()
        );
        assert_eq!(f.extensions.len(), 3);
        assert_eq!(
            f.extensions[0]
                .detail
                .comments
                .as_ref()
                .unwrap()
                .leading()
                .unwrap()
                .trim(),
            "Extend0 Message1"
        );
    }

    fn construct_info() -> SourceCodeInfo {
        //  protoc --plugin=protoc-gen-debug=target/release/protoc-gen-debug \
        // --debug_out=. --debug_opt=./fixtures/cgr/commented \
        // --proto_path=./fixtures/protos ./fixtures/protos/commented/commented.proto

        let bytes = include_bytes!("../../../fixtures/cgr/commented/code_generator_request.bin");
        let mut cgr = CodeGeneratorRequest::parse_from_bytes(bytes).unwrap();
        cgr.proto_file[0].source_code_info.take().unwrap()
    }
}
