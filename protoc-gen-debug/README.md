# protoc-gen-debug

This plugin can be used to create a bin file composed of the entire encoded
`CodeGeneratorRequest` from a protoc execution. The intended usecase is for
testing plugins without having to run protoc on each pass.

Executing the plugin will generate `code_generator_request.bin`, containing
the binary representation of the request (same as a plugin would receive) during
normal execution and `code_generator_request.txt`, the deserialized output as
represented by the [`protobuf`](https://github.com/stepancheg/rust-protobuf).

The output path is determined by the `--debug_opt` flag, as the contents of the
binary file (`code_generator_request.bin`) cannot be represented as a utf-8
string. The `debug_out` field is still required for `protoc` to execute but the
path will not be used.

## Install

```bash
cargo install protoc-gen-debug
```

## Usage

```bash
protoc --debug_out=. --debug_opt=./path/to/output/dir --proto_path=./path/to/protos ./path/to/protos/example.proto
```

## License

MIT or Apache 2.0
