use std::iter::Peekable;

use protobuf::descriptor::{source_code_info::Location as ProtoLoc, SourceCodeInfo};

use crate::error::Error;

use super::path;

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

#[derive(Debug)]
pub(super) struct Detail {
    pub(super) path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
}
impl Detail {
    pub(super) fn new(loc: ProtoLoc) -> Result<Self, Error> {
        let span = Span::new(&loc.span).map_err(|()| Error::invalid_span(&loc))?;
        let comments = Comments::new_maybe(
            loc.leading_comments,
            loc.trailing_comments,
            loc.leading_detached_comments,
        );
        let path = loc.path.into();
        Ok(Self {
            path,
            span,
            comments,
        })
    }
}

#[derive(Debug)]
pub(super) struct File {
    pub(super) syntax: Option<Detail>,
    pub(super) package: Option<Detail>,
    pub(super) dependencies: Vec<Detail>,
    pub(super) messages: Vec<Message>,
    pub(super) enums: Vec<Enum>,
    pub(super) services: Vec<Service>,
    pub(super) extensions: Vec<ExtensionDecl>,
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
        let Detail {
            path,
            span,
            comments,
        } = Detail::new(locations.next().unwrap())?;

        while let Some(loc) = locations.next() {
            match path::File::from_i32(loc.path[0]) {
                path::File::Syntax => {
                    syntax = Some(Detail::new(loc)?);
                }
                path::File::Dependency => {
                    dependencies.push(Detail::new(loc)?);
                }
                path::File::Package => {
                    package = Some(Detail::new(loc)?);
                }
                path::File::Message => {
                    messages.push(Message::new(loc, &mut locations)?);
                }
                path::File::Enum => {
                    enums.push(Enum::new(loc, &mut locations)?);
                }
                path::File::Service => {
                    services.push(Service::new(loc, &mut locations)?);
                }
                path::File::Extension => extensions.push(ExtensionDecl::new(loc, &mut locations)?),
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
    pub(super) detail: Detail,
    pub(super) messages: Vec<Message>,
    pub(super) enums: Vec<Enum>,
    pub(super) extensions: Vec<ExtensionDecl>,
    pub(super) oneofs: Vec<Oneof>,
    pub(super) fields: Vec<Field>,
}

impl Message {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let detail = Detail::new(node)?;

        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut extensions = Vec::new();
        let mut oneofs = Vec::new();
        let mut fields = Vec::new();
        while let Some((loc, path)) = iterate_next(&detail.path, locations) {
            match path {
                path::Message::Field => {
                    fields.push(Field::new(loc, locations)?);
                }
                path::Message::Nested => {
                    messages.push(Self::new(loc, locations)?);
                }
                path::Message::Enum => {
                    enums.push(Enum::new(loc, locations)?);
                }
                path::Message::Extension => {
                    extensions.push(ExtensionDecl::new(loc, locations)?);
                }
                path::Message::Oneof => {
                    oneofs.push(Oneof::new(loc, locations)?);
                }
                path::Message::Unknown(_) => continue,
            }
        }
        Ok(Self {
            detail,
            messages,
            enums,
            extensions,
            oneofs,
            fields,
        })
    }
}

#[derive(Debug)]
pub(super) struct ExtensionDecl {
    pub(super) detail: Detail,
    pub(super) extensions: Vec<Field>,
}
impl ExtensionDecl {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let mut extensions = Vec::new();
        let detail = Detail::new(node)?;
        while let Some((next, _)) = iterate_next::<i32>(&detail.path, locations) {
            extensions.push(Field::new(next, locations)?);
        }
        Ok(Self { detail, extensions })
    }
}

#[derive(Debug)]
pub(super) struct Field {
    pub(super) detail: Detail,
}
impl Field {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let detail = Detail::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
    }
}

#[derive(Debug)]
pub(super) struct Oneof {
    pub(super) detail: Detail,
}
impl Oneof {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let detail = Detail::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
    }
}

#[derive(Debug)]
pub(super) struct Enum {
    pub(super) detail: Detail,
    pub(super) values: Vec<EnumValue>,
}
impl Enum {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let detail = Detail::new(node)?;
        let mut values = Vec::new();
        while let Some((next, next_path)) = iterate_next(&detail.path, locations) {
            match next_path {
                path::Enum::Value => {
                    values.push(EnumValue::new(next, locations)?);
                }
                path::Enum::Unknown(_) => continue,
            }
        }
        Ok(Self { detail, values })
    }
}
#[derive(Debug)]
pub(super) struct EnumValue {
    pub(super) detail: Detail,
}
impl EnumValue {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let detail = Detail::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
    }
}
#[derive(Debug)]
pub(super) struct Service {
    pub(super) detail: Detail,
    pub(super) methods: Vec<Method>,
}
impl Service {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let detail = Detail::new(node)?;
        let mut methods = Vec::new();
        while let Some((next, next_path)) = iterate_next(&detail.path, locations) {
            match next_path {
                path::Service::Method => {
                    methods.push(Method::new(next, locations)?);
                }
                path::Service::Mixin | path::Service::Unknown(_) => continue,
            }
        }
        Ok(Self { detail, methods })
    }
}

#[derive(Debug)]
pub(super) struct Method {
    pub(super) detail: Detail,
}
impl Method {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, Error> {
        let detail = Detail::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
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
                .detail
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
                .detail
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
                .detail
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
                .detail
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
