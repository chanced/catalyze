use core::fmt;
use std::{
    hash::{Hash, Hasher},
    iter::Peekable,
};

use protobuf::descriptor::{source_code_info::Location as ProtoLoc, SourceCodeInfo};

use crate::error::{self, HydrationFailed};

use super::path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    File,
    Message,
    Enum,
    Service,
    Field,
    Extension,
    ExtensionDecl,
    EnumValue,
    Method,
    Oneof,
}
impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Zero-based spans of a node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start_line: i32,
    pub start_column: i32,
    pub end_line: i32,
    pub end_column: i32,
}
impl Span {
    fn new(span: &[i32]) -> Result<Self, error::InvalidSpan> {
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
            _ => Err(error::InvalidSpan {
                span: span.to_vec(),
                backtrace: snafu::Backtrace::capture(),
            }),
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
    pub leading: Option<String>,
    /// Any comment immediately following the entity, without any
    /// whitespace between it and the comment. If the comment would be a leading
    /// comment for another entity, it won't be considered a trailing comment.
    pub trailing: Option<String>,
    /// Each comment block or line above the entity but seperated by whitespace.
    pub leading_detached: Vec<String>,
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
        let leading_detached = leading_detacted.into_iter().collect();
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
    pub fn leading_detached(&self) -> &[String] {
        &self.leading_detached
    }
}
// TODO: newtype this
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
        let next_path = next.path.get(prefix.len()).copied().map(Into::into)?;
        Some((next, next_path))
    })
}

#[derive(Debug)]
pub(super) struct Location {
    pub(super) path: Box<[i32]>,
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
}
impl Location {
    pub(super) fn new(loc: ProtoLoc) -> Result<Self, HydrationFailed> {
        let span = Span::new(&loc.span)?;
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
    pub(super) syntax: Option<Location>,
    pub(super) package: Option<Location>,
    pub(super) dependencies: Vec<Location>,
    pub(super) messages: Vec<MessageLocation>,
    pub(super) enums: Vec<EnumLocation>,
    pub(super) services: Vec<ServiceLocation>,
    pub(super) extensions: Vec<ExtensionDeclLocation>,
    pub(super) node_count: usize,
}

impl File {
    pub(super) fn new(info: SourceCodeInfo) -> Result<Self, HydrationFailed> {
        let mut locations = info.location.into_iter().peekable();
        let mut package = None;
        let mut syntax = None;
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut services = Vec::new();
        let mut dependencies = Vec::new();
        let mut extensions = Vec::new();
        let mut node_count = 0;
        while let Some(loc) = locations.next() {
            if loc.path.is_empty() {
                continue;
            }
            match path::File::from_i32(loc.path[0]) {
                path::File::Syntax => {
                    syntax = Some(Location::new(loc)?);
                }
                path::File::Dependency => {
                    dependencies.push(Location::new(loc)?);
                }
                path::File::Package => {
                    package = Some(Location::new(loc)?);
                }
                path::File::Message => {
                    let message = MessageLocation::new(loc, &mut locations)?;
                    node_count += 1 + message.node_count;
                    messages.push(message);
                }
                path::File::Enum => {
                    let enumeration = EnumLocation::new(loc, &mut locations)?;
                    node_count += 1 + enumeration.values.len();
                    enums.push(enumeration);
                }
                path::File::Service => {
                    let service = ServiceLocation::new(loc, &mut locations)?;
                    node_count += 1 + service.methods.len();
                    services.push(service);
                }
                path::File::Extension => {
                    extensions.push(ExtensionDeclLocation::new(loc, &mut locations)?)
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
            node_count,
        })
    }
}

#[derive(Debug)]
pub(super) struct MessageLocation {
    pub(super) detail: Location,
    pub(super) messages: Vec<MessageLocation>,
    pub(super) enums: Vec<EnumLocation>,
    pub(super) extensions: Vec<ExtensionDeclLocation>,
    pub(super) oneofs: Vec<OneofLocation>,
    pub(super) fields: Vec<FieldLocation>,
    pub(super) node_count: usize,
}

impl MessageLocation {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
        let mut node_count = 0;
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut extensions = Vec::new();
        let mut oneofs = Vec::new();
        let mut fields = Vec::new();
        while let Some((loc, path)) = iterate_next(&detail.path, locations) {
            match path {
                path::Message::Field => {
                    node_count += 1;
                    fields.push(FieldLocation::new(loc, locations)?);
                }
                path::Message::Nested => {
                    let message = Self::new(loc, locations)?;
                    node_count += 1 + message.node_count;
                    messages.push(message);
                }
                path::Message::Enum => {
                    let enumeration = EnumLocation::new(loc, locations)?;
                    node_count += 1 + enumeration.values.len();
                    enums.push(enumeration);
                }
                path::Message::Extension => {
                    let ext_decl = ExtensionDeclLocation::new(loc, locations)?;
                    node_count += ext_decl.extensions.len();
                    extensions.push(ext_decl);
                }
                path::Message::Oneof => {
                    node_count += 1;
                    oneofs.push(OneofLocation::new(loc, locations)?);
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
            node_count,
        })
    }
}

#[derive(Debug)]
pub(super) struct ExtensionDeclLocation {
    pub(super) detail: Location,
    pub(super) extensions: Vec<FieldLocation>,
}
impl ExtensionDeclLocation {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, HydrationFailed> {
        let mut extensions = Vec::new();
        let detail = Location::new(node)?;
        while let Some((next, _)) = iterate_next::<i32>(&detail.path, locations) {
            extensions.push(FieldLocation::new(next, locations)?);
        }
        Ok(Self { detail, extensions })
    }
}

impl Hash for ExtensionDeclLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.detail.path.hash(state);
    }
}
pub(super) type ExtensionLocation = FieldLocation;

