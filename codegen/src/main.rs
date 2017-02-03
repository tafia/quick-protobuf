#[macro_use]
extern crate nom;
#[macro_use]
extern crate error_chain;

mod parser;
mod types;
mod errors;

use std::env;
use std::path::PathBuf;
use types::FileDescriptor;

fn main() {

    let args = env::args().collect::<Vec<_>>();
    let usage = format!("{} <file.proto>", args[0]);

    if args.len() == 0 {
        println!("{}", usage);
        return;
    }

    let in_file: PathBuf = args.get(1).map(|a| a.into()).unwrap();
    match in_file.extension().and_then(|e| e.to_str()) {
        Some("proto") => (),
        _ => {
            println!("{}", usage);
            println!("\r\nExpecting an input file with 'proto' extension");
            return;
        }
    }

    let out_file = in_file.with_extension("rs");

    FileDescriptor::write_proto(&in_file, &out_file)
        .expect(&format!("Could not convert {} into {}", 
                         in_file.display(), out_file.display()));

}
