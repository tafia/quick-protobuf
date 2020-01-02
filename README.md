# quick-protobuf

A pure Rust library to serialize/deserialize [protobuf](https://developers.google.com/protocol-buffers) files.

[Documentation](https://docs.rs/quick-protobuf)

## Description

This library intends to provide a simple yet fast (minimal allocations) protobuf parser implementation.

It provides both:
- [**pb-rs**](pb-rs), a code generation tool: 
  - each `.proto` file will generate a minimal rust module (one function to read, one to write, and one to compute the size of the messages)
  - each message will generate a rust struct where:

    | **Proto**                    | **Rust**                |
    |------------------------------|-------------------------|
    | bytes                        | `Cow<'a, [u8]>`         |
    | string                       | `Cow<'a, str>`          |
    | other scalars                | rust primitive          |
    | repeated                     | `Vec`                   |
    | repeated, packed, fixed size | `Cow<'a, [M]>`          |
    | optional                     | `Option`                |
    | message                      | `struct`                |
    | enum                         | `enum`                  |
    | map                          | `HashMap`               |
    | oneof Name                   | `OneOfName` enum        |
    | nested `m1`                  | `mod_m1` module         |
    | package `a.b`                | `mod_a::mod_b` modules  |
    | import file_a.proto          | `use super::file_a::*` |

  - no need to use google `protoc` tool to generate the modules
- [**quick-protobuf**](quick-protobuf), a protobuf file parser: 
  - this is the crate that you will typically refer to in your library. The generated modules will assume it has been imported.
  - it acts like an event parser, the logic to convert it into struct is handle by `pb-rs`

## Example: protobuf_example project

 - 1. Install **pb-rs** binary to convert your proto file into a **quick-protobuf** compatible source code

```sh
cargo install pb-rs
pb-rs /path/to/your/protobuf/file.proto
# will generate a 
# /path/to/your/protobuf/file.rs
```

 - 2. Add a dependency to quick-protobuf

```toml
# Cargo.toml
[dependencies]
quick-protobuf = "0.6.2"
```

 - 3. Have fun

```rust
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
// FooBar is a message generated from a proto file
// in parcicular it contains a `from_reader` function
use foo_bar::FooBar;
use quick_protobuf::{MessageRead, BytesReader};

fn main() {
    // bytes is a buffer on the data we want to deserialize
    // typically bytes is read from a `Read`:
    // r.read_to_end(&mut bytes).expect("cannot read bytes");
    let mut bytes: Vec<u8>;
    # bytes = vec![];

    // we can build a bytes reader directly out of the bytes
    let mut reader = BytesReader::from_bytes(&bytes);

    // now using the generated module decoding is as easy as:
    let foobar = FooBar::from_reader(&mut reader, &bytes).expect("Cannot read FooBar");

    // if instead the buffer contains a length delimited stream of message we could use:
    // while !r.is_eof() {
    //     let foobar: FooBar = r.read_message(&bytes).expect(...);
    //     ...
    // }
    println!("Found {} foos and {} bars", foobar.foos.len(), foobar.bars.len());
}
```

## Examples directory

You can find basic examples in the [examples](examples) directory.
- [pb_rs_example](quick-protobuf/examples/pb_rs_example.rs): A basic write/read loop on all datatypes

## Message <-> struct

The best way to check for all kind of generated code is to look for the codegen_example data:
- definition: [data_types.proto](quick-protobuf/examples/pb_rs/data_types.proto)
- generated code: [data_types.rs](quick-protobuf/examples/pb_rs/data_types.rs)

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

### Leverage rust module system

#### Nested Messages
```
message A {
    message B {
        // ...
    }
}
```

As rust does not allow a struct and a module to share the same name, we use `mod_Name` for the nested messages.
```rust
pub struct A {
    //...
}

pub mod mod_A {
    pub struct B {
        // ...
    }
}
```

#### Package

```
package a.b;
```

Here we could have used the same name, but for consistency with nested messages, modules are prefixed with `mod_` as well.
```rust
pub mod mod_a {
    pub mod mod_b {
        // ...
    }
}
```

## Why not rust-protobuf

This library is an alternative to the widely used [rust-protobuf](https://github.com/stepancheg/rust-protobuf).

#### Pros / Cons

- Pros
  - [Much faster](perftest), in particular when working with string, bytes and repeated packed fixed size fields (no extra allocation)
  - No need to install `protoc` on your machine
  - No trait objects: faster/simpler parser
  - Very simple generated modules (~10x smaller) so you can easily understand what is happening

- Cons
  - Less popular
    - most rust-protobuf tests have been migrated here (see [v2](quick-protobuf/tests/rust_protobuf/v2/mod.rs) and [v3](quick-protobuf/tests/rust_protobuf/v3/mod.rs))
    - quick-protobuf is being used by many people now and is very reliable
    - [some missing functionalities](https://github.com/tafia/quick-protobuf/issues/12)
  - Not a drop-in replacement of rust-protobuf
    - everything being explicit you have to handle more things yourself (e.g. `Option` unwrapping, `Cow` management)

#### Codegen

Have a look at the different generated modules for the same .proto file:
- [rust-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data.rs): 2371 loc
- [quick-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data_quick.rs): 302 loc

#### Benchmarks

See [perftest](perftest), an adaptation of rust protobuf's perftest. Depending on your scenario each crate has its merit.
quick-protobuf is particularly good at reading large bytes.

## Contribution

Any help is welcomed! (Pull requests of course, bug report, missing functionality etc...)

## Licence

MIT
