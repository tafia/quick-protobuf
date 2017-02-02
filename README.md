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
quick-protobuf = "0.2.0"
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

## Examples directory

You can find basic examples in the [examples](examples) directory.
- [codegen_example](examples/codegen_example.rs): A basic write/read loop on all datatypes

## Message <-> struct

#### Proto definition

```
enum FooEnum {
    FIRST_VALUE = 1;
    SECOND_VALUE = 2;
}
    
message BarMessage {
    required int32 b_required_int32 = 1;
}

message FooMessage {
    optional int32 f_int32 = 1;
    optional int64 f_int64 = 2;
    optional uint32 f_uint32 = 3;
    optional uint64 f_uint64 = 4;
    optional sint32 f_sint32 = 5;
    optional sint64 f_sint64 = 6;
    optional bool f_bool = 7;
    optional FooEnum f_FooEnum = 8;
    optional fixed64 f_fixed64 = 9;
    optional sfixed64 f_sfixed64 = 10;
    optional fixed32 f_fixed32 = 11;
    optional sfixed32 f_sfixed32 = 12;
    optional double f_double = 13;
    optional float f_float = 14;
    optional bytes f_bytes = 15;
    optional string f_string = 16;
    optional FooMessage f_self_message = 17;
    optional BarMessage f_bar_message = 18;
    repeated int32 f_repeated_int32 = 19;
    repeated int32 f_repeated_packed_int32 = 20 [ packed = true ];
}
```

#### Generated structs
```rust
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FooEnum {
    FIRST_VALUE = 1,
    SECOND_VALUE = 2,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BarMessage {                                 // all fields are owned: no lifetime parameter
    pub b_required_int32: i32,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FooMessage<'a> {                             // has borrowed fields: lifetime parameter
    pub f_int32: Option<i32>,
    pub f_int64: Option<i64>,
    pub f_uint32: Option<u32>,
    pub f_uint64: Option<u64>,
    pub f_sint32: Option<i32>,
    pub f_sint64: Option<i64>,
    pub f_bool: Option<bool>,
    pub f_FooEnum: Option<FooEnum>,
    pub f_fixed64: Option<u64>,
    pub f_sfixed64: Option<i64>,
    pub f_fixed32: Option<u32>,
    pub f_sfixed32: Option<i32>,
    pub f_double: Option<f64>,
    pub f_float: Option<f32>,
    pub f_bytes: Option<Cow<'a, [u8]>>,                 // bytes  -> Cow<[u8]>
    pub f_string: Option<Cow<'a, str>>                  // string -> Cow<str>
    pub f_self_message: Option<Box<FooMessage<'a>>>,    // reference cycle -> Boxed message
    pub f_bar_message: Option<BarMessage>,
    pub f_repeated_int32: Vec<i32>,                     // repeated: Vec
    pub f_repeated_packed_int32: Vec<i32>,              // repeated packed: Vec
}
```

## Why not rust-protobuf

This library is an alternative to the widely used [rust-protobuf](https://github.com/stepancheg/rust-protobuf).

#### Pros / Cons

- Pros
  - No need to install anything on your machine but rust
  - No trait objects: faster/simpler parser
  - Very simple generated modules (~10x smaller)
  - Less allocations (bytes and string are converted respectively to `Cow<[u8]>` and `Cow<str>`)

- Cons
  - Very immature library at the moment, [many missing functionalities](https://github.com/tafia/quick-protobuf/issues/12)
  - Not a drop-in replacement of rust-protobuf
    - you have to handle `Option`s unwrapping yourself
    - you may need to handle `Cow` as well if you want to modify it
  - Very little tests in comparison

#### Codegen

Have a look at the different generated modules for the same .proto file:
- [rust-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data.rs): 2322 loc
- [quick-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data_quick.rs): 300 loc

#### Benchmarks

The only implemented benchmarks are the [adaptation from rust-protobuf perftest](benches/rust-protobuf).

They show that quick-protobuf is, at the time of writing and on these specific benches much faster.

## Contribution

Any help is welcomed! (Pull requests of course, bug report, missing functionality etc...)

## Licence

MIT
