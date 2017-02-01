# quick-protobuf

A pure Rust library to serialize/deserialize [protobuf](https://developers.google.com/protocol-buffers) files.

[Documentation](https://docs.rs/quick-protobuf)

## Description

This library intends to provide a simple yet fast (minimal allocations) protobuf parser implementation.

It provides both:
- [**pb-rs**](codegen), a code generation tool: 
  - each `.proto` file will generate a minimal rust module (one function to read, one to write, and one to compute the size of the messages)
  - each message will generate a rust struct where:
    - `bytes` fields are converted to `Cow::Borrowed([u8])`
    - `string` fields are converted to `Cow::Borrowed(str)`
    - `repeated` fields are converted to `Vec`
    - all other fields are converted into the matching rust primitive type
  - no need to use google `protoc` tool to generate the modules
- **quick-protobuf**, a protobuf file parser: 
  - this is the crate that you will typically refer to in your library. The generated modules will assume it has been imported.
  - it acts like an event parser, the logic to convert it into struct is handle by `pb-rs`

## Example: protobuf_example project

 - 1. Use `pb-rs` binary, located into `codegen` directory to automatically generate a foo_bar.rs module from a foo_bar.proto proto file

```sh
git clone https://github.com/tafia/quick-protobuf
cd quick-protobuf/codegen
cargo run ../../protobuf_example/foo_bar.proto
cd ../../protobuf_example
```

 - 2. Add a dependency to quick-protobuf

```toml
# Cargo.toml
[dependencies]
quick-protobuf = "0.1.0"
```

 - 3. Have fun

```rust
// main.rs or lib.rs
extern crate quick_protobuf;

mod foo_bar; // (see 1.)

use quick_protobuf::Reader;

// We will suppose here that Foo and Bar are two messages defined in the .proto file
// and converted into rust structs
//
// FooBar is the root message defined like this:
// message FooBar {
//     repeated Foo foos = 1;
//     repeated Bar bars = 2;
// }
use foo_bar::{FooBar};

fn main() {
    // create a reader, which will parse the protobuf binary file and pop events
    // this reader will read the entire file into an internal buffer
    let mut reader = Reader::from_file("/path/to/binary/protobuf.bin")
        .expect("Cannot read input file");
    
    // use the generated module fns with the reader to convert your data into rust structs
    let foobar = reader.read(FooBar::from_reader).expect("Cannot read FooBar message");

    println!("Found {} foos and {} bars!", foobar.foos.len(), foobar.bars.len());
}
```

## Why not [rust-protobuf](https://github.com/stepancheg/rust-protobuf)

This library is an alternative to the widely used [rust-protobuf](https://github.com/stepancheg/rust-protobuf).
If you want to build anything serious, I strongly advise against using quick-protobuf which is very immature for the moment.

### Pros / Cons

- Pros
  - No need to install anything on your machine but rust
  - No trait objects: faster/simpler parser
  - Very simple generated modules (~10x smaller)
  - Less allocations (bytes and string are converted respectively to Cow::Borrowed([u8]) and Cow::Borrowed(str))

- Cons
  - Very immature library at the moment, [many missing functionalities](https://github.com/tafia/quick-protobuf/issues/12)
  - Not a drop-in replacement of [rust-protobuf]
    - you have to handle `Option`s unwrapping yourself
    - you may need to handle `Cow` as well is you want to modify it
  - Very little tests in comparison

### Codegen

Have a look at the different generated modules for the same .proto file:
- [rust-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data.rs): 2322 loc
- [quick-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data_quick.rs): 300 loc

### Benchmarks

The only implemented benchmarks are the [adaptation from rust-protobuf perftest](benches/rust-protobuf).

## Contribution

Any help is welcomed! (Pull requests of course, bug report, missing functionality etc...)

## Licence

MIT
