# Catalyze

Catalyze is a library that simplifies the creation of `protoc` plugins
for code generators based on [Protocol Buffers](https://protobuf.dev/).

## Table of Contents

-   [Roadmap](#roadmap)
-   [Refresher of `protoc`](#refresh-of-protoc)
-   [Overview](#overview)

## Roadmap

-   [x] Fully materialized Abstract Syntax Tree (AST)
-   [x] Complete encapsulation of the [`protobuf`](https://github.com/stepancheg/rust-protobuf/) crate
-   [ ] Context support for the following crates:
    -   [ ] [`tokio-rs/prost`](https://github.com/tokio-rs/prost)
    -   [ ] [`hyperium/tonic`](https://github.com/hyperium/tonic)
    -   [ ] [`stepancheg/rust-protobuf`](https://github.com/stepancheg/rust-protobuf/)
    -   [ ] [`stepancheg/grpc-rust`](https://crates.io/crates/grpc)
-   [ ] Parallel execution of generators
-   [ ] Encapsulation of
-   [ ] `catalyze-cli`
    -   [ ] Execution of WASM/WASI plugins

## Refresh of `protoc`

`protoc` can be a bit confusing at first so this section aims to provide a brief
explanation on how to interface with the protobuf compiler.

`protoc` is a command line tool that takes a set of `.proto` files, constructs a
[`CodeGeneratorRequest`](https://github.com/protocolbuffers/protobuf/blob/1d6ac5979b909a222db45cb154f0be3a31828324/src/google/protobuf/compiler/plugin.proto#L42-L80)
composed of target proto files as well as any imported proto files the targets depend
upon.

In the example below, `./proto/**/*.proto` indicates which files are targets
and `./path/to/protos` is the path which `protoc` would conduct a search of imported
proto files of the targets.

```sh
protoc -I ./path/to/protos
  --my_generator_out="./desired/output/path"  \
  --my_generator_opt="some_option=some_value" \
  ./proto/**/*.proto
```

The `CodeGeneratorRequest` is serialized and passed to each specified plugin via
`stdin`. Those plugins are then expected to deserialize the request, generate a
[`CodeGeneratorResponse`](https://github.com/protocolbuffers/protobuf/blob/1d6ac5979b909a222db45cb154f0be3a31828324/src/google/protobuf/compiler/plugin.proto#L82-L180)
containing all artifacts that are to be minted, and finally relay that to
`protoc` via `stdout`.

`protoc` determines which plugins to execute based upon a convetion of
`<plugin_name>_out`, the (required) desired output directory for the plugin, and
`<plugin_name>_opt`, the (optional) free-form string that can be used as
parameters to your plugin. `protoc` then searches your `PATH` for a
binary named `protoc-gen-<plugin_name>`. Using the example above, `protoc` would
search `PATH` for a binary named `protoc-gen-my_generator`.

In addition to auto-resolution of plugins by name to binaries in your `PATH`,
`protoc` has two additional procedures for resolving plugins. The first is for
standard plugins that are shipped with `protoc` itself. These std plugins are
resolved by name internally and thus do not to need discoverable in your `PATH`.
The second is to specify a path via the `--plugin` flag. The expected format is
`--plugin=protoc-gen-<plugin_name>=<path_to_plugin>`. For example, the following
would instruct `protoc` to use the binary at `/path/to/protoc-gen-my_generator`
for the `my_generator` plugin.

```sh
protoc -I ./path/to/protos
  --plugin=protoc-gen-my_generator=/path/to/protoc-gen-my_generator
  --my_generator_out="./desired/output/path"  \
  --my_generator_opt="some_option=some_value" \
  ./proto/**/*.proto
```

### Overview

Catalyze aims to both simplify the process of creating `protoc` plugins through the usage of

<p align="center">
	<img alt="simple graph diagram depicting type relations" src="https://github.com/chanced/catalyze/blob/initial-version/media/graph.svg?raw=true">
</p>
