# pb-rs

A simple converter from .proto files into rust quick-protobuf compatible modules.

## Usage

```
pb-rs <file.proto>
```

pb-rs can also be used as a crate (for example in your cargo build scripts).

```rust
extern crate pb_rs;

use std::path::PathBuf;
use pb_rs::types::{Config, FileDescriptor};

fn main() {
    let config = Config {
        in_file: PathBuf::from("protos/Hello.proto"),
        out_file: PathBuf::from("src/hello.rs"),
        single_module: false,
        import_search_path: vec![PathBuf::from("protos")],
        no_output: false,
    };

    FileDescriptor::write_proto(&config).unwrap();
}
```
