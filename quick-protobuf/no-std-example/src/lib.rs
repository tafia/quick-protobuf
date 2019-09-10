#![no_std]

use quick_protobuf::{serialize_into_slice, deserialize_from_slice};
use crate::protos::no_std::*;

mod protos;

pub fn round_trip() {
    let message = NoStdMessage::default();

    let mut buf = [0u8; 1024];
    serialize_into_slice(&message, &mut buf).unwrap();

    let read_message = deserialize_from_slice(&buf).unwrap();
    assert_eq!(message, read_message);
}

