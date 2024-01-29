use protobuf::descriptor;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Reserved {
    pub names: Box<[String]>,
    pub ranges: Box<[ReservedRange]>,
}

impl Reserved {
    #[must_use]
    pub fn names(&self) -> &[String] {
        &self.names
    }

    #[must_use]
    pub fn ranges(&self) -> &[ReservedRange] {
        &self.ranges
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

impl From<descriptor::descriptor_proto::ReservedRange> for ReservedRange {
    fn from(range: descriptor::descriptor_proto::ReservedRange) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<descriptor::enum_descriptor_proto::EnumReservedRange> for ReservedRange {
    fn from(range: descriptor::enum_descriptor_proto::EnumReservedRange) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl ReservedRange {
    #[must_use]
    pub fn start(&self) -> i32 {
        self.start.unwrap_or(0)
    }
    #[must_use]
    pub fn end(&self) -> i32 {
        self.end.unwrap_or(0)
    }
}
