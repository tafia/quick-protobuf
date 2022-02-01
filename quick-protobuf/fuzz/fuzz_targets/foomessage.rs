#![no_main]
use libfuzzer_sys::fuzz_target;
use quick_protobuf::{BytesReader, MessageRead};

fuzz_target!(|data: &[u8]| {
    let mut r = BytesReader::from_bytes(data);
    let _ = pb_rs::data_types::FooMessage::from_reader(&mut r, data);
});

#[path = "../../examples/pb_rs/mod.rs"]
mod pb_rs;
