#![no_std]

extern crate quick_protobuf;

mod pb_rs_nostd;

use crate::pb_rs_nostd::protos::no_std::NoStdMessage;
use quick_protobuf::{serialize_into_slice, deserialize_from_slice};

fn main() {
    let message = NoStdMessage::default();

    let mut buf = [0u8; 1024];
    serialize_into_slice(&message, &mut buf).unwrap();

    let read_message = deserialize_from_slice(&buf).unwrap();
    assert_eq!(message, read_message);
}

