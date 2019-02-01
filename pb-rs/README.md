# pb-rs

A simple converter from .proto files into rust quick-protobuf compatible modules.

## Usage

```
pb-rs <file.proto>
```

pb-rs can also be used as a crate (for example in your cargo build scripts).

```rust
// Cargo.toml

[dependencies]
quick-protobuf = "0.6.3"

[build-dependencies]
pb-rs = "0.6.0"

// build.rs

extern crate pb_rs;

use std::path::{Path, PathBuf};
use pb_rs::types::{Config, FileDescriptor};
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(out_dir).join("hello.rs");

    let config = Config {
        in_file: PathBuf::from("protos/Hello.proto"),
        out_file,
        single_module: false,
        import_search_path: vec![PathBuf::from("protos")],
        no_output: false,
        error_cycle: false, // may change a required field to an optional
        headers: false, // do not generate headers
        custom_struct_derive: vec![] // Nothing
    };

    FileDescriptor::write_proto(&config).unwrap();
}

// main.rs or lib.rs

mod hello {
    include_bytes!(concat!(env!("OUT_DIR")), "/hello.rs");
}
```
