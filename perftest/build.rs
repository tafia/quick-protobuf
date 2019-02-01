extern crate pb_rs;
extern crate prost_build;
extern crate protobuf_codegen_pure;

use pb_rs::types::{Config, FileDescriptor};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // protobuf
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["src/perftest_data.proto"],
        includes: &["src"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");

    // quick-protobuf
    let quick_dest = Path::new(&out_dir).join("perftest_data_quick.rs");
    let config = Config {
        in_file: PathBuf::from("src/perftest_data.proto"),
        out_file: quick_dest,
        single_module: true,
        import_search_path: vec![PathBuf::from("src")],
        no_output: false,
        error_cycle: false,
        headers: false,
        custom_struct_derive: vec![]
    };
    FileDescriptor::write_proto(&config).unwrap();

    // prost
    let old = ::std::env::var("OUT_DIR");
    env::set_var("OUT_DIR", ".");
    prost_build::compile_protos(&["src/perftest_data.proto"], &["src"]).unwrap();
    let _ = old.map(|val| env::set_var("OUT_DIR", &val));
    fs::rename("perftest_data.rs", "src/perftest_data_prost.rs").unwrap();
}