#[derive(Debug)]
pub(super) struct FieldLocation {
    pub(super) detail: Location,
}
impl Hash for FieldLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.detail.path.hash(state);
    }
}

impl FieldLocation {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
    }
}

#[derive(Debug)]
pub(super) struct OneofLocation {
    pub(super) detail: Location,
}
impl OneofLocation {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
    }
}

#[derive(Debug)]
pub(super) struct EnumLocation {
    pub(super) detail: Location,
    pub(super) values: Vec<EnumValueLocation>,
}
impl EnumLocation {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
        let mut values = Vec::new();
        while let Some((next, next_path)) = iterate_next(&detail.path, locations) {
            match next_path {
                path::Enum::Value => {
                    values.push(EnumValueLocation::new(next, locations)?);
                }
                path::Enum::Unknown(_) => continue,
            }
        }
        Ok(Self { detail, values })
    }
}
#[derive(Debug)]
pub(super) struct EnumValueLocation {
    pub(super) detail: Location,
}
impl EnumValueLocation {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
        while iterate_next::<i32>(&detail.path, locations).is_some() {}
        Ok(Self { detail })
    }
}
#[derive(Debug)]
pub(super) struct ServiceLocation {
    pub(super) detail: Location,
    pub(super) methods: Vec<MethodLocation>,
}
impl ServiceLocation {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
        let mut methods = Vec::new();
        while let Some((next, next_path)) = iterate_next(&detail.path, locations) {
            match next_path {
                path::Service::Method => {
                    methods.push(MethodLocation::new(next, locations)?);
                }
                path::Service::Mixin | path::Service::Unknown(_) => continue,
            }
        }
        Ok(Self { detail, methods })
    }
}

#[derive(Debug)]
pub(super) struct MethodLocation {
    pub(super) detail: Location,
}
impl MethodLocation {
    fn new(
        node: ProtoLoc,
        locations: &mut Peekable<std::vec::IntoIter<ProtoLoc>>,
    ) -> Result<Self, HydrationFailed> {
        let detail = Location::new(node)?;
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
        *(cgr.proto_file.pop().unwrap().source_code_info.0.unwrap())
    }
}
