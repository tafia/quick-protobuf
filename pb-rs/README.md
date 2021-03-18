# pb-rs

A simple converter from .proto files into Rust [quick-protobuf](https://crates.io/crates/quick-protobuf) compatible modules.

## Usage

```
pb-rs <file.proto>
```

**pb-rs** can also be used as a crate (for example in your cargo build scripts):

`Cargo.toml`:


```toml
[dependencies]
quick-protobuf = "0.8.0"

[build-dependencies]
pb-rs = "0.9.1"
```

`build.rs`:

```rust
use pb_rs::{types::FileDescriptor, ConfigBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).join("protos");

    let in_dir = PathBuf::from(::std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("protos");
    // Re-run this build.rs if the protos dir changes (i.e. a new file is added)
    println!("cargo:rerun-if-changed={}", in_dir.to_str().unwrap());

    // Find all *.proto files in the `in_dir` and add them to the list of files
    let mut protos = Vec::new();
    let proto_ext = Some(Path::new("proto").as_os_str());
    for entry in WalkDir::new(&in_dir) {
        let path = entry.unwrap().into_path();
        if path.extension() == proto_ext {
            // Re-run this build.rs if any of the files in the protos dir change
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            protos.push(path);
        }
    }

    // Delete all old generated files before re-generating new ones
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    std::fs::DirBuilder::new().create(&out_dir).unwrap();
    let config_builder = ConfigBuilder::new(&protos, None, Some(&out_dir), &[in_dir]).unwrap();
    FileDescriptor::run(&config_builder.build()).unwrap()
}
```

`main.rs` or `lib.rs`:

```rust
mod hello {
    include_bytes!(concat!(env!("OUT_DIR")), "/hello.rs");
}
```
