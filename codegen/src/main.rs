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

use errors::*;

fn run() -> Result<()> {
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
        .arg(Arg::with_name("OUTPUT_DIR")
                .required(false)
                .long("output_directory")
                .short("d")
                .takes_value(true)
                .help("Output directory of generated code"))
        .arg(Arg::with_name("INCLUDE_PATH")
                .required(false)
                .long("include")
                .short("I")
                .takes_value(true)
                .help("Path to search for imported protobufs"))
        .arg(Arg::with_name("SINGLE_MOD")
                .required(false)
                .long("single-mod")
                .short("s")
                .help("Omit generation of modules for each package when there is only one package"))
        .arg(Arg::with_name("INPUT")
                .multiple(true)
                .help("The .proto files used to generate quick-protobuf code")
                .validator(|x| extension_matches(x, "proto")))
        .get_matches();

    let in_files = path_vec(values_t!(matches,"INPUT",String));
    if in_files.is_empty() {
        bail!("no .proto files provided!");
    }

    for f in &in_files {
        if ! f.exists() {
            bail!(format!("input file {:?} does not exist", f));
        }
    }

    let mut include_path = path_vec(values_t!(matches,"INCLUDE_PATH",String));
    if include_path.is_empty() {
        include_path.push(".".into());
    }

    if in_files.len() > 1 && matches.value_of("OUTPUT").is_some() {
        bail!("--output only allowed for single input file");
    }

    for in_file in in_files {
        let mut out_file = in_file.with_extension("rs");

         if let Some(ofile) = matches.value_of("OUTPUT") {
            out_file = PathBuf::from(ofile);
        }

        if let Some(dir) = matches.value_of("OUTPUT_DIR") {
            let mut directory = PathBuf::from(dir);
            if ! directory.is_dir() { // we can create? But only last dir
                bail!(format!("output directory {:?} does not exist", directory));
            }
            directory.push(out_file.file_name().unwrap());
            out_file = directory;
        }

        let config = Config {
            in_file: in_file,
            out_file: out_file,
            single_module: matches.is_present("SINGLE_MOD"),
            import_search_path: include_path.clone(),
        };

        FileDescriptor::write_proto(&config)
            .chain_err(|| format!("Could not convert {} into {}",
                             config.in_file.display(), config.out_file.display()))?;

    }
    Ok(())
}

fn extension_matches<P: AsRef<Path>>(path: P, expected: &str) -> std::result::Result<(), String> {
    match path.as_ref().extension() {
        Some(x) if x == expected => Ok(()),
        Some(x) => Err(format!("Expected path with extension '{}', not: '{}'", expected, x.to_string_lossy())),
        None => Err(format!("Expected path with extension '{}'", expected)),
    }
}

fn path_vec(maybe_vec: std::result::Result<Vec<String>,clap::Error>) -> Vec<PathBuf> {
    maybe_vec.unwrap_or(Vec::new()).iter().map(|s| s.into()).collect()
}


quick_main!(run);
