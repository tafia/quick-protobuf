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
use types::{FileDescriptor, Config};

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(Arg::with_name("OUTPUT")
                .required(false)
                .long("output")
                .short("o")
                .takes_value(true)
                .help("Generated file name, defaults to INPUT with 'rs' extension")
                .validator(|x| extension_matches(x, "rs")))
        .arg(Arg::with_name("SINGLE_MOD")
                .required(false)
                .long("single-mod")
                .short("s")
                .help("Omit generation of modules for each package when there is only one package"))
        .arg(Arg::with_name("INPUT")
                .required(true)
                .index(1)
                .help("The .proto file used to generate quick-protobuf code")
                .validator(|x| extension_matches(x, "proto")))
        .get_matches();

    let in_file: PathBuf = matches.value_of("INPUT").map(|a| a.to_string().into()).unwrap();
    let out_file: PathBuf = matches.value_of("OUTPUT").map(|a| a.to_string().into()).unwrap_or(in_file.with_extension("rs"));

    let config = Config {
        in_file: in_file,
        out_file: out_file,
        single_module: matches.is_present("SINGLE_MOD")
    };

    FileDescriptor::write_proto(&config)
        .expect(&format!("Could not convert {} into {}",
                         config.in_file.display(), config.out_file.display()));

}

fn extension_matches<P: AsRef<Path>>(path: P, expected: &str) -> Result<(), String> {
    match path.as_ref().extension() {
        Some(x) if x == expected => Ok(()),
        Some(x) => Err(format!("Expected path with extension '{}', not: '{}'", expected, x.to_string_lossy())),
        None => Err(format!("Expected path with extension '{}'", expected)),
    }
}
