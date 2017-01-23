#[macro_use]
extern crate nom;

mod parser;
mod types;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter};
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

    let mut data = Vec::with_capacity(in_file.metadata()
                                     .expect("Cannot get input file length")
                                     .len() as usize);
    let mut parsed_file = {
        let f = File::open(&in_file).expect(&usage);
        let mut reader = BufReader::new(f);
        reader.read_to_end(&mut data).expect("Cannot read input file");
        FileDescriptor::from_bytes(&data).expect("Cannot parse protobuf messages")
    };


    parsed_file.break_cycles();

    let name = in_file.file_name().and_then(|e| e.to_str()).unwrap();
    let mut w = BufWriter::new(File::create(out_file).expect("Cannot create output file"));
    parsed_file.write(&mut w, name).expect("Cannot write rust module");

}
