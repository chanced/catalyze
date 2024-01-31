/// code generator requests
pub(crate) mod cgr {
    use protobuf::{plugin::CodeGeneratorRequest, Message};
    fn code_generator_request(bytes: &[u8]) -> CodeGeneratorRequest {
        CodeGeneratorRequest::parse_from_bytes(bytes).unwrap()
    }

    pub(crate) fn commented() -> CodeGeneratorRequest {
        code_generator_request(include_bytes!(
            "../../fixtures/cgr/commented/code_generator_request.bin"
        ))
    }
    pub(crate) fn extended() -> CodeGeneratorRequest {
        code_generator_request(include_bytes!(
            "../../fixtures/cgr/extended/code_generator_request.bin"
        ))
    }
}
