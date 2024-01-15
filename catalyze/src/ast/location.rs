use std::{collections::BTreeMap, iter::Peekable};

use protobuf::descriptor::{source_code_info::Location as ProtoLoc, SourceCodeInfo};

use crate::error::Error;

use super::{path, Comments, Span};

type Iter = Peekable<std::vec::IntoIter<ProtoLoc>>;

pub(super) struct Location {
    pub(super) span: Span,
    pub(super) comments: Option<Comments>,
}

pub(super) struct File {
    pub(super) syntax: Option<Location>,
    pub(super) package: Option<Location>,
    pub(super) messages: Vec<Message>,
    pub(super) enums: Vec<Enum>,
    pub(super) services: Vec<Service>,
    pub(super) extensions: Vec<ExtensionGroup>,
}
impl File {
    pub(super) fn new(info: SourceCodeInfo) -> Result<Self, Error> {
        todo!()
    }
}

fn extract_path_span_and_comments(
    loc: ProtoLoc,
) -> Result<(Vec<i32>, Span, Option<Comments>), Error> {
    let span = Span::new(&loc.span).map_err(|_| Error::invalid_span(&loc))?;
    let comments = Comments::new_maybe(
        loc.leading_comments,
        loc.trailing_comments,
        loc.leading_detached_comments,
    );
    Ok((loc.path, span, comments))
}

pub(super) struct Message {
    pub(super) location: Location,
    pub(super) messages: Vec<Message>,
    pub(super) enums: Vec<Enum>,
    pub(super) extensions: Vec<ExtensionGroup>,
    pub(super) oneofs: Vec<Oneof>,
    pub(super) fields: Vec<Field>,
}

fn iterate_next(prefix: &[i32], locations: &mut Iter) -> Option<ProtoLoc> {
    let Some(next) = locations.peek() else {
        return None;
    };
    if next.path[..prefix.len()] == prefix[..] {
        locations.next()
    } else {
        None
    }
}
impl Message {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let (path, span, comments) = extract_path_span_and_comments(node)?;
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut extensions = Vec::new();
        let mut oneofs = Vec::new();
        let mut fields = Vec::new();

        while let Some(next) = iterate_next(&path, locations) {
            match path::Message::from(next.path[path.len()]) {
                path::Message::Field => {
                    fields.push(Field::new(next, locations)?);
                }
                path::Message::Nested => {
                    messages.push(Message::new(next, locations)?);
                }
                path::Message::Enum => todo!(),
                path::Message::Extension => todo!(),
                path::Message::Oneof => todo!(),
                path::Message::Unknown(_) => todo!(),
            }
        }
        Ok(Self {
            location: Location { span, comments },
            messages,
            fields,
            enums,
            extensions,
            oneofs,
        })
    }
}

pub(super) struct ExtensionGroup {
    pub(super) location: Location,
    pub(super) field: Vec<Field>,
    pub(super) extension_groups: Vec<ExtensionGroup>,
}

pub(super) struct Field {
    pub(super) location: Location,
}
impl Field {
    fn new(node: ProtoLoc, locations: &mut Iter) -> Result<Self, Error> {
        let (path, span, comments) = extract_path_span_and_comments(node)?;
        Ok(Self {
            location: Location { span, comments },
        })
    }
}

pub(super) struct Oneof {
    pub(super) location: Location,
}
pub(super) struct Enum {
    pub(super) location: Location,
    pub(super) values: Vec<EnumValue>,
}
pub(super) struct EnumValue {
    pub(super) location: Location,
}
pub(super) struct Service {
    pub(super) location: Location,
    pub(super) methods: Vec<Method>,
}

pub(super) struct Method {
    pub(super) location: Location,
}

#[cfg(test)]
mod tests {
    use protobuf::{descriptor::SourceCodeInfo, SpecialFields};

    use super::*;

    fn construct_info() -> SourceCodeInfo {
        use protobuf::descriptor::source_code_info::Location;
        SourceCodeInfo {
            location: vec![
                Location {
                    path: vec![],
                    span: vec![0, 0, 9, 1],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 0],
                    span: vec![0, 0, 20],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 0, 1],
                    span: vec![0, 8, 18],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1],
                    span: vec![2, 0, 9, 1],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 1],
                    span: vec![2, 8, 21],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 8, 0],
                    span: vec![4, 2, 8, 3],
                    leading_comments: Some(" oneof comments \n".to_string()),
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 8, 0, 1],
                    span: vec![4, 8, 18],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 0],
                    span: vec![6, 4, 20],
                    leading_comments: Some(" field oneof comments \n".to_string()),
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 0, 5],
                    span: vec![6, 4, 10],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 0, 1],
                    span: vec![6, 11, 15],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 0, 3],
                    span: vec![6, 18, 19],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 1],
                    span: vec![7, 4, 31],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 1, 6],
                    span: vec![7, 4, 14],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 1, 1],
                    span: vec![7, 15, 26],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
                Location {
                    path: vec![4, 1, 2, 1, 3],
                    span: vec![7, 29, 30],
                    leading_comments: None,
                    trailing_comments: None,
                    leading_detached_comments: vec![],
                    special_fields: SpecialFields::default(),
                },
            ],
            special_fields: SpecialFields::default(),
        }
    }

    #[test]
    fn test_name() {}
}
