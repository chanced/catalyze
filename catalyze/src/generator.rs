#[derive(Clone, Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
    pub prerelease: Option<String>,
    pub build_metadata: Option<String>,
    pub prefix: Option<String>,
}

pub trait Input {
    type Parameter;
    fn files(&self) -> &[protobuf::descriptor::FileDescriptorProto];
    fn protoc_version(&self) -> Option<Version>;
}
