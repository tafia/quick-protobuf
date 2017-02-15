#[macro_use]
extern crate nom;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

mod parser;
mod types;
mod errors;
mod keywords;

use std::path::{Path, PathBuf};
use clap::{App, Arg};
use types::FileDescriptor;

fn main() {
    let matches = App::new("pb-rs").version(crate_version!())
        .arg(Arg::with_name("INPUT").required(true).index(1).help("The .proto file used to generate quick-protobuf code").validator(|x| ends_with_proto(x)))
        .get_matches();

    let in_file: PathBuf = matches.value_of("INPUT").map(|a| a.to_string().into()).unwrap();
    let out_file = in_file.with_extension("rs");

    FileDescriptor::write_proto(&in_file, &out_file)
        .expect(&format!("Could not convert {} into {}", 
                         in_file.display(), out_file.display()));

}

fn ends_with_proto<P: AsRef<Path>>(path: P) -> Result<(), String> {
    match path.as_ref().extension() {
        Some(x) if x == "proto" => Ok(()),
        Some(x) => Err(format!("Expected path with extension 'proto', not: '{}'", x.to_string_lossy())),
        None => Err(format!("Expected path with extension 'proto'")),
    }
}
