# quick-protobuf

A pure Rust library to deserialize protobuf files.

Simple codegen. No need of protoc. Fast.

[Documentation](https://docs.rs/quick-protobuf)

# Getting started

1. Convert your .proto files into rust modules using included pb-rs crate

    ```sh
    git clone https://github.com/tafia/quick-protobuf
    cd quick-protobuf/codegen

    # generate a /my/proto/path/my_file.rs module to import into your project
    cargo run /my/proto/path/my_file.proto
    ```

2. Import quick-protobuf into you crate

    ```toml
    # Cargo.toml
    [dependencies]
    quick-protobuf = "0.1.0"
    ```

3. Use the generated module

    ```rust
    // main.rs or lib.rs
    extern crate quick_protobuf;

    mod my_file; // generated with protorust_codegen

    use std::io::Write;
    use quick_protobuf::{MessageRead, MessageWrite, Writer, Result};
    use my_file::Foo;

    fn main() {
        run("/path/to/my/binary/file.bin").unwrap();
    }

    fn run(p: &str) -> Result<()> {
        // Foo implements Message trait, thus we can directly deserialize .bin files
        let mut msg = Foo::from_file(p)?;
        println!("Deserialized msg: {:#?}", msg);

        // Make some chanegs on msg
        msg.repeated_field_1.clear();

        // Write down updated message using any `Write` ...
        let mut buf = Vec::with_capacity(msg.get_size());
        {
            let mut writer = Writer::new(&mut buf);
            writer.write_message(&msg)?;
        }
        
        Ok(())
    }
    ```

# Why not [rust-protobuf](https://github.com/stepancheg/rust-protobuf)

This library is an alternative to the widely used [rust-protobuf](https://github.com/stepancheg/rust-protobuf).
If you want to build anything serious, I strongly advise against using quick-protobuf which is very immature for the moment.


## Pros / Cons

- Pros
  - No need to install anything on your machine but rust
  - No trait objects: faster/simpler parser
  - Dead simple generated modules: 
    - a struct with public fields
    - an implementation of Message(Read/Write), which means just one match loop for reader
    - close to 10x smaller modules in practice
    - modifying the generated code if needed
      while not necessarily advised is totally fine as the code is easy to reason about
  - Easier on memory (no trait objects, no storage of unknown fields)

- Cons
  - Very immature library at the moment: [many missing functionalities](https://github.com/tafia/quick-protobuf/issues/12), very poor test coverage, very poor documentation
  - Not a drop-in replacement of [rust-protobuf], in particular, you have to handle `Option`s unwrapping yourself
  - probably many other corner cases I overlooked

## Codegen

Have a look at the different generated modules for the same .proto file:
- [rust-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data.rs): 2322 loc
- [quick-protobuf](https://github.com/tafia/quick-protobuf/blob/master/benches/rust-protobuf/perftest_data_quick.rs): 300 loc

## Benchmarks

The only implemented benchmarks are the [adaptation from rust-protobuf perftest](benches/rust-protobuf).

