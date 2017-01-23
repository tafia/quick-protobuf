# quick-protobuf

A pure Rust library to deserialize protobuf files.

Simple codegen. No need of protoc. Fast.

# Getting started

1. Convert your .proto files into rust modules.

    ```sh
git clone https://github.com/tafia/quick-protobuf
cd quick-protobuf/protorust_codegen

# generate a /my/proto/path/my_file.rs module to import into your project
cargo run /my/proto/path/my_file.proto
    ```

2. Import quick-protobuf into you crate

    ```toml
# Cargo.toml
[dependencies]
quick-protobuf = { git = "https://github.com/tafia/quick-protobuf" }
    ```

3. Import the generated module

    ```rust
// main.rs or lib.rs
extern crate quick_protobuf;

mod my_file; // generated with protorust_codegen

use quick_protobuf::{Message, Result, Reader};
use my_file::Foo;

fn main() {
    run("/path/to/my/binary/file.bin").unwrap();
}

fn run(p: &str) -> Result<()> {
    // Foo implements Message trait, thus we can directly deserialize .bin files
    let msg = Foo::from_file(p)?;
    println!("Deserialized msg: {:#?}", msg);
}
    ```

# Objectives

This library is an alternative to the widely used [rust-protobuf](https://github.com/stepancheg/rust-protobuf).

- Pros
  - No need to install anything on your machine but rust
  - No trait objects: faster/simpler parser
  - Dead simple generated modules: 
    - a struct with public fields
    - an implementation of Message, which means just one match loop for reader (writer not implemented yet)
    - more than 10x smaller modules in practice
    - modifying the generated code (e.g. use `HashMap`s instead of `Vec`s), while not necessarily advised is totally fine as the code is easy to reason about
  - Easier on memory (no trait objects, no storage of unknown fields)

- Cons
  - Very immature library at the moment: many missing functionalities, very poor test coverage, very poor documentation
  - Not a drop-in replacement of [rust-protobuf], in particular, you have to handle `Option`s unwrapping yourself
  - probably many other corner cases I overlooked

