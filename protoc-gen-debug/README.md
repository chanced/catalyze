# protoc-gen-debug

This plugin can be used to create a bin file composed of the entire encoded
`CodeGeneratorRequest` from a protoc execution. The intended usecase is for
testing plugins without having to run protoc on each pass.

Executing the plugin will place a `code_generator_request.pb.bin` file in the
specified output location. To decode the output, use a protobuf lib, such as
[`protobuf`](https://github.com/stepancheg/rust-protobuf/).

## Install

```bash
cargo install protoc-gen-debug
```

## Usage

```bash
protoc --debug_out="./path/to/output/dir" ./path/to/protos/example.proto
```

## License

MIT or Apache 2.0
